use git2::Commit;

use crate::git::error::Git2Error;
use crate::git::oid::OidOf;
use crate::git::repository::Repository;
use crate::git::rev::filters::PackagePathFilter;
use crate::git::rev::CommitIter;
use crate::SETTINGS;

impl Repository {
    /// Return a commit range for the given package from a [`RevspecPattern2`]
    pub fn get_commit_range_for_package(
        &self,
        pattern: &str,
        package: &str,
    ) -> Result<CommitIter, Git2Error> {
        let mut commit_range = self.revwalk(pattern)?;
        let mut commits = vec![];
        let package = SETTINGS.packages.get(package).expect("package exists");
        let package_path_filter = PackagePathFilter::from_package(package);

        for (oid_of, commit) in commit_range.into_iter() {
            let parent = commit.parent(0).ok().map(|commit| commit.id().to_string());

            let parent_tree = self.tree_to_treeish(parent.as_ref())?;

            let current_tree = self
                .tree_to_treeish(Some(&commit.id().to_string()))?
                .expect("Failed to get commit tree");

            let diff = match parent_tree {
                None => self
                    .0
                    .diff_tree_to_tree(None, current_tree.as_tree(), None)?,
                Some(parent_tree) => {
                    self.0
                        .diff_tree_to_tree(parent_tree.as_tree(), current_tree.as_tree(), None)?
                }
            };

            for delta in diff.deltas() {
                if let Some(old) = delta.old_file().path() {
                    if package_path_filter.is_match(old) {
                        commits.push((oid_of, commit));
                        break;
                    }
                }

                if let Some(new) = delta.new_file().path() {
                    if package_path_filter.is_match(new) {
                        commits.push((oid_of, commit));
                        break;
                    }
                }
            }
        }

        commit_range = CommitIter(commits);
        Ok(commit_range)
    }

    pub fn get_commit_range_for_monorepo_global(
        &self,
        pattern: &str,
    ) -> Result<CommitIter, Git2Error> {
        let mut commit_range = self.revwalk(pattern)?;
        let mut commits = vec![];
        let package_paths: Vec<_> = SETTINGS
            .packages
            .values()
            .map(|package| &package.path)
            .collect();

        for (oid_of, commit) in commit_range.into_iter().rev() {
            let parent = commit.parent(0);

            // First commit is always included in monorepo global tag
            if parent.is_err() {
                commits.push((oid_of, commit));
                continue;
            }

            let parent = parent?.id().to_string();
            let t1 = self
                .tree_to_treeish(Some(&parent))?
                .expect("Failed to get parent tree");

            let t2 = self
                .tree_to_treeish(Some(&commit.id().to_string()))?
                .expect("Failed to get commit tree");

            let diff = self.0.diff_tree_to_tree(t1.as_tree(), t2.as_tree(), None)?;

            for delta in diff.deltas() {
                if let Some(old) = delta.old_file().path() {
                    if package_paths.iter().all(|path| !old.starts_with(path)) {
                        commits.push((oid_of, commit));
                        break;
                    }
                }

                if let Some(new) = delta.new_file().path() {
                    if package_paths.iter().all(|path| !new.starts_with(path)) {
                        commits.push((oid_of, commit));
                        break;
                    }
                }
            }
        }

        commit_range = CommitIter(commits);
        Ok(commit_range)
    }

    /// Return a commit range from a [`RevspecPattern2`]
    pub fn revwalk(&self, spec: &str) -> Result<CommitIter, Git2Error> {
        let spec = self.revspec_from_str(spec)?;
        let mut revwalk = self.0.revwalk()?;
        revwalk.push_range(&spec.to_string())?;

        let mut commits: Vec<(OidOf, Commit)> = vec![];

        for oid in revwalk {
            let oid = oid?;
            // TODO: can we avoid allocating strings here ?
            let oid_of = self.resolve_oid_of(&oid.to_string())?;
            let commit = self.0.find_commit(oid)?;
            commits.push((oid_of, commit));
        }

        // TODO: can we avoid allocating strings here ?
        let first_oid = self.resolve_oid_of(&spec.from().to_string())?;
        let include_start = match &first_oid {
            OidOf::Head(_) | OidOf::FirstCommit(_) => true,
            OidOf::Tag(_) | OidOf::Other(_) => false,
        };

        if include_start {
            let first_commit = self.0.find_commit(*spec.from())?;
            commits.push((first_oid, first_commit));
        }

        Ok(CommitIter(commits))
    }
}

