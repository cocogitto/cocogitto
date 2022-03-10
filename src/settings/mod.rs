use std::collections::HashMap;
use std::path::PathBuf;

use config::{Config, File};
use conventional_commit_parser::commit::CommitType;
use serde::{Deserialize, Serialize};

use crate::conventional::changelog::error::ChangelogError;
use crate::conventional::changelog::template::{RemoteContext, Template};
use crate::conventional::commit::CommitConfig;
use crate::git::repository::Repository;
use crate::settings::error::SettingError;
use crate::{CommitsMetadata, CONFIG_PATH, SETTINGS};

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
        default_types.insert(
            CommitType::Feature,
            CommitConfig::new(
                "Features",
                Some("patches a bug in your codebase".to_string()),
            ),
        );
        default_types.insert(
            CommitType::BugFix,
            CommitConfig::new(
                "Bug Fixes",
                Some("introduces a new feature to the codebase".to_string()),
            ),
        );
        default_types.insert(
            CommitType::Chore,
            CommitConfig::new(
                "Miscellaneous Chores",
                Some("miscellaneous chores".to_string()),
            ),
        );
        default_types.insert(
            CommitType::Revert,
            CommitConfig::new("Revert", Some("a rollback of a previous change\nSee here how conventional commit handles reverts : https://www.conventionalcommits.org/en/v1.0.0/#how-does-conventional-commits-handle-revert-commits".to_string())),
        );
        default_types.insert(
            CommitType::Performances,
            CommitConfig::new(
                "Performance Improvements",
                Some("a code change that improves performance".to_string()),
            ),
        );
        default_types.insert(
            CommitType::Documentation,
            CommitConfig::new(
                "Documentation",
                Some("documentation only changes (ex: README, Javadoc, Rustdoc)".to_string()),
            ),
        );
        default_types.insert(
            CommitType::Style,
            CommitConfig::new(
                "Style",
                Some("a code change that improves performance".to_string()),
            ),
        );
        default_types.insert(
            CommitType::Refactor,
            CommitConfig::new(
                "Refactoring",
                Some("a code change that neither fixes a bug nor adds a feature".to_string()),
            ),
        );
        default_types.insert(
            CommitType::Test,
            CommitConfig::new(
                "Tests",
                Some("adding missing tests or correcting existing tests".to_string()),
            ),
        );
        default_types.insert(
            CommitType::Build,
            CommitConfig::new("Build system", Some("changes that affect the build system or external dependencies (example: Maven, NPM, cargo)".to_string())),
        );
        default_types.insert(
            CommitType::Ci,
            CommitConfig::new("Continuous Integration", Some("changes to the CI configuration files and scripts (example: Travis, Circle, Github Actions)".to_string())),
        );
        default_types
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
