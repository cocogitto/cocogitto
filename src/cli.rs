use crate::{COMMITS_METADATA, SETTINGS};
use clap::{AppSettings, ArgGroup, Args, Parser, Subcommand};
use clap_complete::Shell;
use std::path::PathBuf;

/// A command line tool for the conventional commits and semver specifications
#[derive(Parser)]
#[clap(global_setting = AppSettings::DeriveDisplayOrder)]
#[clap(
    version,
    name = "cog",
    author = "Paul D. <paul.delafosse@protonmail.com>"
)]
pub struct Cli {
    /// The level of verbosity: -v for ERROR, -vv for WARNING, -vvv for INFO
    #[clap(long, short = 'v', parse(from_occurrences))]
    pub verbose: i8,

    /// Silence all output, no matter the value of verbosity
    #[clap(long, short = 'q')]
    pub quiet: bool,

    #[clap(subcommand)]
    pub command: CogCommand,
}

#[derive(Subcommand)]
pub enum CogCommand {
    /// Verify all commit messages against the conventional commit specification
    Check {
        /// Check commit history, starting from the latest tag to HEAD
        #[clap(short = 'l', long)]
        from_latest_tag: bool,
        /// Ignore merge commits messages
        #[clap(short, long)]
        ignore_merge_commits: bool,
    },

    /// Create a new conventional commit
    Commit(CommitArgs),

    /// Interactively rename invalid commit messages
    Edit {
        /// Edit non conventional commits, starting from the latest tag to HEAD
        #[clap(short = 'l', long)]
        from_latest_tag: bool,
    },

    /// Like git log but for conventional commits
    Log {
        /// filter BREAKING CHANGE commits
        #[clap(short = 'B', long)]
        breaking_change: bool,

        /// filter on commit type
        #[clap(short, long = "type", value_name = "type")]
        typ: Option<Vec<String>>,

        /// filter on commit author
        #[clap(short, long)]
        author: Option<Vec<String>>,

        /// filter on commit scope
        #[clap(short, long)]
        scope: Option<Vec<String>>,

        /// omit error on the commit log
        #[clap(short = 'e', long)]
        no_error: bool,
    },

    /// Verify a single commit message
    Verify {
        /// The commit message
        message: String,
        /// Ignore merge commits messages
        #[clap(short, long)]
        ignore_merge_commits: bool,
    },

    /// Display a changelog for the given commit oid range
    Changelog {
        /// Generate the changelog from in the given spec range
        #[clap(conflicts_with = "at")]
        pattern: Option<String>,

        /// Generate the changelog for a specific git tag
        #[clap(short, long)]
        at: Option<String>,

        /// Generate the changelog with the given template.
        /// Possible values are 'remote', 'full_hash', 'default' or the path to your template.
        /// If not specified cog will use cog.toml template config or fallback to 'default'.
        #[clap(name = "template", long, short)]
        template: Option<String>,

        /// Url to use during template generation
        #[clap(name = "remote", long, short, requires_all(&["owner", "repository"]))]
        remote: Option<String>,

        /// Repository owner to use during template generation
        #[clap(name = "owner", long, short, requires_all(& ["remote", "repository"]))]
        owner: Option<String>,

        /// Name of the repository used during template generation
        #[clap(name = "repository", long, requires_all(& ["owner", "remote"]))]
        repository: Option<String>,
    },

    /// Commit changelog from latest tag to HEAD and create new tag
    #[clap(group = ArgGroup::new("bump-spec").required(true))]
    Bump {
        /// Manually set the target version
        #[clap(long, group = "bump-spec")]
        version: Option<String>,

        /// Automatically suggest the target version
        #[clap(short, long, group = "bump-spec")]
        auto: bool,

        /// Increment the major version
        #[clap(short = 'M', long, group = "bump-spec")]
        major: bool,

        /// Increment the minor version
        #[clap(short, long, group = "bump-spec")]
        minor: bool,

        /// Increment the patch version
        #[clap(short, long, group = "bump-spec")]
        patch: bool,

        /// Set the pre-release version
        #[clap(long)]
        pre: Option<String>,

        /// Specify the bump profile hooks to run
        #[clap(short = 'H', long, possible_values = hook_profiles())]
        hook_profile: Option<String>,

        /// Dry-run : get the target version. No action taken
        #[clap(short, long)]
        dry_run: bool,
    },

    /// Install cog config files
    Init {
        /// path to init
        #[clap(default_value = ".")]
        path: PathBuf,
    },

    /// Add git hooks to the repository
    InstallHook {
        /// Type of hook to install
        #[clap(possible_values = &["commit-msg", "pre-push", "all"])]
        hook_type: String,
    },

    /// Generate shell completions
    GenerateCompletions {
        /// Type of completions to generate
        #[clap(name = "type", arg_enum)]
        shell: Shell,
    },
}

#[derive(Args)]
pub struct CommitArgs {
    /// Conventional commit type
    #[clap(name = "type", value_name = "TYPE", possible_values = commit_types())]
    pub typ: String,

    /// Commit description
    pub message: String,

    /// Conventional commit scope
    pub scope: Option<String>,

    /// Create a BREAKING CHANGE commit
    #[clap(short = 'B', long)]
    pub breaking_change: bool,

    /// Open commit message in an editor
    #[clap(short, long)]
    pub edit: bool,
}

fn hook_profiles() -> Vec<&'static str> {
    SETTINGS
        .bump_profiles
        .keys()
        .map(|profile| profile.as_ref())
        .collect()
}

pub fn commit_types() -> Vec<&'static str> {
    COMMITS_METADATA
        .iter()
        .map(|(commit_type, _)| commit_type.as_ref())
        .collect()
}
