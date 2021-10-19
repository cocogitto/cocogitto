use anyhow::Result;
use colored::*;
use conventional_commit_parser::commit::CommitType;
use git2::Commit as Git2Commit;
use itertools::Itertools;
use semver::Version;

use crate::conventional::commit::Commit;
use crate::git::repository::Repository;

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
            VersionIncrement::Auto => self.get_auto_version(current_version),
            VersionIncrement::Major => {
                let mut next = current_version.clone();
                next.major += 1;
                Ok(next)
            }
            VersionIncrement::Patch => {
                let mut next = current_version.clone();
                next.patch += 1;
                Ok(next)
            }
            VersionIncrement::Minor => {
                let mut next = current_version.clone();
                next.minor += 1;
                Ok(next)
            }
        }
    }

    fn get_auto_version(&self, current_version: &Version) -> Result<Version> {
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

        VersionIncrement::get_next_auto_version(current_version, &conventional_commits)
    }

    fn get_next_auto_version(current_version: &Version, commits: &[Commit]) -> Result<Version> {
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

        let mut next_version = current_version.clone();
        if is_major_bump() {
            next_version.major += 1;
        } else if is_minor_bump() {
            next_version.minor += 1;
        } else if is_patch_bump() {
            next_version.patch += 1;
        } else {
            bail!("No commit found to bump current version");
        }

        Ok(next_version)
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
mod test {
    use anyhow::Result;
    use chrono::Utc;
    use conventional_commit_parser::commit::{CommitType, ConventionalCommit};
    use semver::Version;
    use speculoos::prelude::*;

    use crate::conventional::commit::Commit;
    use crate::conventional::version::VersionIncrement;

    // Auto version tests resides in test/ dir since it rely on git log
    // To generate the version

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
    fn should_get_next_auto_version_patch() {
        let patch = Commit {
            oid: "1234".to_string(),
            message: ConventionalCommit {
                commit_type: CommitType::BugFix,
                scope: None,
                summary: "fix".to_string(),
                body: None,
                is_breaking_change: false,
                footers: vec![],
            },
            author: "".to_string(),
            date: Utc::now().naive_local(),
        };

        let version =
            VersionIncrement::get_next_auto_version(&Version::parse("1.0.0").unwrap(), &[patch]);

        assert_that!(version)
            .is_ok()
            .is_equal_to(Version::new(1, 0, 1))
    }

    #[test]
    fn should_get_next_auto_version_breaking_changes() {
        let feature = Commit {
            oid: "1234".to_string(),
            message: ConventionalCommit {
                commit_type: CommitType::Feature,
                scope: None,
                body: None,
                summary: "feature".to_string(),
                is_breaking_change: false,
                footers: vec![],
            },
            author: "".to_string(),
            date: Utc::now().naive_local(),
        };

        let breaking_change = Commit {
            oid: "1234".to_string(),
            message: ConventionalCommit {
                commit_type: CommitType::Feature,
                scope: None,
                body: None,
                summary: "feature".to_string(),
                is_breaking_change: true,
                footers: vec![],
            },
            author: "".to_string(),
            date: Utc::now().naive_local(),
        };

        let version = VersionIncrement::get_next_auto_version(
            &Version::parse("1.0.0").unwrap(),
            &[breaking_change, feature],
        );

        assert_that!(version)
            .is_ok()
            .is_equal_to(Version::new(2, 0, 0))
    }

    #[test]
    fn should_get_next_auto_version_breaking_changes_on_initial_dev_version() {
        let feature = Commit {
            oid: "1234".to_string(),
            message: ConventionalCommit {
                commit_type: CommitType::Feature,
                scope: None,
                body: None,
                summary: "feature".to_string(),
                is_breaking_change: false,
                footers: vec![],
            },
            author: "".to_string(),
            date: Utc::now().naive_local(),
        };

        let breaking_change = Commit {
            oid: "1234".to_string(),
            message: ConventionalCommit {
                commit_type: CommitType::Feature,
                scope: None,
                body: None,
                summary: "feature".to_string(),
                is_breaking_change: true,
                footers: vec![],
            },
            author: "".to_string(),
            date: Utc::now().naive_local(),
        };

        let version = VersionIncrement::get_next_auto_version(
            &Version::parse("0.1.0").unwrap(),
            &[breaking_change, feature],
        );

        assert_that!(version)
            .is_ok()
            .is_equal_to(Version::new(0, 2, 0))
    }

    #[test]
    fn should_get_next_auto_version_minor() {
        let patch = Commit {
            oid: "1234".to_string(),
            message: ConventionalCommit {
                commit_type: CommitType::BugFix,
                scope: None,
                body: None,
                summary: "fix".to_string(),
                is_breaking_change: false,
                footers: vec![],
            },
            author: "".to_string(),
            date: Utc::now().naive_local(),
        };

        let feature = Commit {
            oid: "1234".to_string(),
            message: ConventionalCommit {
                commit_type: CommitType::Feature,
                scope: None,
                body: None,
                summary: "feature".to_string(),
                is_breaking_change: false,
                footers: vec![],
            },
            author: "".to_string(),
            date: Utc::now().naive_local(),
        };

        let version = VersionIncrement::get_next_auto_version(
            &Version::parse("1.0.0").unwrap(),
            &[patch, feature],
        );

        assert_that!(version)
            .is_ok()
            .is_equal_to(Version::new(1, 1, 0))
    }
}
