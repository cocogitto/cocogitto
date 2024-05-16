use std::collections::BTreeMap;
use std::path::PathBuf;
use std::process::{Command, Stdio};

use anyhow::Result;

use cocogitto_config::Settings;
use conventional::version::IncrementCommand;
use error::BumpError;
use git::repository::Repository;

use crate::git::error::Git2Error;
use crate::git::rev::cache::get_cache;

pub mod command;
pub mod conventional;
pub mod error;
pub mod git;
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
    pub fn get() -> Result<Self> {
        let current_dir = &std::env::current_dir()?;
        let repository = Repository::open(current_dir)?;
        let _settings = Settings::get(current_dir.as_path())?;
        let _changelog_path = cocogitto_config::changelog_path();

        Ok(CocoGitto { repository })
    }

    pub fn get_committer(&self) -> Result<String, Git2Error> {
        self.repository.get_author()
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
    use cargo_metadata::MetadataCommand;
    use cmd_lib::run_cmd;

    use crate::git::repository::Repository;

    pub fn git_init_no_gpg() -> anyhow::Result<Repository> {
        run_cmd!(
            git init -b master;
            git config --local commit.gpgsign false;
        )?;

        Ok(Repository::open(".")?)
    }

    pub fn open_cocogitto_repo() -> anyhow::Result<Repository> {
        let metadata = MetadataCommand::new()
            .exec()
            .expect("Failed to get cargo metadata");
        let workspace = metadata.workspace_root;

        let repo = Repository::open(&workspace)?;
        Ok(repo)
    }
}
