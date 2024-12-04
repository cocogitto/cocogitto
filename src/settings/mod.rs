use std::collections::HashMap;
use std::fmt;
use std::fmt::Formatter;
use std::path::PathBuf;

use crate::conventional::commit::CommitConfig;
use crate::git::repository::Repository;
use crate::{CommitConfigOrNull, CommitsMetadata, CONFIG_PATH, SETTINGS};

use crate::conventional::changelog::error::ChangelogError;
use crate::conventional::changelog::template::{RemoteContext, Template};
use crate::hook::Hooks;
use crate::settings::error::SettingError;
use config::{Config, File, FileFormat};
use conventional_commit_parser::commit::CommitType;
use serde::{Deserialize, Serialize};
use std::path::Path;

pub(crate) type AuthorSettings = Vec<AuthorSetting>;

mod error;

#[cfg_attr(feature = "docgen", derive(schemars::JsonSchema))]
#[derive(Copy, Clone)]
pub enum HookType {
    PreBump,
    PostBump,
}

/// # Cocogitto config
#[cfg_attr(feature = "docgen", derive(schemars::JsonSchema))]
#[derive(Debug, Deserialize, Serialize, Eq, PartialEq)]
#[serde(deny_unknown_fields, default)]
pub struct Settings {
    /// Whether to only consider commits since the latest SemVer tag.
    pub from_latest_tag: bool,
    /// A list of glob patterns to allow bumping only on matching branches.
    pub ignore_merge_commits: bool,
    /// Whether to generate a changelog or not during bump.
    pub disable_changelog: bool,
    /// Whether to create a bump commit or not.
    pub disable_bump_commit: bool,
    /// Activate or deactivate global tag generation for mono-repository.
    pub generate_mono_repository_global_tag: bool,
    /// Activate or deactivate package tag generation for mono-repository.
    pub generate_mono_repository_package_tags: bool,
    /// Specify the version separator character for mono-repository package's tags.
    pub monorepo_version_separator: Option<String>,
    /// A list of glob patterns to allow bumping only on matching branches.
    pub branch_whitelist: Vec<String>,
    /// Set a tag prefix value for cocogitto. For instance if you have a `v`
    /// as a tag prefix, cocogitto will generate versions starting with `v` and
    /// commands like `cog changelog` will pick only those versions.
    pub tag_prefix: Option<String>,
    /// A "skip-ci" string to add to the commits when using the `bump` or `commit commands.
    /// Default value is `[skip ci].
    pub skip_ci: String,
    /// Allows to perform bump even if there are untracked or uncommited changes.
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
    pub scopes: Vec<String>,
}

impl Default for Settings {
    fn default() -> Self {
        Self {
            from_latest_tag: false,
            ignore_merge_commits: false,
            disable_changelog: false,
            disable_bump_commit: false,
            generate_mono_repository_global_tag: true,
            generate_mono_repository_package_tags: true,
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
            scopes: Default::default(),
        }
    }
}

#[cfg_attr(feature = "docgen", derive(schemars::JsonSchema))]
#[derive(Debug, Deserialize, Serialize, Eq, PartialEq, Hash, Copy, Clone)]
#[serde(deny_unknown_fields, rename_all = "kebab-case", into = "&str")]
pub enum GitHookType {
    ApplypatchMsg,
    PreApplypatch,
    PostApplypatch,
    PreCommit,
    PreMergeCommit,
    PrePrepareCommitMsg,
    CommitMsg,
    PostCommit,
    PreRebase,
    PostCheckout,
    PostMerge,
    PrePush,
    PreAutoGc,
    PostRewrite,
    SendemailValidate,
    FsmonitorWatchman,
    P4Changelist,
    P4PrepareChangelist,
    P4Postchangelist,
    P4PreSubmit,
    PostIndexChange,
}

