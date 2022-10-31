use crate::conventional::error::BumpError;
use crate::{Commit, Repository, RevspecPattern, Tag, VersionIncrement};
use conventional_commit_parser::commit::CommitType;
use git2::Commit as Git2Commit;
use semver::{BuildMetadata, Prerelease, Version};

pub(crate) trait Bump {
    fn manual_bump(&self, version: &str) -> Result<Self, semver::Error>
    where
        Self: Sized;
    fn major_bump(&self) -> Self;
    fn minor_bump(&self) -> Self;
    fn patch_bump(&self) -> Self;
    fn auto_bump(&self, repository: &Repository) -> Result<Self, BumpError>
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

    fn auto_bump(&self, repository: &Repository) -> Result<Self, BumpError> {
        self.create_version_from_commit_history(repository)
    }
}

impl Tag {
    pub(crate) fn bump(
        &self,
        increment: VersionIncrement,
        repository: &Repository,
    ) -> Result<Self, BumpError> {
        match increment {
            VersionIncrement::Major => Ok(self.major_bump()),
            VersionIncrement::Minor => Ok(self.minor_bump()),
            VersionIncrement::Patch => Ok(self.patch_bump()),
            VersionIncrement::Auto => self.auto_bump(repository),
            VersionIncrement::Manual(version) => self.manual_bump(&version).map_err(Into::into),
        }
    }

    fn reset_metadata(mut self) -> Self {
        self.version.build = BuildMetadata::EMPTY;
        self.version.pre = Prerelease::EMPTY;
        self.oid = None;
        self
    }

    fn create_version_from_commit_history(
        &self,
        repository: &Repository,
    ) -> Result<Tag, BumpError> {
        let changelog_start_oid = match &self.package {
            None => repository.get_latest_tag_oid().ok(),
            Some(package) => repository
                .get_latest_package_tag(package)
                .ok()
                .and_then(|tag| tag.oid),
        }
        .unwrap_or_else(|| repository.get_first_commit().expect("non empty repository"));

        let changelog_start_oid = changelog_start_oid.to_string();
        let changelog_start_oid = Some(changelog_start_oid.as_str());

        let pattern = changelog_start_oid
            .map(|oid| format!("{}..", oid))
            .unwrap_or_else(|| "..".to_string());
        let pattern = pattern.as_str();
        let pattern = RevspecPattern::from(pattern);
        let commits = repository.get_commit_range(&pattern)?;

        let commits: Vec<&Git2Commit> = commits
            .commits
            .iter()
            .filter(|commit| !commit.message().unwrap_or("").starts_with("Merge "))
            .collect();

        VersionIncrement::display_history(&commits)?;

        let conventional_commits: Vec<Commit> = commits
            .iter()
            .map(|commit| Commit::from_git_commit(commit))
            .filter_map(Result::ok)
            .collect();

        let increment_type = self.version_increment_from_commit_history(&conventional_commits)?;

        Ok(match increment_type {
            VersionIncrement::Major => self.major_bump(),
            VersionIncrement::Minor => self.minor_bump(),
            VersionIncrement::Patch => self.patch_bump(),
            _ => unreachable!(),
        })
    }

    fn version_increment_from_commit_history(
        &self,
        commits: &[Commit],
    ) -> Result<VersionIncrement, BumpError> {
        let is_major_bump = || {
            self.version.major != 0
                && commits
                    .iter()
                    .any(|commit| commit.message.is_breaking_change)
        };

        let is_minor_bump = || {
            commits
                .iter()
                .any(|commit| commit.message.commit_type == CommitType::Feature)
        };

        let is_patch_bump = || {
            commits
                .iter()
                .any(|commit| commit.message.commit_type == CommitType::BugFix)
        };

        if is_major_bump() {
            Ok(VersionIncrement::Major)
        } else if is_minor_bump() {
            Ok(VersionIncrement::Minor)
        } else if is_patch_bump() {
            Ok(VersionIncrement::Patch)
        } else {
            Err(BumpError::NoCommitFound)
        }
    }
}

#[cfg(test)]
mod test {
    use crate::conventional::commit::Commit;
    use crate::conventional::version::VersionIncrement;
    use crate::git::repository::Repository;
    use crate::git::tag::Tag;
    use anyhow::Result;
    use chrono::Utc;
    use conventional_commit_parser::commit::{CommitType, ConventionalCommit};
    use sealed_test::prelude::*;
    use semver::Version;
    use speculoos::prelude::*;
    use std::str::FromStr;

    impl Commit {
        fn commit_fixture(commit_type: CommitType, is_breaking_change: bool) -> Self {
            Commit {
                oid: "1234".to_string(),
                message: ConventionalCommit {
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
        let tag = base_version.bump(VersionIncrement::Major, &repository)?;

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
        let tag = base_version.bump(VersionIncrement::Minor, &repository)?;

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
        let tag = base_version.bump(VersionIncrement::Patch, &repository)?;

        // Assert
        assert_that!(tag.version).is_equal_to(Version::new(1, 0, 1));
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
            .is_equal_to(VersionIncrement::Patch);

        Ok(())
    }

    #[test]
    fn increment_minor_version_should_set_patch_to_zero() -> Result<()> {
        // Arrange
        let repository = Repository::init(".")?;
        let version = Tag::from_str("1.1.1", None)?;

        // Act
        let tag = version.bump(VersionIncrement::Minor, &repository)?;

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
        let tag = version.bump(VersionIncrement::Major, &repository)?;

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
        let tag = version.bump(VersionIncrement::Patch, &repository)?;

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
        assert_that!(version)
            .is_ok()
            .is_equal_to(VersionIncrement::Major);

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
        assert_that!(version)
            .is_ok()
            .is_equal_to(VersionIncrement::Minor);

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
        assert_that!(version)
            .is_ok()
            .is_equal_to(VersionIncrement::Minor);

        Ok(())
    }

    #[test]
    fn should_fail_without_feature_bug_fix_or_breaking_change_commit() -> Result<()> {
        // Arrange
        let chore = Commit::commit_fixture(CommitType::Chore, false);
        let docs = Commit::commit_fixture(CommitType::Documentation, false);
        let base_version = Tag::from_str("0.1.0", None)?;

        // Act
        let version = base_version.version_increment_from_commit_history(&[chore, docs]);

        // Assert
        let result = version.unwrap_err().to_string();
        let result = result.as_str();

        assert_eq!(
            result,
            r#"failed to bump version

cause: No conventional commit found to bump current version.
    Only feature, bug fix and breaking change commits will trigger an automatic bump.

suggestion: Please see https://conventionalcommits.org/en/v1.0.0/#summary for more information.
    Alternatively consider using `cog bump <--version <VERSION>|--auto|--major|--minor>`

"#
        );

        Ok(())
    }
}
