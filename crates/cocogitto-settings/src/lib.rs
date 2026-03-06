//! Cocogitto Settings
//!
//! This crate provides configuration management for the Cocogitto tool.
//! It handles loading, parsing, and providing access to Cocogitto's configuration
//! from `cog.toml` files, including settings for conventional commits, version bumping,
//! changelog generation, hooks, and monorepo support.
#![deny(missing_docs)]

use std::collections::HashMap;
use std::fmt;
use std::fmt::Formatter;
use std::path::PathBuf;

use crate::error::SettingError;
use config::{Config, File, FileFormat};
use conventional_commit_parser::commit::CommitType;
use git2::Repository;
use log::warn;
use maplit::hashmap;
use once_cell::sync::Lazy;
use serde::{Deserialize, Serialize};
use std::path::Path;
use std::sync::OnceLock;

pub(crate) type AuthorSettings = Vec<AuthorSetting>;

mod error;
mod hooks;
pub use hooks::Hooks;

/// Default path for the Cocogitto configuration file
pub const DEFAULT_CONFIG_PATH: &str = "cog.toml";
static CONFIG_PATH: OnceLock<String> = OnceLock::new();

/// Gets the current configuration file path
///
/// # Returns
///
/// * `&'static String` - The path to the configuration file
pub fn get_config_path() -> &'static String {
    CONFIG_PATH.get_or_init(|| DEFAULT_CONFIG_PATH.to_owned())
}

/// Sets the configuration file path
///
/// # Arguments
///
/// * `path` - The path to set as the configuration file
pub fn set_config_path(path: String) {
    CONFIG_PATH
        .set(path)
        .expect("config path should not be set");
}

/// Global settings instance that loads configuration from the current directory
///
/// This lazy-initialized static loads settings once and caches them for the lifetime of the program.
pub static SETTINGS: Lazy<Settings> = Lazy::new(|| {
    let settings = Settings::try_from_path(".");
    if let Err(err) = settings.as_ref() {
        warn!("Failed to get config, falling back to default: {err}");
    }

    settings.unwrap_or_default()
});

// This cannot be carried by `Cocogitto` struct since we need it to be available in `Changelog`,
// `Commit` etc. Be sure that `CocoGitto::new` is called before using this  in order to bypass
// unwrapping in case of error.

/// Global commit metadata map that contains configurations for all commit types
///
/// This is loaded from the settings and contains merged default and custom commit configurations.
pub static COMMITS_METADATA: Lazy<HashMap<CommitType, CommitConfig>> =
    Lazy::new(|| SETTINGS.load_commit_types());

