use crate::conventional::commit::Commit;
use crate::git::repository::Repository;
use anyhow::Result;
use colored::*;
use conventional_commit_parser::commit::CommitType;
use git2::Commit as Git2Commit;
use semver::{Identifier, Version};

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
                next.increment_major();
                Ok(next)
            }
            VersionIncrement::Patch => {
                let mut next = current_version.clone();
                next.increment_patch();
                Ok(next)
            }
            VersionIncrement::Minor => {
                let mut next = current_version.clone();
                next.increment_minor();
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
            next_version.increment_major();
        } else if is_minor_bump() {
            next_version.increment_minor();
        } else if is_patch_bump() {
            next_version.increment_patch();
        } else {
            bail!("No commit found to bump current version");
        }

        Ok(next_version)
    }

    fn display_history(commits: &[&Git2Commit]) {
        for commit in commits {
            let commit = Commit::from_git_commit(commit);
            // TODO: prompt for continue on err
            if let Err(err) = commit {
                eprintln!("{}", err);
            } else {
                let commit = commit.unwrap();
                match (
                    &commit.message.commit_type,
                    commit.message.is_breaking_change,
                ) {
                    (CommitType::Feature, false) => {
                        println!("Found feature commit {}", commit.shorthand().blue(),)
                    }
                    (CommitType::BugFix, false) => {
                        println!("Found bug fix commit {}", commit.shorthand().blue(),)
                    }
                    (commit_type, true) => println!(
                        "Found {} commit {} with type : {}",
                        "BREAKING CHANGE".red(),
                        commit.shorthand().blue(),
                        commit_type.as_ref().yellow()
                    ),
                    (_, false) => println!(
                        "Skipping irrelevant commit {} with type : {}",
                        commit.shorthand().blue(),
                        commit.message.commit_type.as_ref().yellow()
                    ),
                }
            }
        }
    }
}

pub fn parse_pre_release(string: &str) -> Result<Vec<Identifier>> {
    ensure!(
        string
            .chars()
            .all(|c| c.is_ascii_alphanumeric() || ['.', '-'].contains(&c)),
        "Pre-release string must be a dot-separated list of identifiers comprised of ASCII alphanumerics and hyphens [0-9A-Za-z-]"
    );

    ensure!(
        !string.starts_with('.') && !string.contains("..") && !string.ends_with('.'),
        "Dot-separated identifiers in the pre-release string must not be empty"
    );

    let idents = string
        .split('.')
        .map(|s| match s.parse::<u64>() {
            Ok(n) => Identifier::Numeric(n),
            Err(_) => Identifier::AlphaNumeric(s.to_string()),
        })
        .collect();

    Ok(idents)
}

#[cfg(test)]
mod test {
    use crate::conventional::commit::Commit;
    use crate::conventional::version::{parse_pre_release, VersionIncrement};
    use anyhow::Result;
    use chrono::Utc;
    use conventional_commit_parser::commit::{CommitType, ConventionalCommit};
    use semver::{Identifier, Version};
    use speculoos::prelude::*;

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
    fn parse_pre_release_valid() -> Result<()> {
        let idents = parse_pre_release("alpha.0-dev.1")?;
        assert_that!(idents).is_equal_to(&vec![
            Identifier::AlphaNumeric("alpha".into()),
            Identifier::AlphaNumeric("0-dev".into()),
            Identifier::Numeric(1),
        ]);
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

    #[test]
    fn parse_pre_release_non_ascii() {
        assert_that!(parse_pre_release("РАСТ")).is_err();
    }

    #[test]
    fn parse_pre_release_illegal_ascii() {
        assert_that!(parse_pre_release("alpha$5")).is_err();
    }

    #[test]
    fn parse_pre_release_empty_ident() {
        assert_that!(parse_pre_release(".alpha.5")).is_err();
        assert_that!(parse_pre_release("alpha..5")).is_err();
        assert_that!(parse_pre_release("alpha.5.")).is_err();
    }
}
