#![feature(drain_filter)]
#[macro_use]
extern crate anyhow;
#[macro_use]
extern crate serde_derive;

mod changelog;
pub mod error;
mod hook;

pub mod commit;
pub mod repository;
pub mod settings;

use crate::changelog::Changelog;
use crate::commit::{CommitMessage, CommitType};
use crate::error::CocoGittoError::SemverError;
use crate::repository::Repository;
use crate::settings::Settings;
use anyhow::Result;
use chrono::Utc;
use colored::*;
use commit::Commit;
use git2::{Commit as Git2Commit, Oid, RebaseOptions, Repository as Git2Repository};
use semver::Version;
use std::fs::File;
use std::io::Write;
use std::process::{Command, Stdio};
use tempdir::TempDir;

pub struct CocoGitto {
    pub settings: Settings,
    repository: Repository,
}

pub enum VersionIncrement {
    Major,
    Minor,
    Patch,
    Auto,
    Manual(String),
}

#[derive(Eq, PartialEq)]
pub enum CommitFilter {
    Type(CommitType),
    Scope(String),
    Author(String),
    BreakingChange,
}

pub struct CommitFilters(pub Vec<CommitFilter>);

impl CommitFilters {
    pub fn filters(&self, commit: &Commit) -> bool {
        // Commit type filters
        let types = self
            .0
            .iter()
            .filter_map(|filter| match filter {
                CommitFilter::Type(commit_type) => Some(commit_type),
                _ => None,
            })
            .collect::<Vec<&CommitType>>();

        let filter_type = if types.is_empty() {
            true
        } else {
            types
                .iter()
                .any(|commit_type| **commit_type == commit.message.commit_type)
        };

        // Scope filters
        let scopes = self
            .0
            .iter()
            .filter_map(|filter| match filter {
                CommitFilter::Scope(scope) => Some(scope),
                _ => None,
            })
            .collect::<Vec<&String>>();

        let filter_scopes = if scopes.is_empty() {
            true
        } else {
            scopes
                .iter()
                .any(|scope| Some(*scope) == commit.message.scope.as_ref())
        };

        // Author filters
        let authors = self
            .0
            .iter()
            .filter_map(|filter| match filter {
                CommitFilter::Author(author) => Some(author),
                _ => None,
            })
            .collect::<Vec<&String>>();

        let filter_authors = if authors.is_empty() {
            true
        } else {
            authors.iter().any(|author| *author == &commit.author)
        };

        // Breaking changes filters
        let filter_breaking_changes = if self.0.contains(&CommitFilter::BreakingChange) {
            commit.message.is_breaking_change
        } else {
            true
        };

        filter_type && filter_authors && filter_scopes && filter_breaking_changes
    }
}

impl CocoGitto {
    pub fn get() -> Result<Self> {
        let repository = Repository::open()?;
        let settings = Settings::get(&repository)?;

        Ok(CocoGitto {
            settings,
            repository,
        })
    }

    pub fn commit_types(&self) -> Vec<String> {
        let mut commit_types: Vec<String> = self
            .settings
            .commit_types
            .iter()
            .map(|(key, _)| key)
            .cloned()
            .collect();

        commit_types.extend_from_slice(&[
            "feat".to_string(),
            "fix".to_string(),
            "chore".to_string(),
            "revert".to_string(),
            "perf".to_string(),
            "docs".to_string(),
            "style".to_string(),
            "refactor".to_string(),
            "test".to_string(),
            "build".to_string(),
            "ci".to_string(),
        ]);

        commit_types
    }

    pub fn get_repository(&self) -> &Git2Repository {
        &self.repository.0
    }

