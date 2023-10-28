use std::str::FromStr;

use git2::Commit as Git2Commit;
use once_cell::sync::Lazy;
use semver::{BuildMetadata, Prerelease, Version};

use crate::conventional::error::BumpError;
use crate::conventional::version::Increment;
use crate::{Commit, IncrementCommand, Repository, RevspecPattern, Tag, SETTINGS};

static FILTER_MERGE_COMMITS: Lazy<fn(&&git2::Commit) -> bool> = Lazy::new(|| {
    |commit| {
        if SETTINGS.ignore_merge_commits {
            commit.parent_count() <= 1
        } else {
            true
        }
    }
});

pub(crate) trait Bump {
    fn manual_bump(&self, version: &str) -> Result<Self, semver::Error>
    where
        Self: Sized;
    fn major_bump(&self) -> Self;
    fn minor_bump(&self) -> Self;
    fn patch_bump(&self) -> Self;
    fn no_bump(&self) -> Self;
    fn auto_bump(&self, repository: &Repository) -> Result<Self, BumpError>
    where
        Self: Sized;
    fn auto_global_bump(
        &self,
        repository: &Repository,
        package_increment: Option<Increment>,
    ) -> Result<Self, BumpError>
    where
        Self: Sized;
    fn auto_package_bump(&self, repository: &Repository, package: &str) -> Result<Self, BumpError>
    where
        Self: Sized;
}

impl Bump for Tag {
    fn manual_bump(&self, version: &str) -> Result<Self, semver::Error> {
        let mut next = self.clone();
        next.version = Version::parse(version)?;
        Ok(next)
    }

    fn major_bump(&self) -> Self {
        let mut next = self.clone();
        next.version.major += 1;
        next.version.minor = 0;
        next.version.patch = 0;
        next.reset_metadata()
    }

    fn minor_bump(&self) -> Self {
        let mut next = self.clone();
        next.version.minor += 1;
        next.version.patch = 0;
        next.reset_metadata()
    }

    fn patch_bump(&self) -> Self {
        let mut next = self.clone();
        next.version.patch += 1;
        next.reset_metadata()
    }

    fn no_bump(&self) -> Self {
        let next = self.clone();
        next.reset_metadata()
    }

    fn auto_bump(&self, repository: &Repository) -> Result<Self, BumpError> {
        self.get_version_from_commit_history(repository)
    }

    fn auto_global_bump(
        &self,
        repository: &Repository,
        package_increment: Option<Increment>,
    ) -> Result<Self, BumpError>
    where
        Self: Sized,
    {
        let tag_from_history = self.get_monorepo_global_version_from_commit_history(repository);
        match (package_increment, tag_from_history) {
            (Some(package_increment), Ok(tag_from_history)) => {
                let tag_from_packages = self.bump(package_increment.into(), repository)?;
                Ok(tag_from_packages.max(tag_from_history))
            }
            (Some(package_increment), Err(_)) => {
                let tag_from_packages = self.bump(package_increment.into(), repository)?;
                Ok(tag_from_packages)
            }
            (None, Ok(tag_from_history)) => Ok(tag_from_history),
            (None, Err(err)) => Err(err),
        }
    }

    fn auto_package_bump(&self, repository: &Repository, package: &str) -> Result<Self, BumpError>
    where
        Self: Sized,
    {
        self.get_package_version_from_commit_history(package, repository)
    }
}

impl Tag {
    pub(crate) fn bump(
        &self,
        increment: IncrementCommand,
        repository: &Repository,
    ) -> Result<Self, BumpError> {
        match increment {
            IncrementCommand::Major => Ok(self.major_bump()),
            IncrementCommand::Minor => Ok(self.minor_bump()),
            IncrementCommand::Patch => Ok(self.patch_bump()),
            IncrementCommand::NoBump => Ok(self.no_bump()),
            IncrementCommand::Auto => self.auto_bump(repository),
            IncrementCommand::AutoPackage(package) => self.auto_package_bump(repository, &package),
            IncrementCommand::AutoMonoRepoGlobal(package_increment) => {
                self.auto_global_bump(repository, package_increment)
            }
            IncrementCommand::Manual(version) => self.manual_bump(&version).map_err(Into::into),
        }
    }