impl From<String> for GitHookType {
    fn from(value: String) -> Self {
        match value.as_str() {
            "applypatch-msg" => Self::ApplypatchMsg,
            "pre-applypatch" => Self::PreApplypatch,
            "post-applypatch" => Self::PostApplypatch,
            "pre-commit" => Self::PreCommit,
            "pre-merge-commit" => Self::PreMergeCommit,
            "pre-commit-msg" => Self::PrePrepareCommitMsg,
            "commit-msg" => Self::CommitMsg,
            "post-commit" => Self::PostCommit,
            "pre-rebase" => Self::PreRebase,
            "post-checkout" => Self::PostCheckout,
            "post-merge" => Self::PostMerge,
            "pre-push" => Self::PrePush,
            "pre-auto-gc" => Self::PreAutoGc,
            "post-rewrite" => Self::PostRewrite,
            "sendemail-validate" => Self::SendemailValidate,
            "fsmonitor-watchman" => Self::FsmonitorWatchman,
            "p4-changelist" => Self::P4Changelist,
            "p4-prepare-changelist" => Self::P4PrepareChangelist,
            "p4-postchangelist" => Self::P4Postchangelist,
            "p4-pre-submit" => Self::P4PreSubmit,
            "post-index-change" => Self::PostIndexChange,
            _ => unreachable!(),
        }
    }
}

impl From<GitHookType> for &str {
    fn from(val: GitHookType) -> Self {
        match val {
            GitHookType::ApplypatchMsg => "applypatch-msg",
            GitHookType::PreApplypatch => "pre-applypatch",
            GitHookType::PostApplypatch => "post-applypatch",
            GitHookType::PreCommit => "pre-commit",
            GitHookType::PreMergeCommit => "pre-merge-commit",
            GitHookType::PrePrepareCommitMsg => "pre-commit-msg",
            GitHookType::CommitMsg => "commit-msg",
            GitHookType::PostCommit => "post-commit",
            GitHookType::PreRebase => "pre-rebase",
            GitHookType::PostCheckout => "post-checkout",
            GitHookType::PostMerge => "post-merge",
            GitHookType::PrePush => "pre-push",
            GitHookType::PreAutoGc => "pre-auto-gc",
            GitHookType::PostRewrite => "post-rewrite",
            GitHookType::SendemailValidate => "sendemail-validate",
            GitHookType::FsmonitorWatchman => "fsmonitor-watchman",
            GitHookType::P4Changelist => "p4-changelist",
            GitHookType::P4PrepareChangelist => "p4-prepare-changelist",
            GitHookType::P4Postchangelist => "p4-postchangelist",
            GitHookType::P4PreSubmit => "p4-pre-submit",
            GitHookType::PostIndexChange => "post-index-change",
        }
    }
}

impl fmt::Display for GitHookType {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        let value: &str = (*self).into();
        write!(f, "{}", value)
    }
}

#[cfg_attr(feature = "docgen", derive(schemars::JsonSchema))]
#[derive(Debug, Deserialize, Serialize, Eq, PartialEq)]
#[serde(deny_unknown_fields, untagged)]
pub enum GitHook {
    Script { script: String },
    File { path: PathBuf },
}

#[cfg_attr(feature = "docgen", derive(schemars::JsonSchema))]
#[derive(Debug, Deserialize, Serialize, Eq, PartialEq)]
#[serde(deny_unknown_fields, default)]
pub struct MonoRepoPackage {
    /// The package path, relative to the repository root dir.
    /// Used to scan commits and set hook commands current directory
    pub path: PathBuf,
    /// List of globs for additional paths to include, relative to
    /// the repository root dir.
    pub include: Vec<String>,
    /// List of globs for paths to ignore, relative to
    /// the repository root dir.
    pub ignore: Vec<String>,
    /// Where to write the changelog
    pub changelog_path: Option<String>,
    /// Bumping package marked as public api will increment
    /// the global monorepo version when using `cog bump --auto`
    pub public_api: bool,
    /// Overrides `pre_package_bump_hooks`
    pub pre_bump_hooks: Option<Vec<String>>,
    /// Overrides `post_package_bump_hooks`
    pub post_bump_hooks: Option<Vec<String>>,
    /// Custom profile to override `pre_bump_hooks`, `post_bump_hooks`
    pub bump_profiles: HashMap<String, BumpProfile>,
}

