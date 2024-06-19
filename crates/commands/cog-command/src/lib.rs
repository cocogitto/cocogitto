use std::path::{Path, PathBuf};

use anyhow::{anyhow, Result};

use cocogitto_config::Settings;
use cocogitto_git::Repository;

pub trait CogCommand {
    fn settings(path: &Path) -> Result<Settings> {
        Settings::try_from(path).map_err(anyhow::Error::from)
    }

    fn repository() -> Result<Repository> {
        let current_dir = &std::env::current_dir()?;
        Repository::open(current_dir).map_err(Into::into)
    }

    fn default_path() -> Result<PathBuf> {
        let repository = Self::repository()?;
        repository
            .get_repo_dir()
            .map(Path::to_path_buf)
            .ok_or(anyhow!("Repository path"))
    }

    fn execute(self) -> Result<()>;
}