    fn reset_metadata(mut self) -> Self {
        self.version.build = BuildMetadata::EMPTY;
        self.version.pre = Prerelease::EMPTY;
        self.oid = None;
        self
    }

    fn get_version_from_commit_history(&self, repository: &Repository) -> Result<Tag, BumpError> {
        let changelog_start_oid = repository
            .get_latest_tag_oid()
            .ok()
            .unwrap_or_else(|| repository.get_first_commit().expect("non empty repository"));
        let changelog_start_oid = changelog_start_oid.to_string();
        let changelog_start_oid = Some(changelog_start_oid.as_str());

        let pattern = changelog_start_oid
            .map(|oid| format!("{oid}.."))
            .unwrap_or_else(|| "..".to_string());
        let pattern = pattern.as_str();
        let pattern = RevspecPattern::from_str(pattern)?;
        let commits = repository.get_commit_range(&pattern)?;

        let commits: Vec<&Git2Commit> = commits
            .commits
            .iter()
            .filter(&*FILTER_MERGE_COMMITS)
            .collect();

        let conventional_commits: Vec<Commit> = commits
            .iter()
            .map(|commit| Commit::from_git_commit(commit))
            .filter_map(Result::ok)
            .collect();

        let increment_type = self.version_increment_from_commit_history(&conventional_commits)?;

        Ok(match increment_type {
            Increment::Major => self.major_bump(),
            Increment::Minor => self.minor_bump(),
            Increment::Patch => self.patch_bump(),
            Increment::NoBump => self.no_bump(),
        })
    }

    fn get_package_version_from_commit_history(
        &self,
        package: &str,
        repository: &Repository,
    ) -> Result<Tag, BumpError> {
        let changelog_start_oid = repository
            .get_latest_package_tag(package)
            .ok()
            .and_then(|tag| tag.oid)
            .unwrap_or_else(|| repository.get_first_commit().expect("non empty repository"));

        let changelog_start_oid = changelog_start_oid.to_string();
        let changelog_start_oid = Some(changelog_start_oid.as_str());

        let pattern = changelog_start_oid
            .map(|oid| format!("{oid}.."))
            .unwrap_or_else(|| "..".to_string());
        let pattern = pattern.as_str();
        let pattern = RevspecPattern::from_str(pattern)?;
        let commits = repository.get_commit_range_for_package(&pattern, package)?;
        let commits: Vec<&Git2Commit> = commits
            .commits
            .iter()
            .filter(&*FILTER_MERGE_COMMITS)
            .collect();

        let conventional_commits: Vec<Commit> = commits
            .iter()
            .map(|commit| Commit::from_git_commit(commit))
            .filter_map(Result::ok)
            .collect();

        let increment_type = self.version_increment_from_commit_history(&conventional_commits)?;

        Ok(match increment_type {
            Increment::Major => self.major_bump(),
            Increment::Minor => self.minor_bump(),
            Increment::Patch => self.patch_bump(),
            Increment::NoBump => self.no_bump(),
        })
    }

    fn get_monorepo_global_version_from_commit_history(
        &self,
        repository: &Repository,
    ) -> Result<Tag, BumpError> {
        let changelog_start_oid = repository
            .get_latest_tag_oid()
            .ok()
            .unwrap_or_else(|| repository.get_first_commit().expect("non empty repository"));

        let changelog_start_oid = changelog_start_oid.to_string();
        let changelog_start_oid = Some(changelog_start_oid.as_str());

        let pattern = changelog_start_oid
            .map(|oid| format!("{oid}.."))
            .unwrap_or_else(|| "..".to_string());
        let pattern = pattern.as_str();
        let pattern = RevspecPattern::from_str(pattern)?;
        let commits = repository.get_commit_range_for_monorepo_global(&pattern)?;

        let commits: Vec<&Git2Commit> = commits
            .commits
            .iter()
            .filter(&*FILTER_MERGE_COMMITS)
            .collect();

        let conventional_commits: Vec<Commit> = commits
            .iter()
            .map(|commit| Commit::from_git_commit(commit))
            .filter_map(Result::ok)
            .collect();

        let increment_type = self.version_increment_from_commit_history(&conventional_commits)?;

        Ok(match increment_type {
            Increment::Major => self.major_bump(),
            Increment::Minor => self.minor_bump(),
            Increment::Patch => self.patch_bump(),
            Increment::NoBump => self.no_bump(),
        })
    }

