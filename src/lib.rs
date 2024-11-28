use std::collections::{BTreeMap, HashMap};
use std::path::PathBuf;
use std::process::{Command, Stdio};

use anyhow::Result;

use ::log::warn;
use conventional_commit_parser::commit::{CommitType, ConventionalCommit};
use conventional_commit_parser::parse_footers;
use once_cell::sync::Lazy;

use conventional::commit::{Commit, CommitConfig};
use conventional::version::IncrementCommand;
use error::BumpError;
use git::repository::Repository;

use serde::{Deserialize, Serialize};
use settings::Settings;

use crate::git::error::{Git2Error, TagError};
use crate::git::rev::cache::get_cache;

use crate::git::tag::Tag;

pub mod command;
pub mod conventional;
pub mod error;
pub mod git;
pub mod hook;
pub mod log;
pub mod settings;

pub type CommitsMetadata = HashMap<CommitType, CommitConfigOrNull>;

#[cfg_attr(feature = "docgen", derive(schemars::JsonSchema))]
#[derive(Debug, Serialize, Deserialize, PartialEq, Eq, Clone)]
#[serde(untagged)]
pub enum CommitConfigOrNull {
    CommitConfig(CommitConfig),
    None {},
}

pub const CONFIG_PATH: &str = "cog.toml";

pub static SETTINGS: Lazy<Settings> = Lazy::new(|| {
    if let Ok(repo) = Repository::open(".") {
        let settings = Settings::get(&repo);
        if let Err(err) = settings.as_ref() {
            warn!("Failed to get config, falling back to default: {err}");
        }

        return settings.unwrap_or_default();
    }

    Settings::default()
});

// This cannot be carried by `Cocogitto` struct since we need it to be available in `Changelog`,
// `Commit` etc. Be sure that `CocoGitto::new` is called before using this  in order to bypass
// unwrapping in case of error.
pub static COMMITS_METADATA: Lazy<HashMap<CommitType, CommitConfig>> =
    Lazy::new(|| SETTINGS.commit_types());

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
        let _settings = Settings::get(&repository)?;
        let _changelog_path = settings::changelog_path();

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

    // Currently only used in test to force rebuild the tag cache
    pub fn clear_cache(&self) {
        let mut cache = get_cache(&self.repository);
        *cache = BTreeMap::new();
    }
}

#[cfg(test)]
pub mod test_helpers {
    use crate::git::repository::Repository;
    use cmd_lib::{run_cmd, run_fun};

    pub(crate) fn git_init_no_gpg() -> anyhow::Result<Repository> {
        run_cmd!(
            git init -b master;
            git config --local commit.gpgsign false;
        )?;

        Ok(Repository::open(".")?)
    }

    pub(crate) fn commit(message: &str) -> anyhow::Result<String> {
        Ok(run_fun!(
            git commit --allow-empty -q -m $message;
            git log --format=%H -n 1;
        )?)
    }

    pub(crate) fn git_tag(version: &str) -> anyhow::Result<()> {
        run_fun!(git tag $version;)?;
        Ok(())
    }
}
