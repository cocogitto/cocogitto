use std::collections::HashMap;

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
}
