use std::collections::HashMap;
use std::path::PathBuf;

use changelog::Changelog;
use commit::CommitConfig;
use config::{Config, File, FileFormat};
use conventional_commit_parser::commit::CommitType;
use error::SettingError;
use git_hook::{GitHook, GitHookType};
use log::warn;
use monorepo::MonoRepoPackage;
use once_cell::sync::Lazy;
use serde::{Deserialize, Serialize};
use std::path::Path;

pub mod changelog;
pub mod commit;
pub mod error;
pub mod git_hook;
pub mod hook;
pub mod monorepo;

pub const CONFIG_PATH: &str = "cog.toml";

pub static SETTINGS: Lazy<Settings> = Lazy::new(|| {
    if let Ok((path, _trust)) = gix_discover::upwards(&PathBuf::from(".")) {
        let path = match path {
            gix_discover::repository::Path::LinkedWorkTree {
                work_dir: _,
                git_dir,
            } => git_dir,
            gix_discover::repository::Path::WorkTree(path) => path,
            gix_discover::repository::Path::Repository(path) => path,
        };

        let settings = Settings::get(path.as_path());
        if let Err(err) = settings.as_ref() {
            warn!("Failed to get config, falling back to default: {err}");
        }

        return settings.unwrap_or_default();
    }

    Settings::default()
});

pub static COMMITS_METADATA: Lazy<HashMap<CommitType, CommitConfig>> =
    Lazy::new(|| SETTINGS.commit_types());

pub type CommitsMetadata = HashMap<CommitType, CommitConfigOrNull>;

#[derive(Debug, Serialize, Deserialize, PartialEq, Eq, Clone)]
#[serde(untagged)]
pub enum CommitConfigOrNull {
    CommitConfig(CommitConfig),
    None {},
}

#[derive(Debug, Deserialize, Serialize, Eq, PartialEq)]
#[serde(deny_unknown_fields, default)]
pub struct Settings {
    pub from_latest_tag: bool,
    pub ignore_merge_commits: bool,
    pub disable_changelog: bool,
    pub disable_bump_commit: bool,
    pub generate_mono_repository_global_tag: bool,
    pub monorepo_version_separator: Option<String>,
    pub branch_whitelist: Vec<String>,
    pub tag_prefix: Option<String>,
    pub skip_ci: String,
    pub skip_untracked: bool,
    pub pre_bump_hooks: Vec<String>,
    pub post_bump_hooks: Vec<String>,
    pub pre_package_bump_hooks: Vec<String>,
    pub post_package_bump_hooks: Vec<String>,
    pub git_hooks: HashMap<GitHookType, GitHook>,
    pub commit_types: HashMap<String, CommitConfigOrNull>,
    pub changelog: Changelog,
    pub bump_profiles: HashMap<String, BumpProfile>,
    pub packages: HashMap<String, MonoRepoPackage>,
}

impl Default for Settings {
    fn default() -> Self {
        Self {
            from_latest_tag: false,
            ignore_merge_commits: false,
            disable_changelog: false,
            disable_bump_commit: false,
            generate_mono_repository_global_tag: true,
            monorepo_version_separator: None,
            branch_whitelist: vec![],
            tag_prefix: None,
            skip_ci: "[skip ci]".to_string(),
            skip_untracked: false,
            pre_bump_hooks: vec![],
            post_bump_hooks: vec![],
            pre_package_bump_hooks: vec![],
            post_package_bump_hooks: vec![],
            git_hooks: HashMap::new(),
            commit_types: Default::default(),
            changelog: Default::default(),
            bump_profiles: Default::default(),
            packages: Default::default(),
        }
    }
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
    pub fn get<T: TryInto<Settings, Error = SettingError>>(
        repository: T,
    ) -> Result<Self, SettingError> {
        repository.try_into()
    }