#[cfg(test)]
mod test {
    use std::collections::HashMap;
    use std::path::PathBuf;

    use anyhow::Result;

    use cmd_lib::run_cmd;
    use git2::Oid;
    use sealed_test::prelude::*;
    use speculoos::prelude::*;

    use crate::conventional::changelog::release::Release;
    use crate::git::oid::OidOf;
    use crate::git::repository::Repository;
    use crate::git::tag::{Tag, TagLookUpOptions};
    use crate::settings::{MonoRepoPackage, Settings};
    use crate::test_helpers::{commit, git_init_no_gpg, git_tag};

    const COCOGITTO_REPOSITORY: &str = env!("CARGO_MANIFEST_DIR");

    #[test]
    fn all_commits() -> Result<()> {
        // Arrange
        let repo = Repository::open(COCOGITTO_REPOSITORY)?;

        // Act
        let range = repo.revwalk("..")?;

        // Assert
        assert_that!(range.0).is_not_empty();
        Ok(())
    }

    #[sealed_test]
    fn shoud_get_range_for_a_single_release() -> Result<()> {
        // Arrange
        let repo = git_init_no_gpg()?;
        let one = commit("chore: first commit")?;
        let two = commit("feat: feature 1")?;
        let three = commit("feat: feature 2")?;
        git_tag("0.1.0")?;

        let range = repo.revwalk("0.1.0");

        let range = range?;

        // Act
        let release = Release::try_from(range)?;

        // Assert
        assert_that!(release.previous).is_none();
        assert_that!(release.version.oid()).is_equal_to(&Oid::from_str(&three)?);
        assert_that!(release.from).is_equal_to(OidOf::FirstCommit(Oid::from_str(&one)?));

        let expected_commits: Vec<String> = release
            .commits
            .into_iter()
            .map(|commit| commit.commit.oid)
            .collect();

        assert_that!(expected_commits).is_equal_to(vec![three, two, one]);

        Ok(())
    }

    #[sealed_test]
    fn shoud_get_range_for_a_multiple_release() -> Result<()> {
        // Arrange
        let repo = git_init_no_gpg()?;
        let one = commit("chore: first commit")?;
        let two = commit("feat: feature 1")?;
        let three = commit("feat: feature 2")?;
        git_tag("0.1.0")?;
        let four = commit("feat: feature 3")?;
        let five = commit("feat: feature 4")?;
        git_tag("0.2.0")?;

        let range = repo.revwalk("..0.2.0")?;

        // Act
        let release = Release::try_from(range)?;

        // Assert
        assert_that!(release.previous).is_some().matches(|_child| {
            let commits: Vec<String> = release
                .previous
                .as_ref()
                .unwrap()
                .commits
                .iter()
                .map(|commit| commit.commit.oid.clone())
                .collect();

            commits == [three.clone(), two.clone(), one.clone()]
        });

        assert_that!(release.version.to_string()).is_equal_to("0.2.0".to_string());
        assert_that!(release.from.to_string()).is_equal_to("0.1.0".to_string());

        let expected_commits: Vec<String> = release
            .commits
            .into_iter()
            .map(|commit| commit.commit.oid)
            .collect();

        assert_that!(expected_commits).is_equal_to(vec![five, four]);

        Ok(())
    }

    #[test]
    fn get_release_range_integration_test() -> Result<()> {
        // Arrange
        let repo = Repository::open(COCOGITTO_REPOSITORY)?;
        let range = repo.revwalk("0.32.1..0.32.3")?;

        // Act
        let release = Release::try_from(range)?;

        // Assert
        assert_that!(release.version.to_string()).is_equal_to("0.32.3".to_string());

        let release = *release.previous.unwrap();
        assert_that!(release.version.to_string()).is_equal_to("0.32.2".to_string());

        assert_that!(release.previous).is_none();
        Ok(())
    }

