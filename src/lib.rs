#![feature(drain_filter)]
#[macro_use]
extern crate anyhow;
#[macro_use]
extern crate serde_derive;
#[macro_use]
extern crate lazy_static;

pub mod changelog;
pub mod error;
pub mod filter;

pub mod commit;
pub mod repository;
pub mod settings;
pub mod version;

use crate::changelog::{Changelog, ChangelogWriter, WriterMode};
use crate::commit::{CommitConfig, CommitMessage, CommitType};
use crate::error::ErrorKind::Semver;
use crate::filter::CommitFilters;
use crate::repository::Repository;
use crate::settings::Settings;
use crate::version::VersionIncrement;
use anyhow::Result;
use chrono::Utc;
use colored::*;
use commit::Commit;
use git2::{Oid, RebaseOptions};
use semver::Version;
use std::collections::HashMap;
use std::fs::File;
use std::io::Write;
use std::path::{Path, PathBuf};
use std::process::{exit, Command, Stdio};
use tempdir::TempDir;

pub type CommitsMetadata = HashMap<CommitType, CommitConfig>;

lazy_static! {
    // This cannot be carried by `Cocogitto` struct since we need it to be available in `Changelog`,
    // `Commit` etc. Be ensure that `CocoGitto::new` is called before using this  in order to bypass
    // unwrapping in case of error.
    static ref COMMITS_METADATA: CommitsMetadata = {
        let repo = Repository::open(".").unwrap();
        Settings::get(&repo).unwrap().commit_types()
    };
}

pub fn init<S: AsRef<Path> + ?Sized>(path: &S) -> Result<()> {
    let path = path.as_ref();

    if !path.exists() {
        std::fs::create_dir(&path)
            .map_err(|err| anyhow!("Could not create directory `{:?}`: {}", path, err))?;
    }

    let mut is_init_commit = false;
    let repository = match Repository::open(&path) {
        Ok(repo) => {
            println!(
                "Found git repository in {:?}, skipping initialisation",
                &path
            );
            repo
        }
        Err(_) => match Repository::init(&path) {
            Ok(repo) => {
                println!("Empty git repository initialized in {:?}", &path);
                is_init_commit = true;
                repo
            }
            Err(err) => panic!("Unable to init repository on {:?}: {}", &path, err),
        },
    };

    let settings = Settings::default();
    let settings_path = path.join("coco.toml");
    if settings_path.exists() {
        eprint!("Found coco.toml in {:?}, Nothing to do", &path);
        exit(1);
    } else {
        std::fs::write(
            &settings_path,
            toml::to_string(&settings)
                .map_err(|err| anyhow!("Failed to serialize coco.toml : {}", err))?,
        )
        .map_err(|err| anyhow!("Could not write file `{:?}` : {}", &settings_path, err))?;
    }

    // TODO : add coco only"
    repository
        .add_all()
        .map_err(|err| anyhow!("Could not add file to repository index : {}", err))?;

    let message = if is_init_commit {
        "chore: init commit".to_string()
    } else {
        "chore: add cocogitto config".to_string()
    };

    repository.commit(message)?;
    Ok(())
}

pub struct CocoGitto {
    repository: Repository,
    changelog_path: PathBuf,
}

impl CocoGitto {
    pub fn get() -> Result<Self> {
        let repository = Repository::open(".")?;
        let settings = Settings::get(&repository).unwrap_or_default();
        let changelog_path = settings
            .changelog_path
            .unwrap_or_else(|| PathBuf::from("CHANGELOG.md"));

        Ok(CocoGitto {
            repository,
            changelog_path,
        })
    }

