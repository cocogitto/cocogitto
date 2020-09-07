use std::path::PathBuf;
use anyhow::Result;
use crate::repository::Repository;
use config::{Config, File};
use crate::commit::SortCommit;
use std::collections::HashMap;

#[derive(Debug, Deserialize)]
pub struct Settings {
    #[serde(default)]
    pub hooks: Vec<String>,
    #[serde(default)]
    pub commit_types: HashMap<String, String>,
    pub changelog_file: PathBuf,
    pub sort_commit: SortCommit,
}

impl Settings {
    pub fn get(repository: &Repository) -> Result<Self> {
        match repository.get_repo_dir() {
            Some(path) => {
                if path.exists() {
                    let mut s = Config::new();
                    s.merge(File::from(path.join("coco.toml")))?;
                    s.try_into()
                        .map_err(|err| anyhow!("Config format error : {}", err))
                } else {
                    Err(anyhow!(
                        "Missing `coco.toml` config file in {}",
                        path.display()
                    ))
                }
            }
            None => Err(anyhow!("Current dir is not a git repository")),
        }
    }
}