    #[sealed_test]
    fn get_annotated_tag_commits() -> Result<()> {
        // Arrange
        let repo = git_init_no_gpg()?;
        let first = commit("chore: init")?;
        let _second = commit("chore: 1.0.0")?;

        // Create an annotated tag
        let head = repo.get_head_commit()?;
        let sig = git2::Signature::now("Author", "email@example.com")?;
        repo.0
            .tag("1.0.0", &head.into_object(), &sig, "the_msg", false)?;

        commit("feat: a commit")?;

        // Act
        let commit_range = repo.revwalk("..1.0.0")?;

        // Assert
        assert_that!(commit_range.from_oid())
            .is_equal_to(OidOf::FirstCommit(Oid::from_str(&first)?));
        assert_that!(commit_range.to_oid().to_string()).is_equal_to("1.0.0".to_string());
        assert_that!(commit_range.0).has_length(2);
        Ok(())
    }

    #[sealed_test]
    fn get_package_commit_range() -> Result<()> {
        // Arrange
        let repo = git_init_no_gpg()?;
        let mut packages = HashMap::new();
        packages.insert(
            "one".to_string(),
            MonoRepoPackage {
                path: PathBuf::from("one"),
                ..Default::default()
            },
        );

        let settings = Settings {
            packages,
            ..Default::default()
        };

        let settings = toml::to_string(&settings)?;

        run_cmd!(
            echo $settings > cog.toml;
            git add .;
            git commit -m "chore: First commit";
            mkdir one;
            echo changes > one/file;
            git add .;
            git commit -m "feat: package one";
            echo changes > global;
            git add .;
            git commit -m "feat: global change";
        )?;

        let commit_range_package = repo.get_commit_range_for_package("..HEAD", "one")?;
        let commit_range_global = repo.get_commit_range_for_monorepo_global("..HEAD");
        let commit_range_global = commit_range_global?;
        assert_that!(commit_range_package.0).has_length(1);
        assert_that!(commit_range_global.0).has_length(2);
        Ok(())
    }

    #[sealed_test]
    fn package_with_changes_have_some_commits() -> Result<()> {
        let repo = init_mono_repo_for_range_filtering()?;

        run_cmd!(
            echo changes > one/file;
            git add .;
            git commit -m "feat: commit to non-ignored path";
        )?;

        let commit_range_package = repo.get_commit_range_for_package("HEAD~1..HEAD", "one")?;

        asserting!("package with changes have some commits")
            .that(&commit_range_package.0)
            .mapped_contains(
                |(_, commit)| commit.message(),
                &Some("feat: commit to non-ignored path\n"),
            );

        Ok(())
    }

    #[sealed_test]
    fn package_with_no_changes_should_have_no_commit() -> Result<()> {
        let repo = init_mono_repo_for_range_filtering()?;

        run_cmd!(
            echo changes > one/ignored/file;
            git add .;
            git commit -m "feat: commit to ignored path";
        )?;

        let commit_range_package = repo.get_commit_range_for_package("HEAD~1..HEAD", "one")?;

        asserting!("package with no changes should have some commits for HEAD~1 revspec")
            .that(&commit_range_package.0)
            .has_length(0);

        Ok(())
    }

    #[sealed_test]
    fn package_with_shared_changes_should_have_some_commits() -> Result<()> {
        let repo = init_mono_repo_for_range_filtering()?;

        run_cmd!(
            echo changes > shared/file;
            git add .;
            git commit -m "feat: commit to extra included path";
        )?;

        let commit_range_package = repo.get_commit_range_for_package("HEAD~1..HEAD", "one")?;

        asserting!("package with shared changes should have some commits for HEAD~1 revspec")
            .that(&commit_range_package.0)
            .mapped_contains(
                |(_, commit)| commit.message(),
                &Some("feat: commit to extra included path\n"),
            );

        assert_that!(commit_range_package.0).has_length(1);

        Ok(())
    }