    pub fn check_and_edit(&self) -> Result<()> {
        let from = self.repository.get_first_commit()?;
        let head = self.repository.get_head_commit_oid()?;
        let commits = self.get_commit_range(from, head)?;
        let editor = std::env::var("EDITOR")?;
        let dir = TempDir::new("cocogito")?;

        let errored_commits: Vec<Oid> = commits
            .iter()
            .map(|commit| {
                let conv_commit = Commit::from_git_commit(&commit);
                (commit.id(), conv_commit)
            })
            .filter(|commit| commit.1.is_err())
            .map(|commit| commit.0)
            .collect();

        let last_errored_commit = errored_commits.last();
        println!("{:?}", last_errored_commit);
        let commit = self
            .repository
            .0
            .find_commit(last_errored_commit.unwrap().to_owned())?;
        let rebase_start = commit.parent_id(0)?;
        let commit = self.repository.0.find_annotated_commit(rebase_start)?;
        let mut options = RebaseOptions::new();
        let mut rebase = self
            .repository
            .0
            .rebase(None, Some(&commit), None, Some(&mut options))?;

        while let Some(op) = rebase.next() {
            if let Ok(rebase_operation) = op {
                let oid = rebase_operation.id();
                let original_commit = self.repository.0.find_commit(oid)?;
                println!("rebasing {}", oid);
                if errored_commits.contains(&oid) {
                    println!("\tmatch found in errored commits");
                    let file_path = dir.path().join(&commit.id().to_string());
                    let mut file = File::create(&file_path)?;
                    file.write_all(original_commit.message_bytes())?;

                    Command::new(&editor)
                        .arg(&file_path)
                        .stdout(Stdio::inherit())
                        .stdin(Stdio::inherit())
                        .stderr(Stdio::inherit())
                        .output()?;

                    let new_message = std::fs::read_to_string(&file_path)?;
                    rebase.commit(None, &original_commit.committer(), Some(&new_message))?;
                } else {
                    rebase.commit(None, &original_commit.committer(), None)?;
                }
            } else {
                eprintln!("{:?}", op);
            }
        }

        rebase.finish(None)?;
        Ok(())
    }

    pub fn check(&self) -> Result<()> {
        let from = self.repository.get_first_commit()?;
        let to = self.repository.get_head_commit_oid()?;
        let commits = self.get_commit_range(from, to)?;
        let errors: Vec<anyhow::Error> = commits
            .iter()
            .filter(|commit| !commit.message().unwrap_or("").starts_with("Merge "))
            .map(|commit| Commit::from_git_commit(commit))
            .filter(|commit| commit.is_err())
            .map(|err| err.unwrap_err())
            .collect();

        if errors.is_empty() {
            let msg = "No errored commits".green();
            println!("{}", msg)
        } else {
            errors.iter().for_each(|err| eprintln!("{}", err))
        }

        Ok(())
    }

    pub fn get_log(&self, filters: CommitFilters) -> Result<String> {
        let from = self.repository.get_first_commit()?;
        let to = self.repository.get_head_commit_oid()?;
        let commits = self.get_commit_range(from, to)?;
        let logs = commits
            .iter()
            // Remove merge commits
            .filter(|commit| !commit.message().unwrap_or("").starts_with("Merge"))
            .map(|commit| Commit::from_git_commit(commit))
            // Remove errors if we have filters
            .filter(|commit| {
                if !filters.0.is_empty() {
                    commit.is_ok()
                } else {
                    true
                }
            })
            // Apply filters
            .filter(|commit| match commit {
                Ok(commit) => filters.filters(commit),
                Err(_) => false,
            })
            // Format
            .map(|commit| match commit {
                Ok(commit) => commit.get_log(),
                Err(err) => err.to_string(),
            })
            .collect::<Vec<String>>()
            .join("\n");

        Ok(logs)
    }

    pub fn verify(message: &str) -> Result<()> {
        Commit::parse_commit_message(message).map(|commit_message| {
            println!(
                "{}",
                Commit {
                    shorthand: "".to_string(),
                    message: commit_message,
                    author: "".to_string(),
                    date: Utc::now().naive_utc(),
                }
            )
        })
    }

    pub fn conventional_commit(
        &self,
        commit_type: &str,
        scope: Option<String>,
        description: String,
        body: Option<String>,
        footer: Option<String>,
        is_breaking_change: bool,
    ) -> Result<()> {
        let commit_type = CommitType::from(commit_type);

        let message = CommitMessage {
            commit_type,
            scope,
            body,
            footer,
            description,
            is_breaking_change,
        }
        .to_string();

        let oid = self.repository.commit(message)?;
        let commit = self.repository.0.find_commit(oid)?;
        let commit = Commit::from_git_commit(&commit)?;
        println!("{}", commit);

        Ok(())
    }

