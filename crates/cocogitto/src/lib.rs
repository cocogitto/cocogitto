use std::collections::BTreeMap;

use anyhow::Result;

use cocogitto_config::Settings;
use cocogitto_git::error::Git2Error;
use cocogitto_git::rev::cache::get_cache;
use cocogitto_git::Repository;
use error::BumpError;

pub mod command;
pub mod error;

pub trait CogCommand {
    fn settings() -> Result<Settings> {
        let current_dir = &std::env::current_dir()?;
        Settings::get(current_dir.as_path()).map_err(Into::into)
    }

    fn repository() -> Result<Repository> {
        let current_dir = &std::env::current_dir()?;
        Repository::open(current_dir).map_err(Into::into)
    }

    fn execute(self) -> anyhow::Result<()>;
}

#[derive(Debug)]
pub struct CocoGitto {
    repository: Repository,
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

    // Currently only used in test to force rebuild the tag cache
    pub fn clear_cache(&self) {
        let mut cache = get_cache(&self.repository);
        *cache = BTreeMap::new();
    }
}