    #[sealed_test]
    fn package_with_extra_included_changes_should_have_some_commits() -> Result<()> {
        let repo = init_mono_repo_for_range_filtering()?;

        run_cmd!(
            echo changes > README.md;
            git add .;
            git commit -m "feat: commit to extra included file";
        )?;

        let commit_range_package = repo.get_commit_range_for_package("HEAD~1..HEAD", "one")?;

        asserting!(
            "package with extra included changes should have some commits for HEAD~1 revspec"
        )
        .that(&commit_range_package.0)
        .mapped_contains(
            |(_, commit)| commit.message(),
            &Some("feat: commit to extra included file\n"),
        );

        assert_that!(commit_range_package.0).has_length(1);

        Ok(())
    }

    #[sealed_test]
    fn package_with_with_only_ignored_path_should_have_no_commit() -> Result<()> {
        let repo = init_mono_repo_for_range_filtering()?;
        let commit_range_package = repo.get_commit_range_for_package("..HEAD", "one")?;
        asserting!("package with with only ignored path commit should have no commits for full range revspec")
            .that(&commit_range_package.0).has_length(0);

        Ok(())
    }

    fn init_mono_repo_for_range_filtering() -> Result<Repository> {
        let repo = git_init_no_gpg()?;
        let mut packages = HashMap::new();
        packages.insert(
            "one".to_string(),
            MonoRepoPackage {
                path: PathBuf::from("one"),
                include: vec![String::from("shared/**"), String::from("README.md")],
                ignore: vec![String::from("one/ignored/**")],
                ..Default::default()
            },
        );

        let settings = Settings {
            packages,
            ..Default::default()
        };

        let settings = toml::to_string(&settings)?;

        run_cmd!(
            echo $settings > cog.toml;
            mkdir -p shared one/ignored;
            git add .;
            git commit -m "chore: First commit";
        )?;
        Ok(repo)
    }

    #[sealed_test]
    fn get_tag_commits() -> Result<()> {
        // Arrange
        let repo = git_init_no_gpg()?;

        let first = commit("chore: init")?;
        commit("chore: 1.0.0")?;
        git_tag("1.0.0")?;
        commit("feat: a commit")?;

        // Act
        let commit_range = repo.revwalk("..1.0.0")?;

        // Assert
        assert_that!(commit_range.from_oid().to_string()).is_equal_to(first);
        assert_that!(commit_range.to_oid().to_string()).is_equal_to("1.0.0".to_string());
        assert_that!(commit_range.0).has_length(2);
        Ok(())
    }

    #[test]
    fn from_tag_to_tag_ok() -> Result<()> {
        // Arrange
        let repo = Repository::open(COCOGITTO_REPOSITORY)?;
        let v1_0_0 = Oid::from_str("549070fa99986b059cbaa9457b6b6f065bbec46b")?;
        let _v1_0_0 = OidOf::Tag(Tag::from_str("1.0.0", Some(v1_0_0), None)?);
        let v3_0_0 = Oid::from_str("c6508e243e2816e2d2f58828ee0c6721502958dd")?;
        let v3_0_0 = OidOf::Tag(Tag::from_str("3.0.0", Some(v3_0_0), None)?);

        // Act
        let range = repo.revwalk("1.0.0..3.0.0")?;

        // Assert
        assert_that!(range.from_oid().to_string())
            .is_equal_to("7c4a1cb692b445f36a44857ce17db32b91acd24c".to_string());
        assert_that!(range.to_oid()).is_equal_to(v3_0_0);

        Ok(())
    }

    #[test]
    fn from_tag_to_head() -> Result<()> {
        // Arrange
        let repo = Repository::open(COCOGITTO_REPOSITORY)?;
        let head = repo.get_head_commit_oid()?;
        let head = OidOf::Head(head);
        let tag = repo.get_latest_tag(TagLookUpOptions::default())?;

        // Cover the case when we release a version and run the test in the CI right after that
        let head = if tag.oid() == Some(head.oid()) {
            OidOf::Tag(tag)
        } else {
            head
        };

        let v1_0_0 = Oid::from_str("549070fa99986b059cbaa9457b6b6f065bbec46b")?;
        let _v1_0_0 = OidOf::Tag(Tag::from_str("1.0.0", Some(v1_0_0), None)?);

        // Act
        let range = repo.revwalk("1.0.0..")?;

        // Assert
        assert_that!(range.from_oid().oid())
            .is_equal_to(&Oid::from_str("7c4a1cb692b445f36a44857ce17db32b91acd24c")?);
        assert_that!(range.to_oid()).is_equal_to(head);

        Ok(())
    }

