use std::fmt;
use std::fmt::Formatter;
use std::path::Path;
use std::str::FromStr;

use git2::{Commit, DescribeFormatOptions, DescribeOptions, ErrorCode, Oid};
use globset::{Candidate, GlobBuilder, GlobSet, GlobSetBuilder};

use crate::conventional::changelog::release::Release;
use crate::git::error::Git2Error;
use crate::git::oid::OidOf;
use crate::git::repository::Repository;
use crate::git::tag::Tag;
use crate::settings::MonoRepoPackage;
use crate::SETTINGS;

#[derive(Debug)]
pub struct CommitRange<'repo> {
    pub from: OidOf,
    pub to: OidOf,
    pub commits: Vec<Commit<'repo>>,
}

#[derive(Debug, Default)]
pub struct RevspecPattern {
    from: Option<String>,
    to: Option<String>,
}

impl fmt::Display for RevspecPattern {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        let from = self.from.as_deref().unwrap_or("");
        let to = self.to.as_deref().unwrap_or("");
        write!(f, "{from}..{to}")
    }
}

impl FromStr for RevspecPattern {
    type Err = Git2Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if let Some((from, to)) = s.split_once("..") {
            let from = if from.is_empty() {
                None
            } else {
                Some(from.to_string())
            };

            let to = if to.is_empty() {
                None
            } else {
                Some(to.to_string())
            };

            Ok(RevspecPattern { from, to })
        } else {
            Err(Git2Error::InvalidCommitRangePattern(s.to_string()))
        }
    }
}

impl From<(&str, &str)> for RevspecPattern {
    fn from((from, to): (&str, &str)) -> Self {
        Self {
            from: Some(from.to_string()),
            to: Some(to.to_string()),
        }
    }
}

#[derive(Debug)]
struct PackagePathFilter {
    include: GlobSet,
    exclude: GlobSet,
}

impl PackagePathFilter {
    fn new(package_path: &str, include_paths: &[String], exclude_paths: &[String]) -> Self {
        let include = {
            let mut builder = GlobSetBuilder::new();
            builder.add(
                GlobBuilder::new(format!("{}/**", package_path).as_str())
                    .literal_separator(true)
                    .build()
                    .expect("glob should be valid"),
            );
            for include in include_paths {
                builder.add(
                    GlobBuilder::new(include)
                        .literal_separator(true)
                        .build()
                        .expect("glob should be valid"),
                );
            }
            builder.build().expect("valid globset")
        };
        let exclude = {
            let mut builder = GlobSetBuilder::new();
            for exclude in exclude_paths {
                builder.add(
                    GlobBuilder::new(exclude)
                        .literal_separator(true)
                        .build()
                        .expect("glob should be valid"),
                );
            }
            builder.build().expect("valid globset")
        };

        PackagePathFilter { include, exclude }
    }

    fn from_package(package: &MonoRepoPackage) -> Self {
        Self::new(
            package.path.to_str().expect("valid package path"),
            &package.include,
            &package.ignore,
        )
    }

    fn is_match<P: AsRef<Path> + ?Sized>(&self, path: &P) -> bool {
        let candidate = Candidate::new(path);
        self.include.is_match_candidate(&candidate) && !self.exclude.is_match_candidate(&candidate)
    }
}

impl Repository {
    /// Return a [`CommitRange`] containing all commit in the current repository
    pub fn all_commits(&self) -> Result<CommitRange, Git2Error> {
        let mut revwalk = self.0.revwalk()?;
        revwalk.push_head()?;
        let mut commits = vec![];

        for oid in revwalk {
            match oid {
                Ok(oid) => {
                    let commit = self.0.find_commit(oid)?;
                    commits.push(commit)
                }
                Err(e) if e.code() == ErrorCode::NotFound => {
                    break;
                }
                Err(e) => return Err(Git2Error::from(e)),
            }
        }

        let to = commits
            .first()
            .map(|commit| commit.id())
            .map(OidOf::Head)
            .expect("No commit found");

        let from = commits
            .last()
            .map(|commit| commit.id())
            .map(OidOf::Other)
            .expect("No commit found");

        Ok(CommitRange { from, to, commits })
    }