    pub fn version_increment_from_commit_history(
        &self,
        commits: &[Commit],
    ) -> Result<Increment, BumpError> {
        let is_major_bump = || self.version.major != 0 && commits.iter().any(Commit::is_major_bump);

        let is_minor_bump = || commits.iter().any(Commit::is_minor_bump);

        let is_patch_bump = || commits.iter().any(Commit::is_patch_bump);

        // At this point, it is not a major, minor or patch bump but we might have found conventional commits
        // -> Must be only chore, docs, refactor ... which means commits that don't require bump but shouldn't throw error
        let no_bump_required = !commits.is_empty();

        if is_major_bump() {
            Ok(Increment::Major)
        } else if is_minor_bump() {
            Ok(Increment::Minor)
        } else if is_patch_bump() {
            Ok(Increment::Patch)
        } else if no_bump_required {
            Ok(Increment::NoBump)
        } else {
            Err(BumpError::NoCommitFound)
        }
    }
}

#[cfg(test)]
mod test {
    use std::collections::HashMap;
    use std::fs;
    use std::path::PathBuf;
    use std::str::FromStr;

    use anyhow::Result;
    use chrono::Utc;
    use cmd_lib::run_cmd;
    use conventional_commit_parser::commit::{CommitType, ConventionalCommit};
    use sealed_test::prelude::*;
    use semver::Version;
    use speculoos::prelude::*;

    use crate::conventional::bump::Bump;
    use crate::conventional::commit::{Commit, CommitConfig};
    use crate::conventional::error::BumpError;
    use crate::conventional::version::{Increment, IncrementCommand};
    use crate::git::repository::Repository;
    use crate::git::tag::Tag;
    use crate::settings::{MonoRepoPackage, Settings};

    impl Commit {
        fn commit_fixture(commit_type: CommitType, is_breaking_change: bool) -> Self {
            Commit {
                oid: "1234".to_string(),
                conventional: ConventionalCommit {
                    commit_type,
                    scope: None,
                    body: None,
                    summary: "message".to_string(),
                    is_breaking_change,
                    footers: vec![],
                },
                author: "".to_string(),
                date: Utc::now().naive_local(),
            }
        }
    }

    #[sealed_test]
    fn major_bump() -> Result<()> {
        // Arrange
        let repository = Repository::init(".")?;
        let base_version = Tag::from_str("1.0.0", None)?;

        // Act
        let tag = base_version.bump(IncrementCommand::Major, &repository)?;

        // Assert
        assert_that!(tag.version).is_equal_to(Version::new(2, 0, 0));
        Ok(())
    }

    #[sealed_test]
    fn minor_bump() -> Result<()> {
        // Arrange
        let repository = Repository::init(".")?;
        let base_version = Tag::from_str("1.0.0", None)?;

        // Act
        let tag = base_version.bump(IncrementCommand::Minor, &repository)?;

        // Assert
        assert_that!(tag.version).is_equal_to(Version::new(1, 1, 0));
        Ok(())
    }

    #[sealed_test]
    fn patch_bump() -> Result<()> {
        // Arrange
        let repository = Repository::init(".")?;
        let base_version = Tag::from_str("1.0.0", None)?;

        // Act
        let tag = base_version.bump(IncrementCommand::Patch, &repository)?;

        // Assert
        assert_that!(tag.version).is_equal_to(Version::new(1, 0, 1));
        Ok(())
    }

    #[sealed_test]
    fn no_bump() -> Result<()> {
        // Arrange
        let repository = Repository::init(".")?;
        let base_version = Tag::from_str("1.0.0", None)?;

        // Act
        let tag = base_version.bump(IncrementCommand::NoBump, &repository)?;

        // Assert
        assert_that!(tag.version).is_equal_to(Version::new(1, 0, 0));
        Ok(())
    }