    #[test]
    fn from_previous_to_tag() -> Result<()> {
        // Arrange
        let repo = Repository::open(COCOGITTO_REPOSITORY)?;
        let v2_1_1 = Oid::from_str("9dcf728d2eef6b5986633dd52ecbe9e416234898")?;
        let _v2_1_1 = OidOf::Tag(Tag::from_str("2.1.1", Some(v2_1_1), None)?);
        let v3_0_0 = Oid::from_str("c6508e243e2816e2d2f58828ee0c6721502958dd")?;
        let v3_0_0 = OidOf::Tag(Tag::from_str("3.0.0", Some(v3_0_0), None)?);

        // Act
        let range = repo.revwalk("2.1.1..3.0.0")?;

        // Assert
        assert_that!(range.from_oid().oid())
            .is_equal_to(&Oid::from_str("434c22295390fda0f276e3a3ee32fa4658489c5d")?);
        assert_that!(range.to_oid()).is_equal_to(v3_0_0);

        Ok(())
    }

    #[test]
    fn recursive_from_origin_to_head() -> Result<()> {
        // Arrange
        let repo = Repository::open(COCOGITTO_REPOSITORY)?;
        let mut tag_count = repo.0.tag_names(None)?.len();
        let head = repo.get_head_commit_oid()?;
        let latest = repo.get_latest_tag(TagLookUpOptions::default())?;
        let latest = latest.oid();
        if latest == Some(&head) {
            tag_count -= 1;
        };

        let range = repo.revwalk("..")?;

        // Act
        let mut release = Release::try_from(range)?;
        let mut count = 0;

        while let Some(previous) = release.previous {
            release = *previous;
            count += 1;
        }

        // Assert
        assert_that!(count).is_equal_to(tag_count);

        Ok(())
    }

    #[sealed_test]
    fn from_commit_to_head() -> Result<()> {
        // Arrange
        let repo = git_init_no_gpg()?;

        commit("chore: init")?;
        commit("feat: a commit")?;
        let one = commit("chore: another commit")?;
        let two = commit("feat: a feature")?;
        let three = commit("chore: 1.0.0")?;
        let four = commit("fix: the bug")?;

        let range = repo.revwalk(&format!("{}..", &one[0..7]))?;

        // Act
        let release = Release::try_from(range)?;

        // Assert
        let actual_oids: Vec<String> = release
            .commits
            .iter()
            .map(|commit| commit.commit.oid.to_string())
            .collect();

        assert_that!(actual_oids).is_equal_to(vec![four, three, two]);

        Ok(())
    }

    #[sealed_test]
    fn from_commit_to_head_with_overlapping_tag() -> Result<()> {
        // Arrange
        let repo = git_init_no_gpg()?;

        commit("chore: init")?;
        commit("feat: a commit")?;

        let from = commit("chore: another commit")?;
        let one = commit("feat: a feature")?;
        let two = commit("chore: 1.0.0")?;
        git_tag("1.0.0")?;
        let three = commit("fix: the bug")?;

        let range = repo.revwalk(&format!("{}..", &from[0..7]))?;

        // Act
        let release = Release::try_from(range)?;

        // Assert
        let head_to_v1: Vec<String> = release
            .commits
            .iter()
            .map(|commit| commit.commit.oid.to_string())
            .collect();

        let commit_before_v1: Vec<String> = release
            .previous
            .unwrap()
            .commits
            .iter()
            .map(|commit| commit.commit.oid.to_string())
            .collect();

        assert_that!(head_to_v1).is_equal_to(vec![three]);
        assert_that!(commit_before_v1).is_equal_to(vec![two, one]);

        Ok(())
    }
}