impl Default for &MonoRepoPackage {
    fn default() -> Self {
        let package = Box::new(MonoRepoPackage {
            path: Default::default(),
            include: vec![],
            ignore: vec![],
            changelog_path: None,
            pre_bump_hooks: None,
            post_bump_hooks: None,
            bump_profiles: Default::default(),
            public_api: true,
        });

        Box::leak(package)
    }
}

impl Default for MonoRepoPackage {
    fn default() -> Self {
        Self {
            path: Default::default(),
            include: vec![],
            ignore: vec![],
            changelog_path: None,
            pre_bump_hooks: None,
            post_bump_hooks: None,
            bump_profiles: Default::default(),
            public_api: true,
        }
    }
}

impl MonoRepoPackage {
    pub fn changelog_path(&self) -> PathBuf {
        self.changelog_path
            .as_ref()
            .map(PathBuf::from)
            .unwrap_or_else(|| self.path.join("CHANGELOG.md"))
    }
}

/// # Changelog
#[cfg_attr(feature = "docgen", derive(schemars::JsonSchema))]
#[derive(Debug, Deserialize, Serialize, Clone, Eq, PartialEq)]
#[serde(deny_unknown_fields, default)]
pub struct Changelog {
    pub template: Option<String>,
    pub package_template: Option<String>,
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
            package_template: None,
            remote: None,
            path: PathBuf::from("CHANGELOG.md"),
            owner: None,
            repository: None,
            authors: vec![],
        }
    }
}

#[cfg_attr(feature = "docgen", derive(schemars::JsonSchema))]
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

/// # Bump profile
#[cfg_attr(feature = "docgen", derive(schemars::JsonSchema))]
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
    pub(crate) fn get<T: TryInto<Settings, Error = SettingError>>(
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

    pub fn commit_scopes(&self) -> Option<Vec<String>> {
        if self.scopes.is_empty() {
            return None;
        };

        Some(self.scopes.clone())
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

    pub fn get_package_changelog_template(&self) -> Result<Template, ChangelogError> {
        let context = self.get_template_context();
        let template = self
            .changelog
            .package_template
            .as_deref()
            .unwrap_or("package_default");

        let template = match template {
            "remote" => "package_remote",
            "full_hash" => "package_full_hash",
            template => template,
        };

        Template::from_arg(template, context)
    }

    pub fn get_monorepo_changelog_template(&self) -> Result<Template, ChangelogError> {
        let context = self.get_template_context();
        let template = self
            .changelog
            .template
            .as_deref()
            .unwrap_or("monorepo_default");

        let template = match template {
            "remote" => "monorepo_remote",
            "full_hash" => "monorepo_full_hash",
            template => template,
        };

        Template::from_arg(template, context)
    }

    pub fn monorepo_separator(&self) -> Option<&str> {
        if self.packages.is_empty() {
            None
        } else {
            self.monorepo_version_separator.as_deref().or(Some("-"))
        }
    }

    pub fn package_paths(&self) -> impl Iterator<Item = &Path> {
        self.packages.values().map(|package| package.path.as_path())
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
        self.pre_bump_hooks
            .as_ref()
            .unwrap_or(&SETTINGS.pre_package_bump_hooks)
    }

    fn post_bump_hooks(&self) -> &Vec<String> {
        self.post_bump_hooks
            .as_ref()
            .unwrap_or(&SETTINGS.post_package_bump_hooks)
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

impl TryFrom<&Repository> for Settings {
    type Error = SettingError;

    fn try_from(repo: &Repository) -> Result<Self, Self::Error> {
        match repo.get_repo_dir() {
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
}

#[cfg(test)]
mod test {
    use std::fs;

    use conventional_commit_parser::commit::CommitType;
    use sealed_test::prelude::*;
    use speculoos::prelude::*;

    use crate::{test_helpers::git_init_no_gpg, COMMITS_METADATA};

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