    pub fn commit_types(&self) -> HashMap<CommitType, CommitConfig> {
        let commit_settings = self.commit_types.clone();
        let mut custom_types = HashMap::new();

        commit_settings.iter().for_each(|(key, value)| {
            let _ = custom_types.insert(CommitType::from(key.as_str()), value.clone());
        });
        let mut default_types = Settings::default_commit_config();

        default_types.extend(custom_types);

        default_types
            .into_iter()
            .filter_map(|(key, value)| match value {
                CommitConfigOrNull::CommitConfig(config) => Some((key, config)),
                CommitConfigOrNull::None {} => None,
            })
            .collect()
    }

    fn default_commit_config() -> CommitsMetadata {
        let mut default_types = HashMap::new();
        default_types.insert(
            CommitType::Feature,
            CommitConfigOrNull::CommitConfig(CommitConfig::new("Features").with_minor_bump()),
        );
        default_types.insert(
            CommitType::BugFix,
            CommitConfigOrNull::CommitConfig(CommitConfig::new("Bug Fixes").with_patch_bump()),
        );

        default_types.insert(
            CommitType::Chore,
            CommitConfigOrNull::CommitConfig(CommitConfig::new("Miscellaneous Chores")),
        );
        default_types.insert(
            CommitType::Revert,
            CommitConfigOrNull::CommitConfig(CommitConfig::new("Revert")),
        );
        default_types.insert(
            CommitType::Performances,
            CommitConfigOrNull::CommitConfig(CommitConfig::new("Performance Improvements")),
        );
        default_types.insert(
            CommitType::Documentation,
            CommitConfigOrNull::CommitConfig(CommitConfig::new("Documentation")),
        );
        default_types.insert(
            CommitType::Style,
            CommitConfigOrNull::CommitConfig(CommitConfig::new("Style")),
        );
        default_types.insert(
            CommitType::Refactor,
            CommitConfigOrNull::CommitConfig(CommitConfig::new("Refactoring")),
        );
        default_types.insert(
            CommitType::Test,
            CommitConfigOrNull::CommitConfig(CommitConfig::new("Tests")),
        );
        default_types.insert(
            CommitType::Build,
            CommitConfigOrNull::CommitConfig(CommitConfig::new("Build system")),
        );
        default_types.insert(
            CommitType::Ci,
            CommitConfigOrNull::CommitConfig(CommitConfig::new("Continuous Integration")),
        );
        default_types
    }

    pub fn tag_prefix(&self) -> Option<&str> {
        self.tag_prefix.as_deref()
    }

    pub fn monorepo_separator(&self) -> Option<&str> {
        if self.packages.is_empty() {
            None
        } else {
            self.monorepo_version_separator.as_deref().or(Some("-"))
        }
    }

    pub fn package_names(&self) -> impl Iterator<Item = &str> {
        self.packages.keys().map(move |n| n.as_str())
    }

    pub fn package_paths(&self) -> impl Iterator<Item = &Path> {
        self.packages.values().map(|package| package.path.as_path())
    }
}

impl TryFrom<String> for Settings {
    type Error = SettingError;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        if value.is_empty() {
            Ok(Settings::default())
        } else {
            Config::builder()
                .add_source(File::from_str(&value, FileFormat::Toml))
                .build()
                .map_err(SettingError::from)?
                .try_deserialize()
                .map_err(SettingError::from)
        }
    }
}

impl TryFrom<&Path> for Settings {
    type Error = SettingError;

    fn try_from(path: &Path) -> Result<Self, Self::Error> {
        let settings_path = path.join(CONFIG_PATH);
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
}

#[cfg(test)]
mod test {
    use std::fs;

    use cocogitto_test_helpers::git_init_no_gpg;
    use conventional_commit_parser::commit::CommitType;
    use sealed_test::prelude::*;
    use speculoos::prelude::*;

    use crate::COMMITS_METADATA;

    #[sealed_test]
    fn should_disable_default_commit_type() -> anyhow::Result<()> {
        git_init_no_gpg()?;
        let settings = r#"
[commit_types]
feat = {}
"#;
        fs::write("cog.toml", settings)?;
        assert_that!(COMMITS_METADATA.keys()).does_not_contain(&CommitType::Feature);
        assert_that!(COMMITS_METADATA.keys()).contains(&CommitType::BugFix);
        Ok(())
    }
}
