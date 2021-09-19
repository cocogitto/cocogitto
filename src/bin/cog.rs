#![cfg(not(tarpaulin_include))]
use std::path::PathBuf;

use anyhow::{Context, Result};
use structopt::clap::{AppSettings, Shell};
use structopt::StructOpt;

use cocogitto::conventional::commit;
use cocogitto::conventional::version::VersionIncrement;
use cocogitto::git::hook::HookKind;
use cocogitto::log::filter::{CommitFilter, CommitFilters};
use cocogitto::log::output::Output;
use cocogitto::{CocoGitto, HOOK_PROFILES};

const APP_SETTINGS: &[AppSettings] = &[
    AppSettings::SubcommandRequiredElseHelp,
    AppSettings::UnifiedHelpMessage,
    AppSettings::ColoredHelp,
    AppSettings::VersionlessSubcommands,
    AppSettings::DeriveDisplayOrder,
];

const SUBCOMMAND_SETTINGS: &[AppSettings] = &[
    AppSettings::UnifiedHelpMessage,
    AppSettings::ColoredHelp,
    AppSettings::DeriveDisplayOrder,
];

fn hook_profiles() -> Vec<&'static str> {
    HOOK_PROFILES
        .iter()
        .map(|profile| profile.as_ref())
        .collect()
}

/// A command line tool for the conventional commits and semver specifications
#[derive(StructOpt)]
#[structopt(name = "Cog", author = "Paul D. <paul.delafosse@protonmail.com>", settings = APP_SETTINGS)]
enum Cli {
    /// Verify all commit messages against the conventional commit specification
    #[structopt(no_version, settings = SUBCOMMAND_SETTINGS)]
    Check {
        /// Check commit history, starting from the latest tag to HEAD
        #[structopt(short = "l", long)]
        from_latest_tag: bool,
    },

    /// Interactively rename invalid commit messages
    #[structopt(no_version, settings = SUBCOMMAND_SETTINGS)]
    Edit,

    /// Like git log but for conventional commits
    #[structopt(no_version, settings = SUBCOMMAND_SETTINGS)]
    Log {
        /// filter BREAKING CHANGE commits
        #[structopt(short = "B", long)]
        breaking_change: bool,

        /// filter on commit type
        #[structopt(short, long = "type", value_name = "type")]
        typ: Option<Vec<String>>,

        /// filter on commit author
        #[structopt(short, long)]
        author: Option<Vec<String>>,

        /// filter on commit scope
        #[structopt(short, long)]
        scope: Option<Vec<String>>,

        /// omit error on the commit log
        #[structopt(short = "e", long)]
        no_error: bool,
    },

    /// Verify a single commit message
    #[structopt(no_version, settings = SUBCOMMAND_SETTINGS)]
    Verify {
        /// The commit message
        message: String,
    },

    /// Display a changelog for the given commit oid range
    #[structopt(no_version, settings = SUBCOMMAND_SETTINGS)]
    Changelog {
        /// Generate the changelog from this commit or tag ref, default latest tag
        #[structopt(short, long, conflicts_with = "at")]
        from: Option<String>,

        /// Generate the changelog to this commit or tag ref, default HEAD
        #[structopt(short, long, conflicts_with = "at")]
        to: Option<String>,

        /// Generate the changelog for a specific git tag
        #[structopt(short, long)]
        at: Option<String>,
    },

    /// Commit changelog from latest tag to HEAD and create new tag
    #[structopt(no_version, settings = SUBCOMMAND_SETTINGS)]
    Bump {
        /// Manually set the next version
        #[structopt(short, long, required_unless_one = &["auto", "major", "minor", "patch"])]
        version: Option<String>,

        /// Automatically suggest the next version
        #[structopt(short, long, required_unless_one = &["version", "major", "minor", "patch"])]
        auto: bool,

        /// Increment the major version
        #[structopt(short = "M", long, required_unless_one = &["version", "auto", "minor", "patch"])]
        major: bool,

        /// Increment the minor version
        #[structopt(short, long, required_unless_one = &["version", "auto", "major", "patch"])]
        minor: bool,

        /// Increment the patch version
        #[structopt(short, long, required_unless_one = &["version", "auto", "major", "minor"])]
        patch: bool,

        /// Set the pre-release version
        #[structopt(long)]
        pre: Option<String>,

        /// Specify the bump profile hooks to run
        #[structopt(short, long, possible_values = &hook_profiles())]
        hook_profile: Option<String>,
    },

