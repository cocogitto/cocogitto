use crate::Settings;
use conventional_commit_parser::commit::CommitType;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Configurations to create new conventional commit types or override behaviors of the existing ones.
#[cfg_attr(feature = "json_schema", derive(schemars::JsonSchema))]
#[derive(Debug, Deserialize, Serialize, Clone, Eq, PartialEq)]
pub struct CommitConfig {
    /// Define the title used in generated changelog for this commit type.
    pub changelog_title: String,
    /// Do not display this commit type in changelogs.
    #[serde(default)]
    pub omit_from_changelog: bool,
    /// Allow for this commit type to bump the minor version.
    #[serde(default)]
    pub bump_minor: bool,
    /// Allow for this commit type to bump the patch version.
    #[serde(default)]
    pub bump_patch: bool,
}

impl Settings {
    pub fn allowed_commit_types(&self) -> Vec<CommitType> {
        self.commit_types().keys().cloned().collect()
    }

    pub fn commit_omitted_from_changelog(&self) -> HashMap<CommitType, bool> {
        self.commit_types()
            .iter()
            .map(|(k, v)| (k.clone(), v.omit_from_changelog))
            .collect()
    }

    pub fn changelog_titles(&self) -> HashMap<CommitType, String> {
        self.commit_types()
            .iter()
            .map(|(k, v)| (k.clone(), v.changelog_title.clone()))
            .collect()
    }

    pub fn is_minor_bump(&self, r#type: &CommitType) -> bool {
        let commit_settings = self.commit_types();
        let Some(commit_config) = commit_settings.get(r#type) else {
            return false;
        };

        commit_config.bump_minor
    }

    pub fn is_patch_bump(&self, r#type: &CommitType) -> bool {
        let commit_settings = self.commit_types();
        let Some(commit_config) = commit_settings.get(r#type) else {
            return false;
        };

        commit_config.bump_patch
    }

    pub fn get_changelog_title(&self, r#type: &CommitType) -> String {
        self.commit_types()
            .iter()
            .find(|(commit_type, _config)| *commit_type == r#type)
            .map(|meta| meta.1.changelog_title.to_string())
            .unwrap_or_else(|| r#type.to_string())
    }

    pub fn commit_usernames(&self) -> HashMap<&str, &str> {
        self.changelog
            .authors
            .iter()
            .map(|author| (author.signature.as_str(), author.username.as_str()))
            .collect()
    }
}

impl CommitConfig {
    pub fn new(changelog_title: &str) -> Self {
        CommitConfig {
            changelog_title: changelog_title.to_string(),
            omit_from_changelog: false,
            bump_minor: false,
            bump_patch: false,
        }
    }

    pub fn with_minor_bump(mut self) -> Self {
        self.bump_minor = true;
        self
    }

    pub fn with_patch_bump(mut self) -> Self {
        self.bump_patch = true;
        self
    }
}