    pub fn get_commit_metadata() -> &'static CommitsMetadata {
        &COMMITS_METADATA
    }

    pub fn check_and_edit(&self) -> Result<()> {
        let from = self.repository.get_first_commit()?;
        let head = self.repository.get_head_commit_oid()?;
        let commits = self.repository.get_commit_range(from, head)?;
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
        let commits = self.repository.get_commit_range(from, to)?;
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
            errors.iter().for_each(|err| eprintln!("{}", err));
        }

        Ok(())
    }

    pub fn get_log(&self, filters: CommitFilters) -> Result<String> {
        let from = self.repository.get_first_commit()?;
        let to = self.repository.get_head_commit_oid()?;
        let commits = self.repository.get_commit_range(from, to)?;
        let logs = commits
            .iter()
            // Remove merge commits
            .filter(|commit| !commit.message().unwrap_or("").starts_with("Merge"))
            .filter(|commit| filters.filter_git2_commit(&commit))
            .map(|commit| Commit::from_git_commit(commit))
            // Apply filters
            .filter(|commit| match commit {
                Ok(commit) => filters.filters(commit),
                Err(_) => filters.no_error(),
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

    pub fn verify(&self, message: &str) -> Result<()> {
        let commit = Commit::parse_commit_message(message);

        match commit {
            Ok(message) => {
                println!(
                    "{}",
                    Commit {
                        shorthand: "not committed".to_string(),
                        message,
                        author: self.repository.get_author()?,
                        date: Utc::now().naive_utc(),
                    }
                );
                Ok(())
            }
            Err(err) => Err(err),
        }
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

    pub fn create_version(&self, increment: VersionIncrement, mode: WriterMode) -> Result<()> {
        let statuses = self.repository.get_statuses()?;

        if !statuses.is_empty() {
            let statuses = statuses
                .iter()
                .map(|status| format!("{} : {:?}\n", status.path().unwrap(), status.status()))
                .collect::<String>();
            return Err(anyhow!(
                "Repository contains unstaged change :\n{}",
                statuses
            ));
        }

        let current_tag = self
            .repository
            .get_latest_tag()
            .unwrap_or_else(|_| Version::new(0, 0, 0).to_string());

        let current_version = Version::parse(&current_tag)?;

        // Error on unstagged changes

        let next_version = increment.bump(&current_version)?;

        if next_version.le(&current_version) || next_version.eq(&current_version) {
            let comparison = format!("{} <= {}", current_version, next_version).red();
            let cause_key = "cause:".red();
            let cause = format!(
                "{} version MUST be greater than current one: {}",
                cause_key, comparison
            );
            return Err(anyhow!(Semver {
                level: "SemVer Error".red().to_string(),
                cause
            }));
        };

        let version_str = next_version.to_string();

        let origin = if current_tag.as_str() == "0.0.0" {
            self.repository.get_first_commit()?.to_string()
        } else {
            current_tag
        };

        let changelog = self.get_changelog(
            Some(&origin),
            Some(&self.repository.get_head_commit_oid()?.to_string()),
        )?;
        let mut writter = ChangelogWriter {
            changelog,
            path: self.changelog_path.clone(),
            mode,
        };

        writter.write()?;

        self.repository.add_all()?;
        self.repository.create_tag(&version_str)?;
        self.repository
            .commit(format!("chore(version): {}", next_version))?;

        let bump = format!("{} -> {}", current_version, next_version).green();
        println!("Bumped version : {}", bump);

        Ok(())
    }

    pub fn get_colored_changelog(&self, from: Option<&str>, to: Option<&str>) -> Result<String> {
        let mut changelog = self.get_changelog(from, to)?;
        Ok(changelog.markdown(true))
    }

    pub(crate) fn get_changelog(&self, from: Option<&str>, to: Option<&str>) -> Result<Changelog> {
        let from = self.resolve_from_arg(from)?;
        let to = self.resolve_to_arg(to)?;

        let mut commits = vec![];

        for commit in self
            .repository
            .get_commit_range(from, to)
            .map_err(|err| anyhow!("Could not get commit range {}...{} : {}", from, to, err))?
        {
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

        Ok(Changelog {
            from,
            to,
            date,
            commits,
        })
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
                .map_err(|err| anyhow!("Could not resolve from arg {} : {}", from, err))
        } else {
            self.repository
                .get_latest_tag_oid()
                .or_else(|_err| self.repository.get_first_commit())
        }
    }

    fn get_raw_oid_or_tag_oid(&self, input: &str) -> Result<Oid> {
        if let Ok(_version) = Version::parse(input) {
            self.repository
                .resolve_lightweight_tag(input)
                .map_err(|err| anyhow!("tag {} not found : {} ", input, err))
        } else {
            Oid::from_str(input).map_err(|err| anyhow!("`{}` is not a valid oid : {}", input, err))
        }
    }
}