    pub(crate) fn get_release_range(&self, pattern: RevspecPattern) -> Result<Release, Git2Error> {
        let target = if let Some(target) = pattern.from {
            self.resolve_oid_of(&target)
        } else {
            OidOf::Other(self.get_first_commit()?)
        };

        let pattern = RevspecPattern {
            from: None,
            to: pattern.to,
        };

        let range = self.get_commit_range(&pattern)?;
        let release = Release::from(range);

        let mut release = if !release.contains_oid(target.oid()) {
            self.populate_previous_release(release, target.oid())?
        } else {
            release
        };

        release.drain_to_target(target.oid());

        Ok(release)
    }

    fn populate_previous_release<'a>(
        &'a self,
        mut release: Release<'a>,
        target: &Oid,
    ) -> Result<Release<'a>, Git2Error> {
        let pattern = format!("..{}", release.from);
        let pattern = RevspecPattern::from_str(pattern.as_str())?;
        let range = self.get_commit_range(&pattern)?;

        let target_in_range = range.commits.iter().any(|commit| commit.id() == *target);

        // Target tag or commit reached
        if range.to.oid() == target {
            return Ok(release);
        }
        // We have reached the `from` target commit
        else if target_in_range {
            if range.from != range.to {
                let previous = Release::from(range);
                release.previous = Some(Box::new(previous));
            }

            return Ok(release);
        }

        let previous = Release::from(range);
        let previous = self.populate_previous_release(previous, target)?;
        release.previous = Some(Box::new(previous));

        Ok(release)
    }

    /// Return a commit range
    /// `from` : either a tag or an oid, latest tag if none, fallbacks to first commit
    /// `to`: HEAD if none
    pub fn get_commit_range(&self, pattern: &RevspecPattern) -> Result<CommitRange, Git2Error> {
        let from = pattern.from.as_deref();
        let to = pattern.to.as_deref();

        // Is the given `to` arg a tag or an oid ?
        let maybe_to_tag = match to {
            // No target tag provided, check if HEAD is tagged
            None => {
                let head = self.get_head_commit_oid()?;
                self.get_latest_tag()
                    .ok()
                    .filter(|tag| *tag.oid_unchecked() == head)
            }
            // Try to resolve a tag from the provided range, ex: ..1.0.0
            Some(to) => self.resolve_tag(to).ok(),
        };

        // get/validate the target oid
        let to = match to {
            None => self.get_head_commit_oid()?,
            Some(to) => self.0.revparse_single(to)?.id(),
        };

        // Either user input, latest tag since `to`, or first commit
        let from = match from {
            // No `from` arg provided get latest tag in `to` parents
            None => self
                .get_latest_tag_starting_from(to)
                .map(OidOf::Tag)
                // No tag in the tree, fallback to first commit
                .unwrap_or_else(|_| {
                    self.get_first_commit()
                        .map(OidOf::Other)
                        .expect("No commit found")
                }),
            // We might have a tag
            Some(from) => self.resolve_oid_of(from),
        };

        // Resolve shorthands and tags
        let spec = format!("{from}..{to}");
        // Attempt to resolve tag names, fallback to oid
        let to = maybe_to_tag
            .map(OidOf::Tag)
            .unwrap_or_else(|| OidOf::Other(to));

        let commits = self.get_commit_range_from_spec(&spec)?;

        Ok(CommitRange { from, to, commits })
    }

    pub fn get_commit_range_for_package(
        &self,
        pattern: &RevspecPattern,
        package: &str,
    ) -> Result<CommitRange, Git2Error> {
        let mut commit_range = self.get_commit_range(pattern)?;
        let mut commits = vec![];
        let package = SETTINGS.packages.get(package).expect("package exists");
        let package_path_filter = PackagePathFilter::from_package(package);

        for commit in commit_range.commits {
            let parent = commit.parent(0).ok().map(|commit| commit.id().to_string());

            let parent_tree = self.tree_to_treeish(parent.as_ref())?;

            let current_tree = self
                .tree_to_treeish(Some(&commit.id().to_string()))?
                .expect("Failed to get commit tree");

            let diff = match parent_tree {
                None => self
                    .0
                    .diff_tree_to_tree(None, current_tree.as_tree(), None)?,
                Some(tree) => {
                    self.0
                        .diff_tree_to_tree(tree.as_tree(), current_tree.as_tree(), None)?
                }
            };

            for delta in diff.deltas() {
                if let Some(old) = delta.old_file().path() {
                    if package_path_filter.is_match(old) {
                        commits.push(commit);
                        break;
                    }
                }

                if let Some(new) = delta.new_file().path() {
                    if package_path_filter.is_match(new) {
                        commits.push(commit);
                        break;
                    }
                }
            }
        }

        commit_range.commits = commits;
        Ok(commit_range)
    }

    pub fn get_commit_range_for_monorepo_global(
        &self,
        pattern: &RevspecPattern,
    ) -> Result<CommitRange, Git2Error> {
        let mut commit_range = self.get_commit_range(pattern)?;
        let mut commits = vec![];
        let package_paths: Vec<_> = SETTINGS
            .packages
            .values()
            .map(|package| &package.path)
            .collect();

        for commit in commit_range.commits {
            let parent = commit.parent(0)?.id().to_string();
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
                        commits.push(commit);
                        break;
                    }
                }

                if let Some(new) = delta.new_file().path() {
                    if package_paths.iter().all(|path| !new.starts_with(path)) {
                        commits.push(commit);
                        break;
                    }
                }
            }
        }

        commit_range.commits = commits;
        Ok(commit_range)
    }

    fn resolve_oid_of(&self, from: &str) -> OidOf {
        // either we have a tag name
        self.resolve_tag(from)
            .map(OidOf::Tag)
            // Or an oid
            .unwrap_or_else(|_| {
                let object = self.0.revparse_single(from).expect("Expected oid or tag");

                // Is the oid pointing to a tag ?
                let tag = self
                    .all_tags()
                    .expect("Error trying to get repository tags")
                    .into_iter()
                    .find(|tag| *tag.oid_unchecked() == object.id());

                match tag {
                    None => OidOf::Other(object.id()),
                    Some(tag) => OidOf::Tag(tag),
                }
            })
    }

    fn get_commit_range_from_spec(&self, spec: &str) -> Result<Vec<Commit>, Git2Error> {
        let mut revwalk = self.0.revwalk()?;

        revwalk.push_range(spec)?;

        let mut commits: Vec<Commit> = vec![];

        for oid in revwalk {
            let oid = oid?;
            let commit = self.0.find_commit(oid)?;
            commits.push(commit);
        }

        Ok(commits)
    }

    // Hide all commit after `starting_point` and get the closest tag
    fn get_latest_tag_starting_from(&self, starting_point: Oid) -> Result<Tag, Git2Error> {
        let starting_point = self.0.find_commit(starting_point)?;
        let starting_point = starting_point.parent(0)?;
        let describe = starting_point
            .as_object()
            .describe(DescribeOptions::new().describe_tags())?;
        let tag_name = describe.format(Some(DescribeFormatOptions::new().abbreviated_size(0)))?;
        let tag_oid = self.0.refname_to_id(&format!("refs/tags/{tag_name}")).ok();
        Tag::from_str(&tag_name, tag_oid).map_err(|_| Git2Error::NoTagFound)
    }
}

