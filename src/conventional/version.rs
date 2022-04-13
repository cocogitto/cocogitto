use crate::conventional::commit::Commit;
use crate::git::repository::Repository;

use crate::conventional::error::BumpError;
use crate::git::revspec::RevspecPattern;
use colored::*;
use conventional_commit_parser::commit::CommitType;
use git2::Commit as Git2Commit;
use itertools::Itertools;
use log::info;
use semver::Version;

#[derive(Debug, PartialEq)]
pub enum VersionIncrement {
    Major,
    Minor,
    Patch,
    Auto,
    Manual(String),
}

impl VersionIncrement {
    pub(crate) fn bump(
        &self,
        current_version: &Version,
        repository: &Repository,
    ) -> Result<Version, BumpError> {
        match self {
            VersionIncrement::Manual(version) => Version::parse(version).map_err(Into::into),
            VersionIncrement::Auto => {
                VersionIncrement::create_version_from_commit_history(current_version, repository)
            }
            VersionIncrement::Major => Ok(Version::new(current_version.major + 1, 0, 0)),
            VersionIncrement::Patch => Ok(Version::new(
                current_version.major,
                current_version.minor,
                current_version.patch + 1,
            )),
            VersionIncrement::Minor => Ok(Version::new(
                current_version.major,
                current_version.minor + 1,
                0,
            )),
        }
    }

    fn create_version_from_commit_history(
        current_version: &Version,
        repository: &Repository,
    ) -> Result<Version, BumpError> {
        let changelog_start_oid = repository
            .get_latest_tag_oid()
            .unwrap_or_else(|_| repository.get_first_commit().unwrap());

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

        VersionIncrement::display_history(&commits);

        let conventional_commits: Vec<Commit> = commits
            .iter()
            .map(|commit| Commit::from_git_commit(commit))
            .filter_map(Result::ok)
            .collect();

        let increment_type = VersionIncrement::version_increment_from_commit_history(
            current_version,
            &conventional_commits,
        )?;

        increment_type.bump(current_version, repository)
    }