/// # HookType
/// Represents the type of hook that can be executed during version bumping.
///
/// This enum defines the different types of hooks that can be configured
/// to run at specific points during the version bumping process.
#[derive(Copy, Clone)]
pub enum HookType {
    /// Hooks that run before the version bump
    PreBump,
    /// Hooks that run after the version bump
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
/// # MonorepoConfig
/// Configuration for monorepo support including packages and dependency resolver.
///
/// This struct defines the monorepo configuration including all packages
/// and a single dependency resolver to use for determining package bump order.
///
///  **Example :**
/// ```toml
/// [monorepo]
/// resolver = "Cargo"
///
/// [monorepo.packages.my-package]
/// path = "packages/my-package"
/// ```
#[cfg_attr(feature = "docgen", derive(cog_schemars::JsonSchema))]
#[derive(Debug, Deserialize, Serialize, Eq, PartialEq, Default)]
#[serde(deny_unknown_fields, default)]
pub struct MonorepoConfig {
    /// Dependency resolver to use for determining package bump order.
    pub resolver: Option<String>,
    /// Monorepo packages configuration.
    pub packages: HashMap<String, MonoRepoPackage>,
}

#[cfg_attr(feature = "docgen", derive(cog_schemars::JsonSchema))]
#[derive(Debug, Deserialize, Serialize, Eq, PartialEq)]
#[serde(deny_unknown_fields, default)]
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
    pub pre: String,
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
    /// Monorepo configuration.
    pub monorepo: Option<MonorepoConfig>,
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
            pre: "alpha.*".to_string(),
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
            monorepo: Default::default(),
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
    /// Hook that is invoked by `git-am`.
    ApplypatchMsg,
    /// Hook that is invoked by `git-am`.
    PreApplypatch,
    /// Hook that is invoked by `git-am`.
    PostApplypatch,
    /// Hook that is invoked by `git-commit`.
    PreCommit,
    /// Hook that is invoked by `git-merge`.
    PreMergeCommit,
    /// Hook that is invoked by `git-commit`.
    PrePrepareCommitMsg,
    /// Hook that is invoked by `git-commit`.
    CommitMsg,
    /// Hook that is invoked by `git-commit`.
    PostCommit,
    /// Hook that is invoked by `git-rebase`.
    PreRebase,
    /// Hook that is invoked by `git-checkout`.
    PostCheckout,
    /// Hook that is invoked by `git-merge`.
    PostMerge,
    /// Hook that is invoked by `git-push`.
    PrePush,
    /// Hook that is invoked by `git-gc`.
    PreAutoGc,
    /// Hook that is invoked by commands that rewrite commits.
    PostRewrite,
    /// Hook that is invoked by `git-send-email`.
    SendemailValidate,
    /// Hook that is invoked by `git-fsmonitor--daemon`.
    FsmonitorWatchman,
    /// Hook that is invoked by `git-p4`.
    P4Changelist,
    /// Hook that is invoked by `git-p4`.
    P4PrepareChangelist,
    /// Hook that is invoked by `git-p4`.
    P4Postchangelist,
    /// Hook that is invoked by `git-p4`.
    P4PreSubmit,
    /// Hook that is invoked by `git-update-index`.
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
    Script {
        /// The script content to execute
        script: String,
    },
    /// Path to a script file that will be executed
    File {
        /// The path to the script file
        path: PathBuf,
    },
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
    /// Returns the path to the changelog file for this package.
    ///
    /// If a custom changelog path is configured, it returns that path.
    /// Otherwise, it returns the default path: `<package_path>/CHANGELOG.md`.
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

/// Looks up the username for a given Git commit author signature.
///
/// This function searches through the configured author mappings to find
/// a match for the given author signature (typically an email address).
///
/// # Arguments
///
/// * `author` - The Git commit signature (email address) to look up
///
/// # Returns
///
/// * `Some(&str)` - The corresponding username if found
/// * `None` - If no mapping is found for the given signature
pub fn commit_username(author: &str) -> Option<&'static str> {
    SETTINGS
        .changelog
        .authors
        .iter()
        .find(|author_map| author_map.signature == author)
        .map(|author| author.username.as_str())
}

/// Returns the path to the changelog file as configured in settings.
///
/// # Returns
///
/// * `&'static PathBuf` - The path to the changelog file
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

/// # CommitConfig
/// Configurations to create new conventional commit types or override behaviors of the existing ones.
#[cfg_attr(feature = "docgen", derive(cog_schemars::JsonSchema))]
#[derive(Debug, Deserialize, Serialize, Clone, Eq, PartialEq)]
pub struct CommitConfig {
    /// Define the title used in generated changelog for this commit type.
    pub changelog_title: Option<String>,
    /// Do not display this commit type in changelogs.
    #[serde(default)]
    pub omit_from_changelog: Option<bool>,
    /// Allow for this commit type to bump the minor version.
    #[serde(default)]
    pub bump_minor: Option<bool>,
    /// Allow for this commit type to bump the patch version.
    #[serde(default)]
    pub bump_patch: Option<bool>,
    /// Specify a sort order attribute for this commit type.
    #[serde(default)]
    pub order: Option<u32>,
}

impl CommitConfig {
    /// Creates a new CommitConfig with the specified changelog title.
    ///
    /// This initializes a commit configuration with default values for all fields
    /// except the changelog title, which is set to the provided value.
    ///
    /// # Arguments
    ///
    /// * `changelog_title` - The title to use in generated changelogs for this commit type
    ///
    /// # Returns
    ///
    /// * `Self` - A new CommitConfig instance
    ///
    /// # Example
    ///
    /// ```rust
    /// use cocogitto_settings::CommitConfig;
    /// let config = CommitConfig::new("Features");
    /// ```
    pub fn new(changelog_title: &str) -> Self {
        CommitConfig {
            changelog_title: Some(changelog_title.to_string()),
            omit_from_changelog: Some(false),
            bump_minor: Some(false),
            bump_patch: Some(false),
            order: Some(0),
        }
    }