#[cfg(test)]
mod test {
    use crate::conventional::changelog::release::Release;
    use anyhow::{anyhow, Result};
    use cmd_lib::{run_cmd, run_fun};
    use git2::Oid;
    use sealed_test::prelude::*;
    use speculoos::prelude::*;
    use std::collections::HashMap;
    use std::path::PathBuf;
    use std::str::FromStr;

    use crate::git::oid::OidOf;
    use crate::git::repository::Repository;
    use crate::git::revspec::RevspecPattern;
    use crate::git::tag::Tag;
    use crate::settings::{MonoRepoPackage, Settings};

    const COCOGITTO_REPOSITORY: &str = env!("CARGO_MANIFEST_DIR");

    #[test]
    fn convert_str_to_pattern_to() -> Result<()> {
        let pattern = RevspecPattern::from_str("..1.0.0")?;

        assert_that!(pattern.from).is_none();
        assert_that!(pattern.to)
            .is_some()
            .is_equal_to("1.0.0".to_string());
        Ok(())
    }

    #[test]
    fn convert_str_to_pattern_from() -> Result<()> {
        let pattern = RevspecPattern::from_str("1.0.0..")?;

        assert_that!(pattern.from)
            .is_some()
            .is_equal_to("1.0.0".to_string());
        assert_that!(pattern.to).is_none();
        Ok(())
    }

