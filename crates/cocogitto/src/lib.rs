use std::collections::{BTreeMap, HashMap};
use std::fs::File;
use std::io::{self, BufRead};
use std::path::{Path, PathBuf};
use std::process::{Command, Stdio};
use std::sync::OnceLock;

use anyhow::Result;

use ::log::warn;
use conventional_commit_parser::commit::{CommitType, ConventionalCommit};
use conventional_commit_parser::parse_footers;
use once_cell::sync::Lazy;

use conventional::commit::{Commit, CommitConfig};
use conventional::version::IncrementCommand;
use error::BumpError;
use git::repository::Repository;

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
/// Settings module containing configuration structures and functions for Cocogitto
pub mod settings;

pub const DEFAULT_CONFIG_PATH: &str = "cog.toml";
static CONFIG_PATH: OnceLock<String> = OnceLock::new();

pub fn get_config_path() -> &'static String {
    if cfg!(test) || std::env::var("RUSTY_FORK_OCCURS").is_ok() {
        CONFIG_PATH.get_or_init(|| DEFAULT_CONFIG_PATH.to_owned())
    } else {
        CONFIG_PATH.get().expect("config path to be set")
    }
}

pub fn set_config_path(path: String) {
    CONFIG_PATH
        .set(path)
        .expect("config path should not be set");
}

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
    Lazy::new(|| SETTINGS.load_commit_types());

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

/// Build the command used to execute a git hook.
///
/// On Windows the kernel cannot interpret shebangs, so hooks are run through
/// a POSIX shell shipped with Git. On Unix, shebang hooks are executed
/// directly and the kernel resolves the interpreter for us.
#[cfg(windows)]
fn git_hook_command(hook_path: &Path, _has_shebang: bool) -> Command {
    if let Some(sh) = windows_shell() {
        let mut command = Command::new(sh);
        command.arg(hook_path);
        return command;
    }

    Command::new(hook_path)
}

#[cfg(not(windows))]
fn git_hook_command(hook_path: &Path, has_shebang: bool) -> Command {
    if has_shebang {
        Command::new(hook_path)
    } else {
        let mut command = Command::new("sh");
        command.arg(hook_path);
        command
    }
}

/// Path to a POSIX shell on Windows, resolved once and reused across hooks.
#[cfg(windows)]
static WINDOWS_SHELL: Lazy<Option<PathBuf>> = Lazy::new(|| {
    if let Ok(sh) = which::which("sh") {
        return Some(sh);
    }

    let mut candidates = Vec::new();
    if let Ok(git) = which::which("git") {
        if let Some(git_root) = git.parent().and_then(Path::parent) {
            candidates.push(git_root.join("usr").join("bin").join("sh.exe"));
            candidates.push(git_root.join("bin").join("sh.exe"));
        }
    }
    candidates.into_iter().find(|path| path.exists())
});

#[cfg(windows)]
fn windows_shell() -> Option<PathBuf> {
    WINDOWS_SHELL.clone()
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

            let mut command = git_hook_command(&hook_path, first_line.starts_with("#!"));

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

    pub(crate) fn mkdir(dirs: &[&str]) -> anyhow::Result<()> {
        for dir in dirs {
            std::fs::create_dir_all(dir)?;
        }
        Ok(())
    }
}

#[cfg(test)]
mod test {
    #[test]
    #[cfg(windows)]
    fn windows_shell_is_resolved_from_git() {
        // Arrange
        let sh = super::windows_shell();

        // Act
        let sh = sh.expect("sh should be resolvable on Windows via Git for Windows");

        // Assert
        assert!(
            sh.is_file(),
            "resolved shell should exist on disk: {}",
            sh.display()
        );
        assert_eq!(
            sh.file_name().and_then(|name| name.to_str()),
            Some("sh.exe")
        );
    }
}
