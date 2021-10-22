use std::collections::HashMap;
use std::path::PathBuf;

use crate::conventional::commit::CommitConfig;
use crate::git::repository::Repository;
use crate::{CommitsMetadata, CONFIG_PATH};

use crate::SETTINGS;
use anyhow::{anyhow, Result};
use config::{Config, File};
use conventional_commit_parser::commit::CommitType;
use lazy_static::lazy_static;
use serde::{Deserialize, Serialize};

type CommitsMetadataSettings = HashMap<String, CommitConfig>;
pub(crate) type AuthorSettings = Vec<AuthorSetting>;

lazy_static! {
    pub static ref COMMITS_METADATA: CommitsMetadata<'static> = {
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
        default_types.insert(CommitType::Refactor, CommitConfig::new("Refactoring"));
        default_types.insert(CommitType::Test, CommitConfig::new("Tests"));

        default_types.insert(CommitType::Build, CommitConfig::new("Build system"));

        default_types.insert(CommitType::Ci, CommitConfig::new("Continuous Integration"));

        default_types
    };
}

#[derive(Copy, Clone)]
pub enum HookType {
    PreBump,
    PostBump,
}

#[derive(Debug, Deserialize, Serialize, Eq, PartialEq)]
#[serde(deny_unknown_fields)]
pub struct Settings {
    pub changelog_path: Option<PathBuf>,
    pub github: Option<String>,
    #[serde(default)]
    pub tag_prefix: Option<String>,
    #[serde(default)]
    pub pre_bump_hooks: Vec<String>,
    #[serde(default)]
    pub post_bump_hooks: Vec<String>,
    #[serde(default)]
    pub authors: AuthorSettings,
    #[serde(default)]
    pub commit_types: CommitsMetadataSettings,
    #[serde(default)]
    pub bump_profiles: HashMap<String, BumpProfile>,
}

#[derive(Debug, Deserialize, Serialize, Clone, Eq, PartialEq)]
#[serde(deny_unknown_fields)]
pub struct AuthorSetting {
    pub signature: String,
    pub username: String,
}

impl Default for Settings {
    fn default() -> Self {
        Settings {
            changelog_path: Some(PathBuf::from("CHANGELOG.md")),
            github: Default::default(),
            pre_bump_hooks: Default::default(),
            post_bump_hooks: Default::default(),
            authors: Default::default(),
            commit_types: Default::default(),
            bump_profiles: Default::default(),
            tag_prefix: None,
        }
    }
}

#[derive(Debug, Deserialize, Serialize, Default, Eq, PartialEq)]
#[serde(deny_unknown_fields)]
pub struct BumpProfile {
    #[serde(default)]
    pub pre_bump_hooks: Vec<String>,
    #[serde(default)]
    pub post_bump_hooks: Vec<String>,
}

impl Settings {
    // Fails only if config exists and is malformed
    pub(crate) fn get(repository: &Repository) -> Result<Self> {
        match repository.get_repo_dir() {
            Some(repo_path) => {
                let settings_path = repo_path.join(CONFIG_PATH);
                if settings_path.exists() {
                    let mut s = Config::new();
                    s.merge(File::from(settings_path))?;
                    s.try_into()
                        .map_err(|err| anyhow!("Config format error:{}", err))
                } else {
                    Ok(Settings::default())
                }
            }
            None => Ok(Settings::default()),
        }
    }

    pub fn get_hooks(&self, hook_type: HookType) -> &Vec<String> {
        match hook_type {
            HookType::PreBump => &self.pre_bump_hooks,
            HookType::PostBump => &self.post_bump_hooks,
        }
    }

    pub fn get_profile_hook(&self, profile: &str, hook_type: HookType) -> &Vec<String> {
        let profile = self
            .bump_profiles
            .get(profile)
            .expect("Bump profile not found");
        match hook_type {
            HookType::PreBump => &profile.pre_bump_hooks,
            HookType::PostBump => &profile.post_bump_hooks,
        }
    }
}