    #[test]
    fn convert_empty_pattern() -> Result<()> {
        let pattern = RevspecPattern::from_str("..")?;

        assert_that!(pattern.from).is_none();
        assert_that!(pattern.to).is_none();
        Ok(())
    }

    #[test]
    fn error_invalid_pattern() {
        let result = RevspecPattern::from_str("1.0.0");
        assert!(result.is_err());
        assert_eq!(
            result.unwrap_err().to_string(),
            "invalid commit range pattern: `1.0.0`\n"
        );
    }

    #[test]
    fn convert_full_pattern() -> Result<()> {
        let pattern = RevspecPattern::from_str("1.0.0..2.0.0")?;

        assert_that!(pattern.from)
            .is_some()
            .is_equal_to("1.0.0".to_string());
        assert_that!(pattern.to)
            .is_some()
            .is_equal_to("2.0.0".to_string());
        Ok(())
    }

    #[test]
    fn all_commits() -> Result<()> {
        // Arrange
        let repo = Repository::open(COCOGITTO_REPOSITORY)?;

        // Act
        let range = repo.all_commits()?;

        // Assert
        assert_that!(range.commits).is_not_empty();
        Ok(())
    }

    #[test]
    fn get_release_range_integration_test() -> Result<()> {
        // Arrange
        let repo = Repository::open(COCOGITTO_REPOSITORY)?;
        let format_version = |release: &Release| format!("{}", release.version);

        // Act
        let release = repo.get_release_range(RevspecPattern::from_str("0.32.1..0.32.3")?)?;

        // Assert
        assert_that!(format_version(&release)).is_equal_to("0.32.3".to_string());

        let release = *release.previous.unwrap();
        assert_that!(format_version(&release)).is_equal_to("0.32.2".to_string());

        assert_that!(release.previous).is_none();
        Ok(())
    }

    #[sealed_test]
    fn get_annotated_tag_commits() -> Result<()> {
        // Arrange
        let repo = Repository::init(".")?;
        run_cmd!(
            git init;
            echo changes > file;
            git add .;
        )?;

        let start = repo.commit("chore: init", false, false)?;

        run_cmd!(
            git init;
            echo changes > file2;
            git add .;
        )?;

        let _end = repo.commit("chore: 1.0.0", false, false)?;

        // Create an annotated tag
        let head = repo.get_head_commit().unwrap();
        let sig = git2::Signature::now("Author", "email@example.com")?;
        repo.0
            .tag("1.0.0", &head.into_object(), &sig, "the_msg", false)?;

        run_cmd!(
            git init;
            echo changes > file3;
            git add .;
        )?;

        repo.commit("feat: a commit", false, false)?;

        let commit_range = repo.get_commit_range(&RevspecPattern::from_str("..1.0.0")?)?;

        assert_that!(commit_range.from).is_equal_to(OidOf::Other(start));
        assert_that!(commit_range.to.to_string()).is_equal_to("1.0.0".to_string());
        assert_that!(commit_range.commits).has_length(1);
        Ok(())
    }

    #[sealed_test]
    fn get_package_commit_range() -> Result<()> {
        // Arrange
        let repo = Repository::init(".")?;
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
            git init;
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

        let commit_range_package =
            repo.get_commit_range_for_package(&RevspecPattern::from_str("..HEAD")?, "one")?;
        let commit_range_global =
            repo.get_commit_range_for_monorepo_global(&RevspecPattern::from_str("..HEAD")?)?;

        assert_that!(commit_range_package.commits).has_length(1);
        assert_that!(commit_range_global.commits).has_length(1);
        Ok(())
    }

    #[sealed_test]
    fn get_package_commit_range_with_filtering() -> Result<()> {
        // Arrange
        let repo = Repository::init(".")?;
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
            git init;
            echo $settings > cog.toml;
            mkdir -p shared one/ignored;
            git add .;
            git commit -m "chore: First commit";
        )?;

        let commit_range_package =
            repo.get_commit_range_for_package(&RevspecPattern::from_str("..HEAD")?, "one")?;
        assert_that!(commit_range_package.commits).has_length(0);

