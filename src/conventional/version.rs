use crate::conventional::commit::Commit;
use crate::git::repository::Repository;

use anyhow::{anyhow, bail, Result};
use colored::*;
use conventional_commit_parser::commit::CommitType;
use git2::Commit as Git2Commit;
use itertools::Itertools;
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
    pub(crate) fn bump(&self, current_version: &Version) -> Result<Version> {
        match self {
            VersionIncrement::Manual(version) => {
                Version::parse(version).map_err(|err| anyhow!(err))
            }
            VersionIncrement::Auto => self.create_version_from_commit_history(current_version),
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

    fn create_version_from_commit_history(&self, current_version: &Version) -> Result<Version> {
        let repository = Repository::open(".")?;
        let changelog_start_oid = repository
            .get_latest_tag_oid()
            .unwrap_or_else(|_| repository.get_first_commit().unwrap());

        let head = repository.get_head_commit_oid()?;

        let commits = repository.get_commit_range(changelog_start_oid, head)?;

        let commits: Vec<&Git2Commit> = commits
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

        increment_type.bump(current_version)
    }

    fn version_increment_from_commit_history(
        current_version: &Version,
        commits: &[Commit],
    ) -> Result<VersionIncrement> {
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
            bail!("No commit found to bump current version")
        }
    }

    fn display_history(commits: &[&Git2Commit]) {
        let conventional_commits: Vec<Result<Commit, anyhow::Error>> = commits
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

        println!("{}", skip_message);

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
                    println!(
                        "Found {} commit {} with type: {}",
                        "BREAKING CHANGE".red(),
                        commit.shorthand().blue(),
                        commit.message.commit_type.as_ref().yellow()
                    )
                }
                Ok(commit) if commit.message.commit_type == CommitType::BugFix => {
                    println!("Found bug fix commit {}", commit.shorthand().blue())
                }
                Ok(commit) if commit.message.commit_type == CommitType::Feature => {
                    println!("Found feature commit {}", commit.shorthand().blue())
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

    use anyhow::Result;
    use chrono::Utc;
    use conventional_commit_parser::commit::{CommitType, ConventionalCommit};
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

    #[test]
    fn major_bump() -> Result<()> {
        let version = VersionIncrement::Major.bump(&Version::new(1, 0, 0))?;
        assert_that!(version).is_equal_to(Version::new(2, 0, 0));
        Ok(())
    }

    #[test]
    fn minor_bump() -> Result<()> {
        let version = VersionIncrement::Minor.bump(&Version::new(1, 0, 0))?;
        assert_that!(version).is_equal_to(Version::new(1, 1, 0));
        Ok(())
    }

    #[test]
    fn patch_bump() -> Result<()> {
        let version = VersionIncrement::Patch.bump(&Version::new(1, 0, 0))?;
        assert_that!(version).is_equal_to(Version::new(1, 0, 1));
        Ok(())
    }

    #[test]
    fn increment_minor_version_should_set_patch_to_zero() {
        let version = Version::from_str("1.1.1").unwrap();

        let bumped = VersionIncrement::Minor.bump(&version).unwrap();

        assert_that!(bumped).is_equal_to(Version::from_str("1.2.0").unwrap())
    }

    #[test]
    fn increment_major_version_should_set_minor_and_patch_to_zero() {
        let version = Version::from_str("1.1.1").unwrap();

        let bumped = VersionIncrement::Major.bump(&version).unwrap();

        assert_that!(bumped).is_equal_to(Version::from_str("2.0.0").unwrap())
    }

    #[test]
    fn increment_should_strip_metadata() {
        let version = Version::from_str("1.1.1-pre+10.1").unwrap();

        let bumped = VersionIncrement::Patch.bump(&version).unwrap();

        assert_that!(bumped).is_equal_to(Version::from_str("1.1.2").unwrap())
    }

    #[test]
    fn should_get_next_auto_version_patch() {
        let patch = Commit::commit_fixture(CommitType::BugFix, false);

        let version = VersionIncrement::version_increment_from_commit_history(
            &Version::parse("1.0.0").unwrap(),
            &[patch],
        );

        assert_that!(version)
            .is_ok()
            .is_equal_to(VersionIncrement::Patch)
    }

    #[test]
    fn should_get_next_auto_version_breaking_changes() {
        let feature = Commit::commit_fixture(CommitType::Feature, false);
        let breaking_change = Commit::commit_fixture(CommitType::Feature, true);

        let version = VersionIncrement::version_increment_from_commit_history(
            &Version::parse("1.0.0").unwrap(),
            &[breaking_change, feature],
        );

        assert_that!(version)
            .is_ok()
            .is_equal_to(VersionIncrement::Major)
    }

    #[test]
    fn should_get_next_auto_version_breaking_changes_on_initial_dev_version() {
        let feature = Commit::commit_fixture(CommitType::Feature, false);
        let breaking_change = Commit::commit_fixture(CommitType::Feature, true);

        let version = VersionIncrement::version_increment_from_commit_history(
            &Version::parse("0.1.0").unwrap(),
            &[breaking_change, feature],
        );

        assert_that!(version)
            .is_ok()
            .is_equal_to(VersionIncrement::Minor)
    }

    #[test]
    fn should_get_next_auto_version_minor() {
        let patch = Commit::commit_fixture(CommitType::BugFix, false);
        let feature = Commit::commit_fixture(CommitType::Feature, false);

        let version = VersionIncrement::version_increment_from_commit_history(
            &Version::parse("1.0.0").unwrap(),
            &[patch, feature],
        );

        assert_that!(version)
            .is_ok()
            .is_equal_to(VersionIncrement::Minor)
    }
}