    #[test]
    fn should_get_next_auto_version_patch() -> Result<()> {
        // Arrange
        let patch = Commit::commit_fixture(CommitType::BugFix, false);
        let base_version = Tag::from_str("1.0.0", None)?;

        // Act
        let increment = base_version.version_increment_from_commit_history(&[patch]);

        // Assert
        assert_that!(increment)
            .is_ok()
            .is_equal_to(Increment::Patch);

        Ok(())
    }

    #[test]
    fn should_not_bump_versions_due_to_non_bump_commits() -> Result<()> {
        // Arrange
        let revert = Commit::commit_fixture(CommitType::Revert, false);
        let perf = Commit::commit_fixture(CommitType::Performances, false);
        let documentation = Commit::commit_fixture(CommitType::Documentation, false);
        let chore = Commit::commit_fixture(CommitType::Chore, false);
        let style = Commit::commit_fixture(CommitType::Style, false);
        let refactor = Commit::commit_fixture(CommitType::Refactor, false);
        let test = Commit::commit_fixture(CommitType::Test, false);
        let build = Commit::commit_fixture(CommitType::Build, false);
        let ci = Commit::commit_fixture(CommitType::Ci, false);

        let base_version = Tag::from_str("1.0.0", None)?;

        // Act
        let increment = base_version.version_increment_from_commit_history(&[
            revert,
            perf,
            documentation,
            chore,
            style,
            refactor,
            test,
            build,
            ci,
        ]);

        // Assert
        assert_that!(increment)
            .is_ok()
            .is_equal_to(Increment::NoBump);

        Ok(())
    }

    #[test]
    fn increment_minor_version_should_set_patch_to_zero() -> Result<()> {
        // Arrange
        let repository = Repository::init(".")?;
        let version = Tag::from_str("1.1.1", None)?;

        // Act
        let tag = version.bump(IncrementCommand::Minor, &repository)?;

        // Assert
        assert_that!(tag.version).is_equal_to(Version::from_str("1.2.0")?);

        Ok(())
    }

    #[sealed_test]
    fn increment_major_version_should_set_minor_and_patch_to_zero() -> Result<()> {
        // Arrange
        let repository = Repository::init(".")?;
        let version = Tag::from_str("1.1.1", None)?;

        // Act
        let tag = version.bump(IncrementCommand::Major, &repository)?;

        // Assert
        assert_that!(tag.version).is_equal_to(Version::from_str("2.0.0")?);

        Ok(())
    }

    #[sealed_test]
    fn increment_should_strip_metadata() -> Result<()> {
        // Arrange
        let repository = Repository::init(".")?;
        let version = Tag::from_str("1.1.1-pre+10.1", None)?;

        // Act
        let tag = version.bump(IncrementCommand::Patch, &repository)?;

        // Assert
        assert_that!(tag.version).is_equal_to(Version::from_str("1.1.2")?);

        Ok(())
    }

    #[test]
    fn should_get_next_auto_version_breaking_changes() -> Result<()> {
        // Arrange
        let feature = Commit::commit_fixture(CommitType::Feature, false);
        let breaking_change = Commit::commit_fixture(CommitType::Feature, true);
        let base_version = Tag::from_str("1.0.0", None)?;

        // Act
        let version =
            base_version.version_increment_from_commit_history(&[feature, breaking_change]);

        // Assert
        assert_that!(version).is_ok().is_equal_to(Increment::Major);

        Ok(())
    }

    #[test]
    fn should_get_next_auto_version_breaking_changes_on_initial_dev_version() -> Result<()> {
        // Arrange
        let feature = Commit::commit_fixture(CommitType::Feature, false);
        let breaking_change = Commit::commit_fixture(CommitType::Feature, true);
        let base_version = Tag::from_str("0.1.0", None)?;

        // Act
        let version =
            base_version.version_increment_from_commit_history(&[feature, breaking_change]);

        // Assert
        assert_that!(version).is_ok().is_equal_to(Increment::Minor);

        Ok(())
    }

