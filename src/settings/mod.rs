use std::collections::HashMap;
use std::fmt;
use std::fmt::Formatter;
use std::path::PathBuf;

use crate::conventional::changelog::context::RemoteContext;
use crate::conventional::commit::CommitConfig;
use crate::git::repository::Repository;
use crate::{get_config_path, SETTINGS};

use crate::conventional::changelog::error::ChangelogError;
use crate::conventional::changelog::template::Template;
use crate::hook::Hooks;
use crate::settings::error::SettingError;
use config::{Config, File, FileFormat};
use conventional_commit_parser::commit::CommitType;
use maplit::hashmap;
use serde::{Deserialize, Serialize};
use std::path::Path;

pub(crate) type AuthorSettings = Vec<AuthorSetting>;

mod error;

#[derive(Copy, Clone)]
pub enum HookType {
    PreBump,
    PostBump,
}

/// # Settings
/// Configuration structure for the Cocogitto tool.
///
/// This struct defines the main configuration options for Cocogitto, including settings
/// for version generation, changelog handling, commit conventions, hooks, and monorepo support.
///
///  **Example :**
/// ```toml
/// # Basic settings
/// from_latest_tag = true
/// ignore_merge_commits = true
///
/// # Changelog settings
/// [changelog]
/// path = "CHANGELOG.md"
/// template = "remote"
///
/// # Git hooks
/// [git_hooks.pre-commit]
/// script = "./scripts/pre-commit.sh"
///
/// # Monorepo configuration
/// [packages.my-package]
/// path = "packages/my-package"
/// ```
#[cfg_attr(feature = "docgen", derive(cog_schemars::JsonSchema))]
#[derive(Debug, Deserialize, Serialize, Eq, PartialEq)]
#[serde(deny_unknown_fields, default)]
pub struct Settings {
    /// Whether to only consider commits since the latest SemVer tag.
    pub from_latest_tag: bool,
    /// A list of glob patterns to allow bumping only on matching branches.
    pub ignore_merge_commits: bool,
    /// Silently ignore fixup commits
    pub ignore_fixup_commits: bool,
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
    /// Default pre-release pattern to be used when auto-incrementing pre-release versions.
    /// It must contain exactly one wildcard `*` to be replaced by the numeric identifier.
    pub pre_pattern: String,
    /// A "skip-ci" string to add to the commits when using the `bump` or `commit` commands.
    /// Default value is `[skip ci].
    pub skip_ci: String,
    /// Allows to perform bump even if there are untracked or uncommitted changes.
    pub skip_untracked: bool,
    /// Hooks that will be executed before a bump command in root dir.
    pub pre_bump_hooks: Vec<String>,
    /// Hooks that will be executed after a bump command in root dir.
    pub post_bump_hooks: Vec<String>,
    /// Hooks that will be executed before a bump command in package dir.
    pub pre_package_bump_hooks: Vec<String>,
    /// Hooks that will be executed after a bump command in package dir.
    pub post_package_bump_hooks: Vec<String>,
    /// Git hooks configuration.
    pub git_hooks: HashMap<GitHookType, GitHook>,
    /// Custom commit types configuration.
    // Note: the custom serde deserializer is needed to be able to serialize from an empty object `{}`
    // to disable default commit. This translates to `Option<CommitConfig>` in the schemar doc generator.
    pub commit_types: HashMap<String, CommitConfig>,
    /// Changelog configuration.
    pub changelog: Changelog,
    /// Custom bump profiles configurations.
    pub bump_profiles: HashMap<String, BumpProfile>,
    /// Monorepo packages configuration.
    pub packages: HashMap<String, MonoRepoPackage>,
    /// List of valid commit scopes.
    pub scopes: Option<Vec<String>>,
}