        run_cmd!(
            echo changes > one/file;
            git add .;
            git commit -m "feat: commit to non-ignored path";
        )?;

        let commit_range_package =
            repo.get_commit_range_for_package(&RevspecPattern::from_str("HEAD~1..HEAD")?, "one")?;
        assert_that!(commit_range_package.commits).has_length(1);

        run_cmd!(
            echo changes > one/ignored/file;
            git add .;
            git commit -m "feat: commit to ignored path";
        )?;

        let commit_range_package =
            repo.get_commit_range_for_package(&RevspecPattern::from_str("HEAD~1..HEAD")?, "one")?;
        assert_that!(commit_range_package.commits).has_length(0);

        run_cmd!(
            echo changes > shared/file;
            git add .;
            git commit -m "feat: commit to extra included path";
        )?;

        let commit_range_package =
            repo.get_commit_range_for_package(&RevspecPattern::from_str("HEAD~1..HEAD")?, "one")?;
        assert_that!(commit_range_package.commits).has_length(1);

        run_cmd!(
            echo changes > README.md;
            git add .;
            git commit -m "feat: commit to extra included file";
        )?;

        let commit_range_package =
            repo.get_commit_range_for_package(&RevspecPattern::from_str("HEAD~1..HEAD")?, "one")?;
        assert_that!(commit_range_package.commits).has_length(1);

