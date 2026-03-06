use anyhow::Result;
use cocogitto_settings::Settings;
use conventional::commit::Commit;
use conventional::version::IncrementCommand;
use conventional_commit_parser::commit::{CommitType, ConventionalCommit};
use conventional_commit_parser::parse_footers;
use error::BumpError;
use git::repository::Repository;
use std::collections::BTreeMap;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::PathBuf;
use std::process::{Command, Stdio};

use crate::git::error::{Git2Error, TagError};
use crate::git::rev::cache::get_cache;

use crate::git::tag::Tag;

pub mod command;
pub mod conventional;
pub mod error;
pub mod git;
pub mod hook;
pub mod log;

#[derive(Debug)]
pub struct CocoGitto {
    repository: Repository,
}

pub enum CommitHook {
    PreCommit,
    PrepareCommitMessage(String),
    CommitMessage,
    PostCommit,
}

impl CocoGitto {
    pub fn get_at(path: PathBuf) -> Result<Self> {
        let repository = Repository::open(&path)?;
        let _settings = Settings::try_from_path(&path)?;
        let _changelog_path = cocogitto_settings::changelog_path();

        Ok(CocoGitto { repository })
    }

    pub fn get() -> Result<Self> {
        CocoGitto::get_at(std::env::current_dir()?)
    }

    pub fn get_committer(&self) -> Result<String, Git2Error> {
        self.repository.get_author()
    }

    /// Tries to get a commit message conforming to the Conventional Commit spec.
    /// If the commit message does _not_ conform, `None` is returned instead.
    pub fn get_conventional_message(
        commit_type: &str,
        scope: Option<String>,
        summary: String,
        body: Option<String>,
        footer: Option<String>,
        is_breaking_change: bool,
    ) -> Result<String> {
        // Ensure commit type is known
        let commit_type = CommitType::from(commit_type);

        // Ensure footers are correctly formatted
        let footers = match footer {
            Some(footers) => parse_footers(&footers)?,
            None => Vec::with_capacity(0),
        };

        let conventional_message = ConventionalCommit {
            commit_type,
            scope,
            body,
            footers,
            summary,
            is_breaking_change,
        }
        .to_string();

        // Validate the message
        conventional_commit_parser::parse(&conventional_message)?;

        Ok(conventional_message)
    }

    pub fn run_commit_hook(&self, hook: CommitHook) -> Result<(), Git2Error> {
        let git_dir = self.repository.get_git_dir();
        let repo_dir = self.repository.get_repo_dir().expect("git repository");
        let git_config = self.repository.0.config()?;
        let hooks_dir = git_config
            .get_string("core.hooksPath")
            .map(|path| repo_dir.join(path))
            .unwrap_or_else(|_| git_dir.join("hooks"));

        let edit_message = git_dir.join("COMMIT_EDITMSG");
        let edit_message = edit_message.to_string_lossy();

        let (hook_path, args) = match hook {
            CommitHook::PreCommit => (hooks_dir.join("pre-commit"), vec![]),
            CommitHook::PrepareCommitMessage(template) => (
                hooks_dir.join("prepare-commit-msg"),
                vec![edit_message.to_string(), template],
            ),
            CommitHook::CommitMessage => {
                (hooks_dir.join("commit-msg"), vec![edit_message.to_string()])
            }
            CommitHook::PostCommit => (hooks_dir.join("post-commit"), vec![]),
        };

        if hook_path.exists() {
            let file = File::open(&hook_path)?;
            let mut reader = io::BufReader::new(file);
            let mut first_line = String::new();
            reader.read_line(&mut first_line)?;

            let mut command = if first_line.starts_with("#!") {
                Command::new(&hook_path)
            } else {
                let mut cmd = Command::new("sh");
                cmd.arg(&hook_path);
                cmd
            };

            let status = command
                .args(&args)
                .stdout(Stdio::inherit())
                .stdin(Stdio::inherit())
                .stderr(Stdio::inherit())
                .output()?
                .status;

            if !status.success() {
                return Err(Git2Error::GitHookNonZeroExit(status.code().unwrap_or(1)));
            }
        }

        Ok(())
    }

    pub fn prepare_edit_message_path(&self) -> PathBuf {
        self.repository.get_git_dir().join("COMMIT_EDITMSG")
    }

    // Currently only used in test to force rebuild the tag cache
    pub fn clear_cache(&self) {
        let mut cache = get_cache(&self.repository);
        *cache = BTreeMap::new();
    }
}
