#[macro_use]
extern crate anyhow;
#[macro_use]
extern crate serde_derive;
#[macro_use]
extern crate lazy_static;

pub mod error;

pub mod conventional;
pub mod git;
pub mod hook;
pub mod log;
pub mod settings;

use crate::error::ErrorKind::Semver;
use crate::settings::{HookType, Settings};
use anyhow::{Context, Result};
use chrono::Utc;
use colored::*;
use conventional::changelog::{Changelog, ChangelogWriter, WriterMode};
use conventional::commit::Commit;
use conventional::commit::{CommitConfig, CommitMessage, CommitType};
use conventional::version::{parse_pre_release, VersionIncrement};
use git::repository::Repository;
use git2::{Oid, RebaseOptions};
use hook::Hook;
use itertools::Itertools;
use log::filter::CommitFilters;
use semver::Version;
use serde::export::fmt::Display;
use serde::export::Formatter;
use settings::AuthorSetting;
use std::fs::File;
use std::io::Write;
use std::path::{Path, PathBuf};
use std::process::{exit, Command, Stdio};
use std::{collections::HashMap, str::FromStr};
use tempdir::TempDir;

pub type CommitsMetadata = HashMap<CommitType, CommitConfig>;

pub const CONFIG_PATH: &str = "cog.toml";