impl Default for Settings {
    fn default() -> Self {
        Self {
            from_latest_tag: false,
            ignore_merge_commits: false,
            ignore_fixup_commits: false,
            disable_changelog: false,
            disable_bump_commit: false,
            generate_mono_repository_global_tag: true,
            generate_mono_repository_package_tags: true,
            monorepo_version_separator: None,
            branch_whitelist: vec![],
            tag_prefix: None,
            pre_pattern: "alpha.*".to_string(),
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

/// # GitHookType
/// Represents the different types of Git hooks that can be configured.
///
/// This enum defines all the standard Git hook types that can be used
/// in the configuration. Each variant corresponds to a specific Git hook
/// that gets triggered at different points in Git's execution.
///
///  **Example :**
/// ```toml
/// [git_hooks.pre-commit]
/// script = "./scripts/pre-commit.sh"
/// ```
#[cfg_attr(feature = "docgen", derive(cog_schemars::JsonSchema))]
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
        write!(f, "{value}")
    }
}

/// # GitHook
/// A GitHook can be defined either as a script string that will be executed directly,
/// or as a path to a script file that will be executed
#[cfg_attr(feature = "docgen", derive(cog_schemars::JsonSchema))]
#[derive(Debug, Deserialize, Serialize, Eq, PartialEq)]
#[serde(deny_unknown_fields, untagged)]
pub enum GitHook {
    /// Direct script string that will be executed
    Script { script: String },
    /// Path to a script file that will be executed
    File { path: PathBuf },
}

/// # MonoRepoPackage
/// Configuration for a package in a monorepo setup.
///
/// This struct defines how a single package within a monorepo should be handled,
/// including its location, included/excluded files, changelog settings, and bump behavior.
///
///  **Example :**
/// ```toml
/// [packages.my-package]
/// path = "packages/my-package"
/// include = ["packages/my-package/**"]
/// ignore = ["**/test/**"]
/// changelog_path = "CHANGELOG.md"
/// public_api = true
/// bump_order = 1
/// ```
#[cfg_attr(feature = "docgen", derive(cog_schemars::JsonSchema))]
#[derive(Debug, Deserialize, Serialize, Eq, PartialEq)]
#[serde(deny_unknown_fields, default)]
pub struct MonoRepoPackage {
    /// The package path, relative to the repository root dir.
    /// Used to scan commits and set hook commands current directory.
    pub path: PathBuf,
    /// List of globs for additional paths to include, relative to
    /// the repository root dir.
    pub include: Vec<String>,
    /// List of globs for paths to ignore, relative to
    /// the repository root dir.
    pub ignore: Vec<String>,
    /// Where to write the changelog.
    pub changelog_path: Option<String>,
    /// Bumping package marked as public api will increment
    /// the global monorepo version when using `cog bump --auto`.
    pub public_api: bool,
    /// Ordering of packages in the changelog, this affect in which order
    /// packages will be bumped.
    pub bump_order: Option<usize>,
    /// Overrides `pre_package_bump_hooks`.
    pub pre_bump_hooks: Option<Vec<String>>,
    /// Overrides `post_package_bump_hooks`.
    pub post_bump_hooks: Option<Vec<String>>,
    /// Custom profile to override `pre_bump_hooks`, `post_bump_hooks`.
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
            bump_order: None,
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
            bump_order: None,
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
/// Configuration for changelog generation.
///
/// This struct defines how the changelog should be generated,
/// including templates, remote repository information, and author settings.
///
///  **Example :**
/// ```toml
/// [changelog]
/// template = "remote"
/// path = "CHANGELOG.md"
/// remote = "github.com"
/// owner = "cocogitto"
/// repository = "cocogitto"
/// ```
#[cfg_attr(feature = "docgen", derive(cog_schemars::JsonSchema))]
#[derive(Debug, Deserialize, Serialize, Clone, Eq, PartialEq)]
#[serde(deny_unknown_fields, default)]
pub struct Changelog {
    /// Template to use for changelog generation. Can be "remote", "full_hash" or a custom template path
    pub template: Option<String>,
    /// Template to use for package changelogs in monorepos
    pub package_template: Option<String>,
    /// Remote Git repository URL (e.g. "github.com")
    pub remote: Option<String>,
    /// Path where changelog file should be written
    pub path: PathBuf,
    /// Repository owner/organization name
    pub owner: Option<String>,
    /// Repository name
    pub repository: Option<String>,
    /// Author mappings for changelog generation
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

/// # AuthorSetting
/// Configuration for mapping Git signatures to usernames.
///
/// This struct defines the mapping between a Git commit signature (email address)
/// and the corresponding username to use in changelog generation.
///
///  **Example :**
/// ```toml
/// [[changelog.authors]]
/// signature = "user@example.com"
/// username = "githubuser"
/// ```
#[cfg_attr(feature = "docgen", derive(cog_schemars::JsonSchema))]
#[derive(Debug, Deserialize, Serialize, Clone, Eq, PartialEq)]
#[serde(deny_unknown_fields)]
pub struct AuthorSetting {
    /// The Git commit signature (typically an email address)
    pub signature: String,
    /// The username to display in changelogs
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

/// # BumpProfile
/// A custom profile for configuring hooks that run before and after version bumps.
///
/// Bump profiles allow defining different sets of hooks that can be selected
/// when running bump commands.
///
///  **Example :**
/// ```toml
/// [bump_profiles.production]
/// pre_bump_hooks = ["./scripts/pre-release.sh"]
/// post_bump_hooks = ["./scripts/post-release.sh"]
/// ```
#[cfg_attr(feature = "docgen", derive(cog_schemars::JsonSchema))]
#[derive(Debug, Deserialize, Serialize, Default, Eq, PartialEq)]
#[serde(deny_unknown_fields)]
pub struct BumpProfile {
    /// List of hooks to run before bumping the version
    #[serde(default)]
    pub pre_bump_hooks: Vec<String>,
    /// List of hooks to run after bumping the version
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