    pub fn create_version(&self, increment: VersionIncrement) -> Result<()> {
        let tag = self
            .repository
            .get_latest_tag()
            .unwrap_or_else(|_| Version::new(0, 0, 0).to_string());

        let current_version = Version::parse(&tag)?;

        let next_version = match increment {
            VersionIncrement::Manual(version) => Version::parse(&version)?,
            VersionIncrement::Auto => self.auto_bump_version(&current_version)?,
            VersionIncrement::Major => {
                let mut next = current_version.clone();
                next.increment_major();
                next
            }
            VersionIncrement::Patch => {
                let mut next = current_version.clone();
                next.increment_patch();
                next
            }
            VersionIncrement::Minor => {
                let mut next = current_version.clone();
                next.increment_minor();
                next
            }
        };

        if next_version.le(&current_version) || next_version.eq(&current_version) {
            let comparison = format!("{} <= {}", current_version, next_version).red();
            let cause_key = "cause:".red();
            let cause = format!(
                "{} version MUST be greater than current one: {}",
                cause_key, comparison
            );
            return Err(anyhow!(SemverError {
                level: "SemVer Error".red().to_string(),
                cause
            }));
        };

        self.repository.create_tag(&next_version.to_string())?;
        let bump = format!("{} -> {}", current_version, next_version).green();
        println!("Bumped version : {}", bump);

        Ok(())
    }

    pub fn get_changelog(&self, from: Option<&str>, to: Option<&str>) -> anyhow::Result<String> {
        let from = self.resolve_from_arg(from)?;
        let to = self.resolve_to_arg(to)?;

        let mut commits = vec![];

        for commit in self.get_commit_range(from, to)? {
            // We skip the origin commit (ex: from 0.1.0 to 1.0.0)
            if commit.id() == from {
                break;
            }

            // Ignore merge commits
            if let Some(message) = commit.message() {
                if message.starts_with("Merge") {
                    continue;
                }
            }

            match Commit::from_git_commit(&commit) {
                Ok(commit) => commits.push(commit),
                Err(err) => {
                    let err = format!("{}", err).red();
                    eprintln!("{}", err);
                }
            };
        }

        let date = Utc::now().naive_utc().date().to_string();

        let mut changelog = Changelog {
            from,
            to,
            date,
            commits,
        };

        Ok(changelog.tag_diff_to_markdown())
    }

    fn auto_bump_version(&self, current_version: &Version) -> Result<Version> {
        let mut next_version = current_version.clone();

        let changelog_start_oid = self
            .repository
            .get_latest_tag_oid()
            .unwrap_or_else(|_| self.repository.get_first_commit().unwrap());

        let head = self.repository.get_head_commit_oid()?;

        let commits = self.get_commit_range(changelog_start_oid, head)?;

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

    // TODO : revparse
    fn resolve_to_arg(&self, to: Option<&str>) -> Result<Oid> {
        if let Some(to) = to {
            self.get_raw_oid_or_tag_oid(to)
        } else {
            self.repository
                .get_head_commit_oid()
                .or_else(|_err| self.repository.get_first_commit())
        }
    }

    // TODO : revparse
    fn resolve_from_arg(&self, from: Option<&str>) -> Result<Oid> {
        if let Some(from) = from {
            self.get_raw_oid_or_tag_oid(from)
        } else {
            self.repository
                .get_latest_tag_oid()
                .or_else(|_err| self.repository.get_first_commit())
        }
    }

    fn get_raw_oid_or_tag_oid(&self, input: &str) -> Result<Oid> {
        if let Ok(_version) = Version::parse(input) {
            self.repository.resolve_lightweight_tag(input)
        } else {
            Oid::from_str(input).map_err(|err| anyhow!(err))
        }
    }

    fn get_commit_range(&self, from: Oid, to: Oid) -> Result<Vec<Git2Commit>> {
        // Ensure commit exists
        let repository = self.get_repository();
        repository.find_commit(from)?;
        repository.find_commit(to)?;

        let mut revwalk = repository.revwalk()?;
        revwalk.push(to)?;
        revwalk.push(from)?;

        let mut commits: Vec<Git2Commit> = vec![];

        for oid in revwalk {
            let oid = oid?;
            let commit = repository.find_commit(oid)?;
            commits.push(commit);
        }

        Ok(commits)
    }
}

#[cfg(test)]
mod test {
    #[test]
    fn should_open_repo() {}
}
