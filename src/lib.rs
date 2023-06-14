use std::collections::HashMap;
use std::path::PathBuf;
use std::process::{Command, Stdio};

use anyhow::Result;

use conventional_commit_parser::commit::{CommitType, ConventionalCommit};
use conventional_commit_parser::parse_footers;
use once_cell::sync::Lazy;

use conventional::commit::{Commit, CommitConfig};
use conventional::version::IncrementCommand;
use error::BumpError;
use git::repository::Repository;

use settings::Settings;

use crate::git::error::{Git2Error, TagError};

use crate::git::revspec::RevspecPattern;
use crate::git::tag::Tag;

pub mod command;
pub mod conventional;
pub mod error;
pub mod git;
pub mod hook;
pub mod log;
pub mod settings;

pub type CommitsMetadata = HashMap<CommitType, CommitConfig>;

pub const CONFIG_PATH: &str = "cog.toml";

pub static SETTINGS: Lazy<Settings> = Lazy::new(|| {
    if let Ok(repo) = Repository::open(".") {
        Settings::get(&repo).unwrap_or_default()
    } else {
        Settings::default()
    }
});

// This cannot be carried by `Cocogitto` struct since we need it to be available in `Changelog`,
// `Commit` etc. Be sure that `CocoGitto::new` is called before using this  in order to bypass
// unwrapping in case of error.
pub static COMMITS_METADATA: Lazy<CommitsMetadata> = Lazy::new(|| SETTINGS.commit_types());

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
    pub fn get() -> Result<Self> {
        let repository = Repository::open(&std::env::current_dir()?)?;
        let _settings = Settings::get(&repository)?;
        let _changelog_path = settings::changelog_path();

        Ok(CocoGitto { repository })
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
        let repo_dir = self.repository.get_repo_dir().expect("git repository");
        let hooks_dir = repo_dir.join(".git/hooks");
        let edit_message = repo_dir.join(".git/COMMIT_EDITMSG");
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
            let status = Command::new(hook_path)
                .args(args)
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
        self.repository
            .get_repo_dir()
            .map(|path| path.join(".git/COMMIT_EDITMSG"))
            .expect("git repository")
    }
}
