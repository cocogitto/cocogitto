use crate::commit::{Commit, CommitType};
use crate::repository::Repository;
use anyhow::Result;
use colored::*;
use git2::Commit as Git2Commit;
use semver::Version;

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
                Version::parse(&version).map_err(|err| anyhow!(err))
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
        let mut next_version = current_version.clone();
        let repository = Repository::open()?;
        let changelog_start_oid = repository
            .get_latest_tag_oid()
            .unwrap_or_else(|_| repository.get_first_commit().unwrap());

        let head = repository.get_head_commit_oid()?;

        let commits = repository.get_commit_range(changelog_start_oid, head)?;

        let commits: Vec<&Git2Commit> = commits
            .iter()
            .filter(|commit| !commit.message().unwrap_or("").starts_with("Merge "))
            .collect();

        for commit in commits {
            let commit = Commit::from_git_commit(&commit);

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
                        next_version.increment_minor();
                        println!(
                            "Found feature commit {}, bumping to {}",
                            commit.shorthand.blue(),
                            next_version.to_string().green()
                        )
                    }
                    (CommitType::BugFix, false) => {
                        next_version.increment_patch();
                        println!(
                            "Found bug fix commit {}, bumping to {}",
                            commit.shorthand.blue(),
                            next_version.to_string().green()
                        )
                    }
                    (commit_type, true) => {
                        next_version.increment_major();
                        println!(
                            "Found {} commit {} with type : {}",
                            "BREAKING CHANGE".red(),
                            commit.shorthand.blue(),
                            commit_type.get_key_str().yellow()
                        )
                    }
                    (_, false) => println!(
                        "Skipping irrelevant commit {} with type : {}",
                        commit.shorthand.blue(),
                        commit.message.commit_type.get_key_str().yellow()
                    ),
                }
            }
        }

        Ok(next_version)
    }
}