    /// Merges this CommitConfig with another, giving priority to the other config's values.
    ///
    /// This method combines two commit configurations, using values from the `other` config
    /// where they are defined, and falling back to values from `self` where they are not.
    ///
    /// # Arguments
    ///
    /// * `other` - The CommitConfig to merge with this one
    ///
    /// # Returns
    ///
    /// * `CommitConfig` - A new CommitConfig containing the merged configuration
    ///
    /// # Example
    ///
    /// ```rust
    /// use cocogitto_settings::CommitConfig;
    /// let config1 = CommitConfig::new("Features").with_minor_bump();
    /// let config2 = CommitConfig::new("Enhancements").with_patch_bump();
    /// let merged = config1.merge(config2);
    /// ```
    pub fn merge(self, other: CommitConfig) -> CommitConfig {
        if other.none() {
            return other;
        }

        CommitConfig {
            changelog_title: other.changelog_title.or(self.changelog_title),
            omit_from_changelog: other.omit_from_changelog.or(self.omit_from_changelog),
            bump_minor: other.bump_minor.or(self.bump_minor),
            bump_patch: other.bump_patch.or(self.bump_patch),
            order: other.order.or(self.order),
        }
    }

    /// Configures this commit type to bump the minor version.
    ///
    /// This sets the `bump_minor` field to `true`, indicating that commits
    /// of this type should trigger a minor version bump.
    ///
    /// # Returns
    ///
    /// * `Self` - The modified CommitConfig instance (builder pattern)
    ///
    /// # Example
    ///
    /// ```rust
    /// use cocogitto_settings::CommitConfig;
    /// let config = CommitConfig::new("Features").with_minor_bump();
    /// ```
    pub fn with_minor_bump(mut self) -> Self {
        self.bump_minor = Some(true);
        self
    }

    /// Configures this commit type to bump the patch version.
    ///
    /// This sets the `bump_patch` field to `true`, indicating that commits
    /// of this type should trigger a patch version bump.
    ///
    /// # Returns
    ///
    /// * `Self` - The modified CommitConfig instance (builder pattern)
    ///
    /// # Example
    ///
    /// ```rust
    /// use cocogitto_settings::CommitConfig;
    /// let config = CommitConfig::new("Bug Fixes").with_patch_bump();
    /// ```
    pub fn with_patch_bump(mut self) -> Self {
        self.bump_patch = Some(true);
        self
    }

    /// Sets the sort order for this commit type.
    ///
    /// This sets the `order` field to the specified value, which determines
    /// the display order of this commit type in generated changelogs.
    ///
    /// # Arguments
    ///
    /// * `order` - The sort order value (lower values appear first)
    ///
    /// # Returns
    ///
    /// * `Self` - The modified CommitConfig instance (builder pattern)
    ///
    /// # Example
    ///
    /// ```rust
    /// use cocogitto_settings::CommitConfig;
    /// let config = CommitConfig::new("Features").with_order(1);
    /// ```
    pub fn with_order(mut self, order: u32) -> Self {
        self.order = Some(order);
        self
    }

    /// Returns whether this commit type should be omitted from changelogs.
    ///
    /// # Returns
    ///
    /// * `bool` - `true` if this commit type should be omitted, `false` otherwise
    ///
    /// # Example
    ///
    /// ```rust
    /// use cocogitto_settings::CommitConfig;
    /// let config = CommitConfig::new("Internal");
    /// if config.omit_from_changelog() {
    ///     println!("This commit type is hidden from changelogs");
    /// }
    /// ```
    pub fn omit_from_changelog(&self) -> bool {
        self.omit_from_changelog.unwrap_or_default()
    }

    /// Returns whether this commit type should bump the minor version.
    ///
    /// # Returns
    ///
    /// * `bool` - `true` if this commit type triggers minor version bumps, `false` otherwise
    ///
    /// # Example
    ///
    /// ```rust
    /// use cocogitto_settings::CommitConfig;
    /// let config = CommitConfig::new("Features").with_minor_bump();
    /// if config.bump_minor() {
    ///     println!("This commit type bumps minor versions");
    /// }
    /// ```
    pub fn bump_minor(&self) -> bool {
        self.bump_minor.unwrap_or_default()
    }