        Ok(())
    }

    #[sealed_test]
    fn get_tag_commits() -> Result<()> {
        // Arrange
        let repo = Repository::init(".")?;

        run_cmd!(
            git init;
            echo changes > file;
            git add .;
        )?;

        let start = repo.commit("chore: init", false, false)?;

        run_cmd!(
            git commit --allow-empty -m "chore: 1.0.0";
            git tag 1.0.0;
            git commit --allow-empty -m "feat: a commit";
        )?;

        // Act
        let commit_range = repo.get_commit_range(&RevspecPattern::from_str("..1.0.0")?)?;

        // Assert
        assert_that!(commit_range.from).is_equal_to(OidOf::Other(start));
        assert_that!(commit_range.to.to_string()).is_equal_to("1.0.0".to_string());
        assert_that!(commit_range.commits).has_length(1);
        Ok(())
    }

    #[test]
    fn from_tag_to_tag_ok() -> Result<()> {
        // Arrange
        let repo = Repository::open(COCOGITTO_REPOSITORY)?;
        let v1_0_0 = Oid::from_str("549070fa99986b059cbaa9457b6b6f065bbec46b")?;
        let v1_0_0 = OidOf::Tag(Tag::from_str("1.0.0", Some(v1_0_0))?);
        let v3_0_0 = Oid::from_str("c6508e243e2816e2d2f58828ee0c6721502958dd")?;
        let v3_0_0 = OidOf::Tag(Tag::from_str("3.0.0", Some(v3_0_0))?);

        // Act
        let range = repo.get_commit_range(&RevspecPattern::from_str("1.0.0..3.0.0")?)?;

        // Assert
        assert_that!(range.from).is_equal_to(v1_0_0);
        assert_that!(range.to).is_equal_to(v3_0_0);

        Ok(())
    }

    #[test]
    fn from_tag_to_head() -> Result<()> {
        // Arrange
        let repo = Repository::open(COCOGITTO_REPOSITORY)?;
        let head = repo.get_head_commit_oid()?;
        let head = OidOf::Other(head);
        let tag = repo.get_latest_tag()?;

        // Cover the case when we release a version and run the test in the CI right after that
        let head = if tag.oid() == Some(head.oid()) {
            OidOf::Tag(tag)
        } else {
            head
        };

        let v1_0_0 = Oid::from_str("549070fa99986b059cbaa9457b6b6f065bbec46b")?;
        let v1_0_0 = OidOf::Tag(Tag::from_str("1.0.0", Some(v1_0_0))?);

        // Act
        let range = repo.get_commit_range(&RevspecPattern::from_str("1.0.0..")?)?;

        // Assert
        assert_that!(range.from).is_equal_to(v1_0_0);
        assert_that!(range.to).is_equal_to(head);

        Ok(())
    }

    #[test]
    fn from_latest_to_head() -> Result<()> {
        // Arrange
        let repo = Repository::open(COCOGITTO_REPOSITORY)?;
        let head = repo.get_head_commit_oid()?;
        let head = OidOf::Other(head);
        let mut tags = repo.all_tags()?;
        tags.sort();
        let mut latest = tags.last().unwrap();

        if latest.oid().unwrap() == head.oid() {
            latest = &tags[tags.len() - 2];
        }

        let latest = OidOf::Tag(latest.clone());

        // Act
        let range = repo.get_commit_range(&RevspecPattern::default())?;

        // Assert
        assert_that!(range.from).is_equal_to(latest);
        assert_that!(range.to.oid()).is_equal_to(head.oid());

        Ok(())
    }

    #[test]
    fn from_previous_to_tag() -> Result<()> {
        // Arrange
        let repo = Repository::open(COCOGITTO_REPOSITORY)?;
        let v2_1_1 = Oid::from_str("9dcf728d2eef6b5986633dd52ecbe9e416234898")?;
        let v2_1_1 = OidOf::Tag(Tag::from_str("2.1.1", Some(v2_1_1))?);
        let v3_0_0 = Oid::from_str("c6508e243e2816e2d2f58828ee0c6721502958dd")?;
        let v3_0_0 = OidOf::Tag(Tag::from_str("3.0.0", Some(v3_0_0))?);

        // Act
        let range = repo.get_commit_range(&RevspecPattern::from_str("..3.0.0")?)?;

        // Assert
        assert_that!(range.from).is_equal_to(v2_1_1);
        assert_that!(range.to).is_equal_to(v3_0_0);

        Ok(())
    }

    #[test]
    fn recursive_from_origin_to_head() -> Result<()> {
        // Arrange
        let repo = Repository::open(COCOGITTO_REPOSITORY)?;
        let mut tag_count = repo.0.tag_names(None)?.len();
        let head = repo.get_head_commit_oid()?;
        let latest = repo.get_latest_tag()?;
        let latest = latest.oid();
        if latest == Some(&head) {
            tag_count -= 1;
        };

        // Act
        let mut release = repo.get_release_range(RevspecPattern::from_str("..")?)?;
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
        let repo = Repository::init(".")?;

        let commit: fn(&str) -> Result<String> = |message| {
            run_fun!(
                git commit --allow-empty -q -m $message;
                git log --format=%H -n 1;
            )
            .map_err(|e| anyhow!(e))
        };

        run_cmd!(
            git init;
            echo changes > file;
            git add .;
        )?;

        commit("chore: init")?;
        commit("feat: a commit")?;

        let from = commit("chore: another commit")?;
        let one = commit("feat: a feature")?;
        let two = commit("chore: 1.0.0")?;
        let three = commit("fix: the bug")?;

        let pattern = format!("{}..", &from[0..7]);

        // Act
        let release = repo.get_release_range(RevspecPattern::from_str(pattern.as_str())?)?;

        // Assert
        let oids: Vec<String> = release
            .commits
            .iter()
            .map(|commit| commit.commit.oid.to_string())
            .collect();

        assert_that!(oids).is_equal_to(vec![three, two, one, from]);

        Ok(())
    }

    #[sealed_test]
    fn from_commit_to_head_with_overlapping_tag() -> Result<()> {
        // Arrange
        let repo = Repository::init(".")?;

        let commit: fn(&str) -> Result<String> = |message| {
            run_fun!(
                git commit --allow-empty -q -m $message;
                git log --format=%H -n 1;
            )
            .map_err(|e| anyhow!(e))
        };

        run_cmd!(
            git init;
            echo changes > file;
            git add .;
        )?;

        commit("chore: init")?;
        commit("feat: a commit")?;

        let from = commit("chore: another commit")?;
        let one = commit("feat: a feature")?;
        let two = commit("chore: 1.0.0")?;
        run_cmd!(git tag 1.0.0)?;
        let three = commit("fix: the bug")?;

        let pattern = format!("{}..", &from[0..7]);

        // Act
        let release = repo.get_release_range(RevspecPattern::from_str(pattern.as_str())?)?;

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
        assert_that!(commit_before_v1).is_equal_to(vec![two, one, from]);

        Ok(())
    }
}