    #[test]
    fn should_get_next_auto_version_minor() -> Result<()> {
        // Arrange
        let patch = Commit::commit_fixture(CommitType::BugFix, false);
        let feature = Commit::commit_fixture(CommitType::Feature, false);
        let base_version = Tag::from_str("0.1.0", None)?;

        // Act
        let version = base_version.version_increment_from_commit_history(&[patch, feature]);

        // Assert
        assert_that!(version).is_ok().is_equal_to(Increment::Minor);

        Ok(())
    }

    #[sealed_test]
    fn should_get_next_auto_version_minor_with_custom_commit_type() -> Result<()> {
        // Arrange
        Repository::init(".")?;
        let mut commit_types = HashMap::new();
        commit_types.insert("ex".to_string(), CommitConfig::new("Ex").with_minor_bump());
        let settings = Settings {
            commit_types,
            ..Default::default()
        };

        let settings = toml::to_string(&settings)?;
        fs::write("cog.toml", settings)?;

        let patch = Commit::commit_fixture(CommitType::BugFix, false);
        let feature = Commit::commit_fixture(CommitType::Custom("ex".to_string()), false);
        let base_version = Tag::from_str("0.1.0", None)?;

        // Act
        let version = base_version.version_increment_from_commit_history(&[patch, feature]);

        // Assert
        assert_that!(version).is_ok().is_equal_to(Increment::Minor);

        Ok(())
    }

    #[sealed_test]
    fn should_get_next_auto_version_patch_with_custom_commit_type() -> Result<()> {
        // Arrange
        Repository::init(".")?;
        let mut commit_types = HashMap::new();
        commit_types.insert("ex".to_string(), CommitConfig::new("Ex").with_patch_bump());
        let settings = Settings {
            commit_types,
            ..Default::default()
        };

        let settings = toml::to_string(&settings)?;
        fs::write("cog.toml", settings)?;

        let patch = Commit::commit_fixture(CommitType::Chore, false);
        let feature = Commit::commit_fixture(CommitType::Custom("ex".to_string()), false);
        let base_version = Tag::from_str("0.1.0", None)?;

        // Act
        let version = base_version.version_increment_from_commit_history(&[patch, feature]);

        // Assert
        assert_that!(version).is_ok().is_equal_to(Increment::Patch);

        Ok(())
    }

    #[sealed_test]
    fn should_override_bump_behavior_for_existing_commit_type() -> Result<()> {
        // Arrange
        Repository::init(".")?;
        let mut commit_types = HashMap::new();
        commit_types.insert(
            "perf".to_string(),
            CommitConfig::new("Perf").with_minor_bump(),
        );
        let settings = Settings {
            commit_types,
            ..Default::default()
        };

        let settings = toml::to_string(&settings)?;
        fs::write("cog.toml", settings)?;

        let patch = Commit::commit_fixture(CommitType::Chore, false);
        let feature = Commit::commit_fixture(CommitType::Performances, false);
        let base_version = Tag::from_str("0.1.0", None)?;

        // Act
        let version = base_version.version_increment_from_commit_history(&[patch, feature]);

        // Assert
        assert_that!(version).is_ok().is_equal_to(Increment::Minor);

        Ok(())
    }

    #[test]
    fn should_not_fail_without_feature_bug_fix_or_breaking_change_commit() -> Result<()> {
        // Arrange
        let chore = Commit::commit_fixture(CommitType::Chore, false);
        let docs = Commit::commit_fixture(CommitType::Documentation, false);
        let base_version = Tag::from_str("0.1.0", None)?;

        // Act
        let version = base_version.version_increment_from_commit_history(&[chore, docs]);

        // Assert
        assert_that!(version).is_ok().is_equal_to(Increment::NoBump);

        Ok(())
    }

    #[sealed_test]
    fn get_global_monorepo_version_from_history_should_fail_with_only_package_commit() -> Result<()>
    {
        // Arrange
        let repository = init_monorepo()?;
        run_cmd!(
            echo "feature" > one;
            git add .;
            git commit -m "feat: feature package one";
        )?;

        let base_version = Tag::from_str("0.1.0", None)?;

        // Act
        let tag = base_version.get_monorepo_global_version_from_commit_history(&repository);

        // Assert
        assert_that!(tag)
            .is_err()
            .matches(|err| matches!(err, BumpError::NoCommitFound));

        Ok(())
    }

