use crate::conventional::commit::CommitConfig;
use crate::git::repository::Repository;
use crate::{CommitsMetadata, CONFIG_PATH};
use anyhow::Result;
use config::{Config, File};
use conventional_commit_parser::commit::CommitType;
use std::collections::HashMap;
use std::fmt::Debug;
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
    /// Relative path to the repository changelog
    /// Default: CHANGELOG.md
    pub changelog_path: Option<PathBuf>,
    ///
    pub github: Option<String>,
    pub repository: Option<RepositorySettings>,
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

/// This is user to set how markdown changelog links will be generated. You need to Set this only
/// if you repo is not hosted on github.
#[derive(Debug, Deserialize, Serialize)]
pub struct RepositorySettings {
    /// The base url of the web platform hosting the repository (ex : `https://gitlab.com`)
    pub host: String,
    /// Name of the repository owner (ex : "DeveloperC")
    pub owner: String,
    /// Name of the repository on the target platform , (ex: `conventional_commits_next_version`)
    pub repository: String,
    /// A URL template representing a specific commit at a hash.
    /// Default : "{{host}}/{{owner}}/{{repository}}/commit/{{hash}}"
    pub commit_url_format: String,
    /// `{{host}}/{{owner}}/{{repository}}/compare/{{previous_tag}}...{{current_tag}}`
    pub compare_url_format: String,
    /// A URL representing the issue format (allowing a different URL format to be swapped in for Gitlab, Bitbucket, etc).
    /// Default:  `{{host}}/{{owner}}/{{repository}}/issues/{{id}}`
    pub issue_url_format: String,
    /// A URL representing the a user's profile URL on GitHub, Gitlab, etc. This URL is used for substituting @bcoe with https://github.com/bcoe in commit messages.
    /// Default : `{{host}}/{{user}}`
    pub user_url_format: String,
    /// A string to be used to format the auto-generated release commit message.
    /// Default : `chore(version): {{version}}`
    pub release_commit_message_format: String,
    /// An array of prefixes used to detect references to issues
    /// Default: "#"
    pub issue_prefix: Vec<String>,
}

impl Default for RepositorySettings {
    fn default() -> Self {
        RepositorySettings {
            host: "".to_string(),
            owner: "".to_string(),
            repository: "".to_string(),
            commit_url_format: "{{host}}/{{owner}}/{{repository}}/commit/{{hash}}".to_string(),
            compare_url_format:
                "{{host}}/{{owner}}/{{repository}}/compare/{{previous_tag}}...{{current_tag}}"
                    .to_string(),
            issue_url_format: "{{host}}/{{owner}}/{{repository}}/issues/{{id}}".to_string(),
            user_url_format: "{{host}}/{{user}".to_string(),
            release_commit_message_format: "chore(version): {{version}}".to_string(),
            issue_prefix: vec![],
        }
    }
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
            github: Default::default(),
            repository: None,
            pre_bump_hooks: Default::default(),
            post_bump_hooks: Default::default(),
            authors: Default::default(),
            commit_types: Default::default(),
            bump_profiles: Default::default(),
        }
    }
}

#[derive(Debug, Deserialize, Serialize, Default)]
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
        default_types.insert(CommitType::Refactor, CommitConfig::new("Refactoring"));
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
