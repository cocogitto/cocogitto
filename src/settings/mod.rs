use std::collections::HashMap;
use std::path::PathBuf;

use crate::conventional::commit::CommitConfig;
use crate::git::repository::Repository;
use crate::{CommitsMetadata, CONFIG_PATH, SETTINGS};

use crate::conventional::changelog::error::ChangelogError;
use crate::conventional::changelog::template::{RemoteContext, Template};
use crate::git::hook::Hooks;
use crate::settings::error::SettingError;
use config::{Config, File};
use conventional_commit_parser::commit::CommitType;
use serde::{Deserialize, Serialize};

type CommitsMetadataSettings = HashMap<String, CommitConfig>;
pub(crate) type AuthorSettings = Vec<AuthorSetting>;

mod error;

#[derive(Copy, Clone)]
pub enum HookType {
    PreBump,
    PostBump,
}

#[derive(Debug, Deserialize, Serialize, Eq, PartialEq, Default)]
#[serde(deny_unknown_fields)]
pub struct Settings {
    #[serde(default)]
    pub from_latest_tag: bool,
    #[serde(default)]
    pub ignore_merge_commits: bool,
    #[serde(default)]
    pub branch_whitelist: Vec<String>,
    pub tag_prefix: Option<String>,
    #[serde(default)]
    pub pre_bump_hooks: Vec<String>,
    #[serde(default)]
    pub post_bump_hooks: Vec<String>,
    #[serde(default)]
    pub commit_types: CommitsMetadataSettings,
    #[serde(default)]
    pub changelog: Changelog,
    #[serde(default)]
    pub bump_profiles: HashMap<String, BumpProfile>,
    #[serde(default)]
    pub packages: HashMap<String, MonoRepoPackage>,
}

#[derive(Debug, Deserialize, Serialize, Eq, PartialEq, Default)]
#[serde(deny_unknown_fields, default)]
pub struct MonoRepoPackage {
    pub path: PathBuf,
    pub changelog_path: Option<String>,
    pub pre_bump_hooks: Vec<String>,
    pub post_bump_hooks: Vec<String>,
    pub bump_profiles: HashMap<String, BumpProfile>,
}

impl MonoRepoPackage {
    pub fn changelog_path(&self) -> PathBuf {
        self.changelog_path
            .as_ref()
            .map(PathBuf::from)
            .unwrap_or_else(|| self.path.join("CHANGELOG.md"))
    }
}

#[derive(Debug, Deserialize, Serialize, Clone, Eq, PartialEq)]
#[serde(deny_unknown_fields, default)]
pub struct Changelog {
    pub template: Option<String>,
    pub remote: Option<String>,
    pub path: PathBuf,
    pub owner: Option<String>,
    pub repository: Option<String>,
    pub authors: AuthorSettings,
}

impl Default for Changelog {
    fn default() -> Self {
        Changelog {
            template: None,
            remote: None,
            path: PathBuf::from("CHANGELOG.md"),
            owner: None,
            repository: None,
            authors: vec![],
        }
    }
}

#[derive(Debug, Deserialize, Serialize, Clone, Eq, PartialEq)]
#[serde(deny_unknown_fields)]
pub struct AuthorSetting {
    pub signature: String,
    pub username: String,
}

pub fn commit_username(author: &str) -> Option<&'static str> {
    SETTINGS
        .changelog
        .authors
        .iter()
        .find(|author_map| author_map.signature == author)
        .map(|author| author.username.as_str())
}

pub fn changelog_path() -> &'static PathBuf {
    &SETTINGS.changelog.path
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
    pub(crate) fn get(repository: &Repository) -> Result<Self, SettingError> {
        match repository.get_repo_dir() {
            Some(repo_path) => {
                let settings_path = repo_path.join(CONFIG_PATH);
                if settings_path.exists() {
                    Config::builder()
                        .add_source(File::from(settings_path))
                        .build()
                        .map_err(SettingError::from)?
                        .try_deserialize()
                        .map_err(SettingError::from)
                } else {
                    Ok(Settings::default())
                }
            }
            None => Ok(Settings::default()),
        }
    }

    pub fn commit_types(&self) -> CommitsMetadata {
        let commit_settings = self.commit_types.clone();
        let mut custom_types = HashMap::new();

        commit_settings.iter().for_each(|(key, value)| {
            let _ = custom_types.insert(CommitType::from(key.as_str()), value.clone());
        });

        let mut default_types = Settings::default_commit_config();

        default_types.extend(custom_types);

        default_types
    }

    fn default_commit_config() -> CommitsMetadata {
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
    }

    pub fn get_template_context(&self) -> Option<RemoteContext> {
        let remote = self.changelog.remote.as_ref().cloned();

        let repository = self.changelog.repository.as_ref().cloned();

        let owner = self.changelog.owner.as_ref().cloned();

        RemoteContext::try_new(remote, repository, owner)
    }

    pub fn get_changelog_template(&self) -> Result<Template, ChangelogError> {
        let context = self.get_template_context();
        let template = self.changelog.template.as_deref().unwrap_or("default");

        Template::from_arg(template, context)
    }
}

impl Hooks for Settings {
    fn bump_profiles(&self) -> &HashMap<String, BumpProfile> {
        &self.bump_profiles
    }

    fn pre_bump_hooks(&self) -> &Vec<String> {
        &self.pre_bump_hooks
    }

    fn post_bump_hooks(&self) -> &Vec<String> {
        &self.post_bump_hooks
    }
}

impl Hooks for MonoRepoPackage {
    fn bump_profiles(&self) -> &HashMap<String, BumpProfile> {
        &self.bump_profiles
    }

    fn pre_bump_hooks(&self) -> &Vec<String> {
        &self.pre_bump_hooks
    }

    fn post_bump_hooks(&self) -> &Vec<String> {
        &self.post_bump_hooks
    }
}