    /// Returns whether this commit type should bump the patch version.
    ///
    /// # Returns
    ///
    /// * `bool` - `true` if this commit type triggers patch version bumps, `false` otherwise
    ///
    /// # Example
    ///
    /// ```rust
    /// use cocogitto_settings::CommitConfig;
    /// let config = CommitConfig::new("Bug Fixes").with_patch_bump();
    /// if config.bump_patch() {
    ///     println!("This commit type bumps patch versions");
    /// }
    /// ```
    pub fn bump_patch(&self) -> bool {
        self.bump_patch.unwrap_or_default()
    }

    /// Returns whether this commit configuration has no values set.
    ///
    /// This method checks if all optional fields are `None`, indicating
    /// that this configuration is effectively empty.
    ///
    /// # Returns
    ///
    /// * `bool` - `true` if all fields are `None`, `false` otherwise
    ///
    /// # Example
    ///
    /// ```rust
    /// use cocogitto_settings::CommitConfig;
    /// let config = CommitConfig::new("");
    /// if config.none() {
    ///     println!("This configuration is empty");
    /// }
    /// ```
    pub fn none(&self) -> bool {
        self.bump_patch.is_none()
            && self.omit_from_changelog.is_none()
            && self.bump_minor.is_none()
            && self.changelog_title.is_none()
    }
}

impl Settings {
    /// Attempts to load settings from a configuration file in the given path.
    ///
    /// This method discovers the Git repository at the given path and looks for
    /// a configuration file (default: `cog.toml`). If the file doesn't exist,
    /// it returns default settings.
    ///
    /// # Arguments
    ///
    /// * `path` - The path to search for the configuration file
    ///
    /// # Returns
    ///
    /// * `Result<Self, SettingError>` - The loaded settings or an error
    pub fn try_from_path<P: AsRef<Path>>(path: P) -> Result<Self, SettingError> {
        let repository = Repository::discover(path)?;

        match repository.workdir() {
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

    /// Loads and merges commit types configuration.
    ///
    /// This method combines default commit types with any custom configurations
    /// defined in the settings, applying overrides where specified.
    ///
    /// # Returns
    ///
    /// * `HashMap<CommitType, CommitConfig>` - A map of commit types to their configurations
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

    /// Returns the configured commit scopes, if any.
    ///
    /// # Returns
    ///
    /// * `Option<Vec<String>>` - List of valid commit scopes, or None if not configured
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

    /// Returns the version separator for monorepo package tags.
    ///
    /// # Returns
    ///
    /// * `Option<&str>` - The separator string, or None if monorepo is not configured
    pub fn monorepo_separator(&self) -> Option<&str> {
        if self
            .monorepo
            .as_ref()
            .map(|m| m.packages.is_empty())
            .unwrap_or(true)
        {
            None
        } else {
            self.monorepo_version_separator.as_deref().or(Some("-"))
        }
    }

    /// Returns an iterator over all package paths in the monorepo.
    ///
    /// # Returns
    ///
    /// * `impl Iterator<Item = &Path>` - Iterator over package paths
    pub fn package_paths(&self) -> impl Iterator<Item = &Path> {
        self.monorepo
            .as_ref()
            .map(|m| m.packages.values())
            .unwrap_or_default()
            .map(|package| package.path.as_path())
    }
}

impl TryFrom<String> for Settings {
    type Error = SettingError;

    /// Attempts to create Settings from a TOML string.
    ///
    /// This allows creating settings programmatically from a TOML-formatted string.
    /// If the string is empty, it returns default settings.
    ///
    /// # Arguments
    ///
    /// * `value` - The TOML string to parse
    ///
    /// # Returns
    ///
    /// * `Result<Self, SettingError>` - The parsed settings or an error
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

#[cfg(test)]
mod test {
    use std::fs;

    use conventional_commit_parser::commit::CommitType;
    use sealed_test::prelude::*;
    use speculoos::prelude::*;

    use crate::COMMITS_METADATA;
    use cocogitto_test_helpers::git_init_no_gpg;

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
