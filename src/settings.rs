use crate::conventional::commit::{CommitConfig, CommitType};
use crate::git::repository::Repository;
use crate::{CommitsMetadata, CONFIG_PATH};
use anyhow::Result;
use config::{Config, File};
use std::collections::HashMap;
use std::path::PathBuf;

type CommitsMetadataSettings = HashMap<String, CommitConfig>;
pub(crate) type AuthorSettings = Vec<AuthorSetting>;

#[derive(Copy, Clone)]
pub(crate) enum HookType {
    PreBump,
    PostBump,
}

#[derive(Debug, Deserialize, Serialize)]
pub(crate) struct Settings {
    pub changelog_path: Option<PathBuf>,
    pub github: Option<String>,
    #[serde(default)]
    pub pre_bump_hooks: Vec<String>,
    #[serde(default)]
    pub post_bump_hooks: Vec<String>,
    #[serde(default)]
    pub authors: AuthorSettings,
    #[serde(default)]
    pub commit_types: CommitsMetadataSettings,
    #[serde(default)]
    pub sign_commit: bool,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct AuthorSetting {
    pub signature: String,
    pub username: String,
}

impl Default for Settings {
    fn default() -> Self {
        Settings {
            changelog_path: Some(PathBuf::from("CHANGELOG.md")),
            commit_types: Default::default(),
            pre_bump_hooks: vec![],
            post_bump_hooks: vec![],
            authors: vec![],
            github: None,
            sign_commit: false,
        }
    }
}

impl Settings {
    // Fails only if config exists and is malformed
    pub(crate) fn get(repository: &Repository, config_path: Option<&str>) -> Result<Self> {
        match repository.get_repo_dir() {
            Some(repo_path) => {
                let settings_path = repo_path.join(config_path.unwrap_or(CONFIG_PATH));
                if settings_path.exists() {
                    let mut s = Config::new();
                    s.merge(File::from(settings_path))?;
                    s.try_into()
                        .map_err(|err| anyhow!("Config format error : {}", err))
                } else {
                    Ok(Settings::default())
                }
            }
            None => Ok(Settings::default()),
        }
    }

    pub(crate) fn commit_types(&self) -> CommitsMetadata {
        let commit_settings = self.commit_types.clone();
        let mut custom_types = HashMap::new();

        commit_settings.iter().for_each(|(key, value)| {
            let _ = custom_types.insert(CommitType::from(key.as_str()), value.clone());
        });

        let mut default_types = Settings::get_default_commit_config();

        default_types.extend(custom_types);

        default_types
    }

    fn get_default_commit_config() -> CommitsMetadata {
        let mut default_types = HashMap::new();
        default_types.insert(CommitType::Feature, CommitConfig::new("Features"));
        default_types.insert(CommitType::BugFix, CommitConfig::new("Bug Fixes"));
        default_types.insert(CommitType::Chore, CommitConfig::new("Miscellaneous Chores"));
        default_types.insert(CommitType::Revert, CommitConfig::new("Revert"));
        default_types.insert(
            CommitType::Performances,
            CommitConfig::new("Performance Improvements"),
        );
        default_types.insert(
            CommitType::Documentation,
            CommitConfig::new("Documentation"),
        );
        default_types.insert(CommitType::Style, CommitConfig::new("Style"));
        default_types.insert(CommitType::Refactoring, CommitConfig::new("Refactoring"));
        default_types.insert(CommitType::Test, CommitConfig::new("Tests"));

        default_types.insert(CommitType::Build, CommitConfig::new("Build system"));

        default_types.insert(CommitType::Ci, CommitConfig::new("Continuous Integration"));

        default_types
    }

    pub fn get_hooks(&self, hook_type: HookType) -> &Vec<String> {
        match hook_type {
            HookType::PreBump => &self.pre_bump_hooks,
            HookType::PostBump => &self.post_bump_hooks,
        }
    }
}