    pub fn load_commit_types(&self) -> HashMap<CommitType, CommitConfig> {
        let commit_settings = self.commit_types.clone();
        let mut custom_types = HashMap::new();

        commit_settings.iter().for_each(|(key, value)| {
            let _ = custom_types.insert(CommitType::from(key.as_str()), value.clone());
        });

        let mut default_types: HashMap<CommitType, CommitConfig> =
            Settings::default_commit_config()
                .into_iter()
                .map(|(key, config)| {
                    if let Some(custom_config) = custom_types.remove(&key) {
                        let config = config.merge(custom_config);
                        (key, config)
                    } else {
                        (key, config)
                    }
                })
                .collect();

        default_types.extend(custom_types);
        default_types
            .into_iter()
            .filter(|(_, config)| !config.none())
            .collect()
    }

    pub fn commit_scopes(&self) -> Option<Vec<String>> {
        self.scopes.clone()
    }

    fn default_commit_config() -> HashMap<CommitType, CommitConfig> {
        hashmap! {
            CommitType::Feature => CommitConfig::new("Features").with_minor_bump().with_order(1),
            CommitType::BugFix => CommitConfig::new("Bug Fixes").with_patch_bump().with_order(2),
            CommitType::Performances => CommitConfig::new("Performance Improvements").with_order(3),
            CommitType::Revert => CommitConfig::new("Revert").with_order(4),
            CommitType::Documentation => CommitConfig::new("Documentation").with_order(5),
            CommitType::Test => CommitConfig::new("Tests").with_order(6),
            CommitType::Build => CommitConfig::new("Build system").with_order(7),
            CommitType::Ci => CommitConfig::new("Continuous Integration").with_order(8),
            CommitType::Refactor => CommitConfig::new("Refactoring").with_order(9),
            CommitType::Chore => CommitConfig::new("Miscellaneous Chores").with_order(10),
            CommitType::Style => CommitConfig::new("Style").with_order(11),
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

        // TODO: there should be a unified settings
        Template::from_arg(template, context, false)
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

        // TODO: there should be a unified settings
        Template::from_arg(template, context, false)
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

        // TODO: there should be a unified settings
        Template::from_arg(template, context, false)
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
                let settings_path = repo_path.join(get_config_path());
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