    #[sealed_test]
    fn monorepo_auto_bump_should_succeed_with_only_package_commits() -> Result<()> {
        // Arrange
        let repository = init_monorepo()?;
        run_cmd!(
            echo "feature" > one;
            git add .;
            git commit -m "feat: feature package one";
        )?;
        let base_version = Tag::from_str("0.1.0", None)?;

        // Act
        let tag = base_version.auto_global_bump(&repository, Some(Increment::Minor))?;

        // Assert
        assert_that!(tag.version).is_equal_to(Version::new(0, 2, 0));

        Ok(())
    }

    #[sealed_test]
    fn monorepo_auto_bump_should_succeed_with_only_global_commits() -> Result<()> {
        // Arrange
        let repository = init_monorepo()?;

        run_cmd!(
            echo "global" > global;
            git add .;
            git commit -m "feat: non package commit";
        )?;

        // Act
        let tag = Tag::default().auto_global_bump(&repository, None)?;

        // Assert
        assert_that!(tag.version).is_equal_to(Version::new(0, 1, 0));

        Ok(())
    }

    #[sealed_test]
    fn monorepo_auto_bump_should_succeed_selecting_history_bump() -> Result<()> {
        // Arrange
        let repository = init_monorepo()?;

        // Patch increment from global commits
        // Minor increment from package bumps
        run_cmd!(
            echo "global" > global;
            git add .;
            git commit -m "fix: global fix";
            echo "feature" > one;
            git add .;
            git commit -m "feat: feature 1 package one";
        )?;

        // Act
        let tag = Tag::default().auto_global_bump(&repository, Some(Increment::Minor))?;

        // Assert
        assert_that!(tag.version).is_equal_to(Version::new(0, 1, 0));

        Ok(())
    }

    #[sealed_test]
    fn monorepo_auto_bump_should_succeed_selecting_package_bump() -> Result<()> {
        // Arrange
        let repository = init_monorepo()?;

        // Minor increment from global commits
        // Patch increment from package bumps
        run_cmd!(
            echo "global" > global;
            git add .;
            git commit -m "feat: global fix";
            echo "feature" > one;
            git add .;
            git commit -m "fix: fix 1 package one";
        )?;

        // Act
        let tag = Tag::default().auto_global_bump(&repository, Some(Increment::Patch))?;

        // Assert
        assert_that!(tag.version).is_equal_to(Version::new(0, 1, 0));

        Ok(())
    }

    #[sealed_test]
    fn monorepo_auto_bump_should_succeed_with_equals_history_and_package() -> Result<()> {
        // Arrange
        let repository = init_monorepo()?;

        // Minor increment from global commits
        // Minor increment from package bumps
        run_cmd!(
            echo "global" > global;
            git add .;
            git commit -m "feat: global fix";
            echo "feature" > one;
            git add .;
            git commit -m "feature: package one";
        )?;

        // Act
        let tag = Tag::default().auto_global_bump(&repository, Some(Increment::Minor))?;

        // Assert
        assert_that!(tag.version).is_equal_to(Version::new(0, 1, 0));

        Ok(())
    }

    #[sealed_test]
    fn monorepo_auto_bump_should_succeed_with_mixed_commit() -> Result<()> {
        // Arrange
        let repository = init_monorepo()?;

        // Minor increment from global commits
        // Minor increment from package bumps
        run_cmd!(
            echo "start" > start;
            git add .;
            git commit -m "chore: version";
            git tag "0.1.0";
            git tag "one-0.1.0";
            echo "feature" > one;
            echo "global" > global;
            git add .;
            git commit -m "feature: package one and global";
        )?;

        // Act
        let tag =
            Tag::from_str("0.1.0", None)?.auto_global_bump(&repository, Some(Increment::Minor))?;

        // Assert
        assert_that!(tag.version).is_equal_to(Version::new(0, 2, 0));

        Ok(())
    }

    fn init_monorepo() -> Result<Repository> {
        let repository = Repository::init(".")?;
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
            git commit -m "chore: first commit";
        )?;

        Ok(repository)
    }
}