    fn version_increment_from_commit_history(
        current_version: &Version,
        commits: &[Commit],
    ) -> Result<VersionIncrement, BumpError> {
        let is_major_bump = || {
            current_version.major != 0
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

    fn display_history(commits: &[&Git2Commit]) {
        let conventional_commits: Vec<Result<_, _>> = commits
            .iter()
            .map(|commit| Commit::from_git_commit(commit))
            .collect();

        // Commits which type are neither feat, fix nor breaking changes
        // won't affect the version number.
        let mut non_bump_commits: Vec<&CommitType> = conventional_commits
            .iter()
            .filter_map(|commit| match commit {
                Ok(commit) => match commit.message.commit_type {
                    CommitType::Feature | CommitType::BugFix => None,
                    _ => Some(&commit.message.commit_type),
                },
                Err(_) => None,
            })
            .collect();

        non_bump_commits.sort();

        let non_bump_commits: Vec<(usize, &CommitType)> = non_bump_commits
            .into_iter()
            .dedup_by_with_count(|c1, c2| c1 == c2)
            .collect();

        let mut skip_message = "Skipping irrelevant commits:\n".to_string();
        for (count, commit_type) in non_bump_commits {
            skip_message.push_str(&format!("\t- {}: {}\n", commit_type.as_ref(), count))
        }

        info!("{}", skip_message);

        let bump_commits = conventional_commits
            .iter()
            .filter_map(|commit| match commit {
                Ok(commit) => match commit.message.commit_type {
                    CommitType::Feature | CommitType::BugFix => Some(Ok(commit)),
                    _ => None,
                },
                Err(err) => Some(Err(err)),
            });

        for commit in bump_commits {
            match commit {
                Ok(commit) if commit.message.is_breaking_change => {
                    info!(
                        "Found {} commit {} with type: {}",
                        "BREAKING CHANGE".red(),
                        commit.shorthand().blue(),
                        commit.message.commit_type.as_ref().yellow()
                    )
                }
                Ok(commit) if commit.message.commit_type == CommitType::BugFix => {
                    info!("Found bug fix commit {}", commit.shorthand().blue())
                }
                Ok(commit) if commit.message.commit_type == CommitType::Feature => {
                    info!("Found feature commit {}", commit.shorthand().blue())
                }
                _ => (),
            }
        }
    }
}

#[cfg(test)]
// Auto version tests resides in test/ dir since it rely on git log
// To generate the version
mod test {
    use std::str::FromStr;

    use crate::conventional::commit::Commit;
    use crate::conventional::version::VersionIncrement;

    use crate::Repository;
    use anyhow::Result;
    use chrono::Utc;
    use conventional_commit_parser::commit::{CommitType, ConventionalCommit};
    use pretty_assertions::assert_eq;
    use sealed_test::prelude::*;
    use semver::Version;
    use speculoos::prelude::*;

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
        let base_version = Version::new(1, 0, 0);

        // Act
        let version = VersionIncrement::Major.bump(&base_version, &repository)?;

        // Assert
        assert_that!(version).is_equal_to(Version::new(2, 0, 0));
        Ok(())
    }

    #[sealed_test]
    fn minor_bump() -> Result<()> {
        // Arrange
        let repository = Repository::init(".")?;

        // Act
        let base_version = Version::new(1, 0, 0);
        let version = VersionIncrement::Minor.bump(&base_version, &repository)?;

        // Assert
        assert_that!(version).is_equal_to(Version::new(1, 1, 0));
        Ok(())
    }

    #[sealed_test]
    fn patch_bump() -> Result<()> {
        // Arrange
        let repository = Repository::init(".")?;
        let base_version = Version::new(1, 0, 0);

        // Act
        let version = VersionIncrement::Patch.bump(&base_version, &repository)?;

        // Assert
        assert_that!(version).is_equal_to(Version::new(1, 0, 1));
        Ok(())
    }

    #[test]
    fn increment_minor_version_should_set_patch_to_zero() -> Result<()> {
        // Arrange
        let repository = Repository::init(".")?;
        let version = Version::from_str("1.1.1")?;

        // Act
        let bumped = VersionIncrement::Minor.bump(&version, &repository);

        // Assert
        assert_that!(bumped)
            .is_ok()
            .is_equal_to(Version::from_str("1.2.0")?);

        Ok(())
    }

    #[sealed_test]
    fn increment_major_version_should_set_minor_and_patch_to_zero() -> Result<()> {
        // Arrange
        let repository = Repository::init(".")?;
        let version = Version::from_str("1.1.1")?;

        // Act
        let bumped = VersionIncrement::Major.bump(&version, &repository);

        // Assert
        assert_that!(bumped)
            .is_ok()
            .is_equal_to(Version::from_str("2.0.0")?);

        Ok(())
    }

    #[sealed_test]
    fn increment_should_strip_metadata() -> Result<()> {
        // Arrange
        let repository = Repository::init(".")?;
        let version = Version::from_str("1.1.1-pre+10.1")?;

        // Act
        let bumped = VersionIncrement::Patch.bump(&version, &repository);

        // Assert
        assert_that!(bumped)
            .is_ok()
            .is_equal_to(Version::from_str("1.1.2")?);

        Ok(())
    }

    #[test]
    fn should_get_next_auto_version_patch() -> Result<()> {
        // Arrange
        let patch = Commit::commit_fixture(CommitType::BugFix, false);

        // Act
        let version = VersionIncrement::version_increment_from_commit_history(
            &Version::parse("1.0.0")?,
            &[patch],
        );

        // Assert
        assert_that!(version)
            .is_ok()
            .is_equal_to(VersionIncrement::Patch);

        Ok(())
    }

    #[test]
    fn should_get_next_auto_version_breaking_changes() -> Result<()> {
        // Arrange
        let feature = Commit::commit_fixture(CommitType::Feature, false);
        let breaking_change = Commit::commit_fixture(CommitType::Feature, true);

        // Act
        let version = VersionIncrement::version_increment_from_commit_history(
            &Version::parse("1.0.0")?,
            &[breaking_change, feature],
        );

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

        // Act
        let version = VersionIncrement::version_increment_from_commit_history(
            &Version::parse("0.1.0")?,
            &[breaking_change, feature],
        );

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

        // Act
        let version = VersionIncrement::version_increment_from_commit_history(
            &Version::parse("1.0.0")?,
            &[patch, feature],
        );

        // Assert
        assert_that!(version)
            .is_ok()
            .is_equal_to(VersionIncrement::Minor);

        Ok(())
    }

    #[test]
    fn should_fail_without_feature_bug_fix_or_breaking_change_commit() -> Result<()> {
        // Arrange
        let patch = Commit::commit_fixture(CommitType::Chore, false);
        let feature = Commit::commit_fixture(CommitType::Documentation, false);

        // Act
        let version = VersionIncrement::version_increment_from_commit_history(
            &Version::parse("1.0.0")?,
            &[patch, feature],
        );

        let result = version.unwrap_err().to_string();
        let result = result.as_str();

        // Assert
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