    /// Install cog config files
    #[structopt(no_version, settings = SUBCOMMAND_SETTINGS)]
    Init {
        /// path to init
        #[structopt(default_value = ".")]
        path: PathBuf,
    },

    /// Add git hooks to the repository
    #[structopt(no_version, settings = SUBCOMMAND_SETTINGS)]
    InstallHook {
        /// Type of hook to install
        #[structopt(possible_values = &["pre-commit", "pre-push", "all"])]
        hook_type: String,
    },

    /// Generate shell completions
    #[structopt(no_version, settings = SUBCOMMAND_SETTINGS)]
    GenerateCompletions {
        /// Type of completions to generate
        #[structopt(name = "type", possible_values = &["bash", "elvish", "fish", "zsh"])]
        shell: Shell,
    },
}

fn main() -> Result<()> {
    let cli = Cli::from_args();

    match cli {
        Cli::Bump {
            version,
            auto,
            major,
            minor,
            patch,
            pre,
            hook_profile,
        } => {
            let mut cocogitto = CocoGitto::get()?;

            let increment = if let Some(version) = version {
                VersionIncrement::Manual(version)
            } else if auto {
                VersionIncrement::Auto
            } else if major {
                VersionIncrement::Major
            } else if minor {
                VersionIncrement::Minor
            } else if patch {
                VersionIncrement::Patch
            } else {
                unreachable!()
            };

            // TODO mode to cli
            cocogitto.create_version(increment, pre.as_deref(), hook_profile.as_deref())?
        }
        Cli::Verify { message } => {
            let author = CocoGitto::get()
                .map(|cogito| cogito.get_committer().unwrap())
                .ok();

            commit::verify(author, &message)?;
        }
        Cli::Check { from_latest_tag } => {
            let cocogitto = CocoGitto::get()?;
            cocogitto.check(from_latest_tag)?;
        }
        Cli::Edit => {
            let cocogitto = CocoGitto::get()?;
            cocogitto.check_and_edit()?;
        }
        Cli::Log {
            breaking_change,
            typ,
            author,
            scope,
            no_error,
        } => {
            let cocogitto = CocoGitto::get()?;

            let repo_tag_name = cocogitto.get_repo_tag_name();
            let repo_tag_name = repo_tag_name.as_deref().unwrap_or("cog log");

            let mut output = Output::builder()
                .with_pager_from_env("PAGER")
                .with_file_name(repo_tag_name)
                .build()?;

            let mut filters = vec![];
            if let Some(commit_types) = typ {
                filters.extend(
                    commit_types
                        .iter()
                        .map(|commit_type| CommitFilter::Type(commit_type.as_str().into())),
                );
            }

            if let Some(scopes) = scope {
                filters.extend(scopes.into_iter().map(CommitFilter::Scope));
            }

            if let Some(authors) = author {
                filters.extend(authors.into_iter().map(CommitFilter::Author));
            }

            if breaking_change {
                filters.push(CommitFilter::BreakingChange);
            }

            if no_error {
                filters.push(CommitFilter::NoError);
            }

            let filters = CommitFilters(filters);

            let content = cocogitto.get_log(filters)?;
            output
                .handle()?
                .write_all(content.as_bytes())
                .context("failed to write log into the pager")?;
        }
        Cli::Changelog { from, to, at } => {
            let cocogitto = CocoGitto::get()?;
            let result = match at {
                Some(at) => cocogitto.get_colored_changelog_at_tag(&at)?,
                None => cocogitto.get_colored_changelog(from.as_deref(), to.as_deref())?,
            };
            println!("{}", result);
        }
        Cli::Init { path } => {
            cocogitto::init(&path)?;
        }
        Cli::InstallHook { hook_type } => {
            let cocogitto = CocoGitto::get()?;
            match hook_type.as_str() {
                "pre-commit" => cocogitto.install_hook(HookKind::PrepareCommit)?,
                "pre-push" => cocogitto.install_hook(HookKind::PrePush)?,
                "all" => cocogitto.install_hook(HookKind::All)?,
                _ => unreachable!(),
            }
        }
        Cli::GenerateCompletions { shell } => {
            Cli::clap().gen_completions_to("cog", shell, &mut std::io::stdout());
        }
    }

    Ok(())
}