lazy_static! {
    // This cannot be carried by `Cocogitto` struct since we need it to be available in `Changelog`,
    // `Commit` etc. Be ensure that `CocoGitto::new` is called before using this  in order to bypass
    // unwrapping in case of error.
    static ref COMMITS_METADATA: CommitsMetadata = {
        if let Ok(repo) = Repository::open(".") {
            Settings::get(&repo).unwrap_or_default().commit_types()
        } else {
            Settings::default().commit_types()
        }
    };

    static ref REMOTE_URL: Option<String> = {
        if let Ok(repo) = Repository::open(".") {
            Settings::get(&repo).unwrap_or_default().github
        } else {
            None
        }
    };

        static ref AUTHORS: Vec<AuthorSetting> = {
        if let Ok(repo) = Repository::open(".") {
            Settings::get(&repo).unwrap_or_default().authors
        } else {
            vec![]
        }
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
    let settings_path = path.join(CONFIG_PATH);
    if settings_path.exists() {
        eprint!("Found {} in {:?}, Nothing to do", CONFIG_PATH, &path);
        exit(1);
    } else {
        std::fs::write(
            &settings_path,
            toml::to_string(&settings)
                .map_err(|err| anyhow!("Failed to serialize {} : {}", CONFIG_PATH, err))?,
        )
        .map_err(|err| anyhow!("Could not write file `{:?}` : {}", &settings_path, err))?;
    }

    // TODO : add coco only"
    repository
        .add_all()
        .map_err(|err| anyhow!("Could not add file to repository index : {}", err))?;

    if is_init_commit {
        repository.commit("chore: initial commit")?;
    }

    Ok(())
}

pub struct CocoGitto {
    repository: Repository,
    changelog_path: PathBuf,
}

impl CocoGitto {
    pub fn get() -> Result<Self> {
        let repository = Repository::open(&std::env::current_dir()?)?;
        let settings = Settings::get(&repository)?;
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

    pub fn get_committer(&self) -> Result<String> {
        self.repository.get_author()
    }

    pub fn get_repo_tag_name(&self) -> Option<String> {
        let mut repo_tag_name = String::new();

        let repo_path = self.repository.get_repo_dir()?.iter().last()?;
        repo_tag_name.push_str(repo_path.to_str()?);

        if let Some(branch_shorthand) = self.repository.get_branch_shorthand() {
            repo_tag_name.push_str(" on ");
            repo_tag_name.push_str(&branch_shorthand);
        }

        if let Ok(latest_tag) = self.repository.get_latest_tag() {
            repo_tag_name.push(' ');
            repo_tag_name.push_str(&latest_tag);
        };

        Some(repo_tag_name)
    }

    pub fn check_and_edit(&self) -> Result<()> {
        let from = self.repository.get_first_commit()?;
        let head = self.repository.get_head_commit()?;
        let commits = self.repository.get_commit_range(from, head.id())?;
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

        // Get the last commit oid on the list as a starting point for our rebase
        let last_errored_commit = errored_commits.last();
        if let Some(last_errored_commit) = last_errored_commit {
            let commit = self
                .repository
                .0
                .find_commit(last_errored_commit.to_owned())?;

            let rebase_start = if commit.parent_count() == 0 {
                commit.id()
            } else {
                commit.parent_id(0)?
            };

            let commit = self.repository.0.find_annotated_commit(rebase_start)?;
            let mut options = RebaseOptions::new();

            let mut rebase =
                self.repository
                    .0
                    .rebase(None, Some(&commit), None, Some(&mut options))?;

            while let Some(op) = rebase.next() {
                if let Ok(rebase_operation) = op {
                    let oid = rebase_operation.id();
                    let original_commit = self.repository.0.find_commit(oid)?;
                    if errored_commits.contains(&oid) {
                        println!("Found errored commits : {}", &oid.to_string()[0..7]);
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
                        println!(
                            "Changed commit message to : \"{}\"",
                            &new_message.trim_end()
                        );
                    } else {
                        rebase.commit(None, &original_commit.committer(), None)?;
                    }
                } else {
                    eprintln!("{:?}", op);
                }
            }

            rebase.finish(None)?;
        }

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
            println!("{}", msg);
            Ok(())
        } else {
            let err: String = errors
                .iter()
                .map(|err| err.to_string())
                .intersperse("\n".to_string())
                .collect();
            Err(anyhow!("{}", err))
        }
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

        let oid = self.repository.commit(&message)?;
        let commit = self.repository.0.find_commit(oid)?;
        let commit = Commit::from_git_commit(&commit)?;
        println!("{}", commit);

        Ok(())
    }

    pub fn create_version(
        &self,
        increment: VersionIncrement,
        mode: WriterMode,
        pre_release: Option<&str>,
    ) -> Result<()> {
        let statuses = self.repository.get_statuses()?;

        // Fail if repo contains un-staged or un-committed changes
        if !statuses.0.is_empty() {
            return Err(anyhow!("{}", self.repository.get_statuses()?));
        }

        let current_tag = self
            .repository
            .get_latest_tag()
            .unwrap_or_else(|_| Version::new(0, 0, 0).to_string());

        let current_version = Version::parse(&current_tag)?;

        let mut next_version = increment
            .bump(&current_version)
            .map_err(|err| anyhow!("Cannot bump version : {}", err))?;

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

        if let Some(pre_release) = pre_release {
            next_version.pre = parse_pre_release(pre_release)?;
        }

        let version_str = next_version.to_string();

        let origin = if current_tag.as_str() == "0.0.0" {
            self.repository.get_first_commit()?.to_string()
        } else {
            current_tag
        };

        let changelog = self.get_changelog(
            Some(&origin),
            Some(&self.repository.get_head_commit_oid()?.to_string()),
            Some(next_version.to_string()),
        )?;

        let mut writter = ChangelogWriter {
            changelog,
            path: self.changelog_path.clone(),
            mode,
        };

        writter
            .write()
            .map_err(|err| anyhow!("Unable to write CHANGELOG.md : {}", err))?;

        self.run_hooks(HookType::PreBump, &version_str)?;

        self.repository.add_all()?;
        self.repository
            .commit(&format!("chore(version): {}", next_version))?;
        self.repository.create_tag(&version_str)?;

        self.run_hooks(HookType::PostBump, &version_str)?;

        let bump = format!("{} -> {}", current_version, next_version).green();
        println!("Bumped version : {}", bump);

        Ok(())
    }

    pub fn get_colored_changelog(&self, from: Option<&str>, to: Option<&str>) -> Result<String> {
        let mut changelog = self.get_changelog(from, to, None)?;
        Ok(changelog.markdown(true))
    }

    /// ## Get a changelog between two oids
    /// - `from` default value : latest tag or else first commit
    /// - `to` default value : `HEAD` or else first commit
    pub(crate) fn get_changelog(
        &self,
        from: Option<&str>,
        to: Option<&str>,
        tag_name: Option<String>,
    ) -> Result<Changelog> {
        let from = self.resolve_from_arg(from)?;
        let to = self.resolve_to_arg(to)?;

        let mut commits = vec![];

        let from_oid = from.get_oid();
        let to_oid = to.get_oid();

        for commit in self
            .repository
            .get_commit_range(from_oid, to_oid)
            .map_err(|err| anyhow!("Could not get commit range {}...{} : {}", from, to, err))?
        {
            // We skip the origin commit (ex: from 0.1.0 to 1.0.0)
            if commit.id() == from_oid {
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
            tag_name,
        })
    }

    // TODO : revparse
    fn resolve_to_arg(&self, to: Option<&str>) -> Result<OidOf> {
        if let Some(to) = to {
            self.get_raw_oid_or_tag_oid(to)
        } else {
            self.repository
                .get_head_commit_oid()
                .map(OidOf::Head)
                .or_else(|_err| self.repository.get_first_commit().map(OidOf::Other))
        }
    }

    // TODO : revparse
    fn resolve_from_arg(&self, from: Option<&str>) -> Result<OidOf> {
        if let Some(from) = from {
            self.get_raw_oid_or_tag_oid(from)
                .map_err(|err| anyhow!("Could not resolve from arg {} : {}", from, err))
        } else {
            self.repository
                .get_latest_tag_oidof()
                .or_else(|_err| self.repository.get_first_commit().map(OidOf::Other))
        }
    }

    fn get_raw_oid_or_tag_oid(&self, input: &str) -> Result<OidOf> {
        if let Ok(_version) = Version::parse(input) {
            self.repository
                .resolve_lightweight_tag(input)
                .map(|oid| OidOf::Tag(input.to_owned(), oid))
                .map_err(|err| anyhow!("tag {} not found : {} ", input, err))
        } else {
            Oid::from_str(input)
                .map(OidOf::Other)
                .map_err(|err| anyhow!("`{}` is not a valid oid : {}", input, err))
        }
    }

    fn run_hooks(&self, hook_type: HookType, next_version: &str) -> Result<()> {
        let settings = Settings::get(&self.repository)?;

        let hooks = settings
            .get_hooks(hook_type)
            .iter()
            .map(String::as_str)
            .map(Hook::from_str)
            .enumerate()
            .map(|(idx, result)| result.context(format!("Cannot parse hook at index {}", idx)))
            .collect::<Result<Vec<Hook>>>()?;

        for mut hook in hooks {
            hook.insert_version(next_version);
            hook.run().context(format!("{}", hook))?;
        }

        Ok(())
    }
}

/// A wrapper for git2 oid including tags and HEAD ref
enum OidOf {
    Tag(String, Oid),
    Head(Oid),
    Other(Oid),
}

impl OidOf {
    fn get_oid(&self) -> Oid {
        match self {
            OidOf::Tag(_, v) | OidOf::Head(v) | OidOf::Other(v) => v.to_owned(),
        }
    }
}

impl Display for OidOf {
    /// Print the oid according to it's type
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            OidOf::Tag(tag, _) => write!(f, "{}", tag),
            OidOf::Head(_) => write!(f, "HEAD"),
            OidOf::Other(oid) => write!(f, "{}", &oid.to_string()[0..6]),
        }
    }
}
