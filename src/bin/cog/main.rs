mod commit;

use std::path::PathBuf;

use cocogitto::conventional::changelog::template::{RemoteContext, Template};
use cocogitto::conventional::commit as conv_commit;
use cocogitto::conventional::version::VersionIncrement;
use cocogitto::git::hook::HookKind;
use cocogitto::git::revspec::RevspecPattern;
use cocogitto::log::filter::{CommitFilter, CommitFilters};
use cocogitto::log::output::Output;
use cocogitto::{CocoGitto, SETTINGS};

use anyhow::{bail, Context, Result};
use clap::builder::{PossibleValue, PossibleValuesParser};
use clap::{ArgAction, ArgGroup, Args, CommandFactory, Parser, Subcommand, ValueEnum};
use clap_complete::{shells, Generator};
use clap_complete_nushell::Nushell;

fn hook_profiles() -> PossibleValuesParser {
    let profiles = SETTINGS
        .bump_profiles
        .keys()
        .map(|profile| -> &str { profile });

    profiles.into()
}

fn packages() -> PossibleValuesParser {
    let profiles = SETTINGS.packages.keys().map(|profile| -> &str { profile });

    profiles.into()
}

/// Shell with auto-generated completion script available.
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
#[non_exhaustive]
pub enum Shell {
    /// Bourne Again SHell (bash)
    Bash,
    /// Elvish shell
    Elvish,
    /// Friendly Interactive SHell (fish)
    Fish,
    /// PowerShell
    PowerShell,
    /// Z SHell (zsh)
    Zsh,
    /// Nu shell (nu)
    Nu,
}

impl Generator for Shell {
    fn file_name(&self, name: &str) -> String {
        match self {
            Shell::Bash => shells::Bash.file_name(name),
            Shell::Elvish => shells::Elvish.file_name(name),
            Shell::Fish => shells::Fish.file_name(name),
            Shell::PowerShell => shells::PowerShell.file_name(name),
            Shell::Zsh => shells::Zsh.file_name(name),
            Shell::Nu => Nushell.file_name(name),
        }
    }

    fn generate(&self, cmd: &clap::Command, buf: &mut dyn std::io::Write) {
        match self {
            Shell::Bash => shells::Bash.generate(cmd, buf),
            Shell::Elvish => shells::Elvish.generate(cmd, buf),
            Shell::Fish => shells::Fish.generate(cmd, buf),
            Shell::PowerShell => shells::PowerShell.generate(cmd, buf),
            Shell::Zsh => shells::Zsh.generate(cmd, buf),
            Shell::Nu => Nushell.generate(cmd, buf),
        }
    }
}

// Hand-rolled so it can work even when `derive` feature is disabled
impl ValueEnum for Shell {
    fn value_variants<'a>() -> &'a [Self] {
        &[
            Shell::Bash,
            Shell::Elvish,
            Shell::Fish,
            Shell::PowerShell,
            Shell::Zsh,
            Shell::Nu,
        ]
    }

    fn to_possible_value<'a>(&self) -> Option<PossibleValue> {
        Some(match self {
            Shell::Bash => PossibleValue::new("bash"),
            Shell::Elvish => PossibleValue::new("elvish"),
            Shell::Fish => PossibleValue::new("fish"),
            Shell::PowerShell => PossibleValue::new("powershell"),
            Shell::Zsh => PossibleValue::new("zsh"),
            Shell::Nu => PossibleValue::new("nu"),
        })
    }
}

/// A command line tool for the conventional commits and semver specifications
#[derive(Parser)]
#[command(
    version,
    name = "Cog",
    author = "Paul D. <paul.delafosse@protonmail.com>"
)]
struct Cli {
    /// The level of verbosity: -v for ERROR, -vv for WARNING, -vvv for INFO
    #[arg(long, short = 'v', action = ArgAction::Count)]
    verbose: u8,

    /// Silence all output, no matter the value of verbosity
    #[arg(long, short = 'q')]
    quiet: bool,

    #[command(subcommand)]
    command: Command,
}

#[derive(Subcommand)]
enum Command {
    /// Verify all commit messages against the conventional commit specification
    Check {
        /// Check commit history, starting from the latest tag to HEAD
        #[arg(short = 'l', long)]
        from_latest_tag: bool,

        /// Ignore merge commits messages
        #[arg(short, long)]
        ignore_merge_commits: bool,
    },

    /// Create a new conventional commit
    Commit(CommitArgs),

    /// Interactively rename invalid commit messages
    Edit {
        /// Edit non conventional commits, starting from the latest tag to HEAD
        #[arg(short = 'l', long)]
        from_latest_tag: bool,
    },

    /// Like git log but for conventional commits
    Log {
        /// Filter BREAKING CHANGE commits
        #[arg(short = 'B', long)]
        breaking_change: bool,

        /// Filter on commit type
        #[arg(short, long = "type", value_name = "type")]
        typ: Option<Vec<String>>,

        /// Filter on commit author
        #[arg(short, long)]
        author: Option<Vec<String>>,

        /// Filter on commit scope
        #[arg(short, long)]
        scope: Option<Vec<String>>,

        /// Omit error on the commit log
        #[arg(short = 'e', long)]
        no_error: bool,
    },

    /// Verify a single commit message
    Verify {
        /// The commit message
        message: String,

        /// Ignore merge commit messages
        #[arg(short, long)]
        ignore_merge_commits: bool,
    },

    /// Display a changelog for the given commit oid range
    Changelog {
        /// Generate the changelog in the given spec range
        #[arg(conflicts_with = "at")]
        pattern: Option<String>,

        /// Generate the changelog for a specific git tag
        #[arg(short, long)]
        at: Option<String>,

        /// Generate the changelog with the given template.
        ///
        /// Possible values are 'remote', 'full_hash', 'default' or the path to your template.  
        /// If not specified cog will use cog.toml template config or fallback to 'default'.
        #[arg(long, short)]
        template: Option<String>,

        /// Url to use during template generation
        #[arg(long, short, requires_all = ["owner", "repository"])]
        remote: Option<String>,

        /// Repository owner to use during template generation
        #[arg(long, short, requires_all = ["remote", "repository"])]
        owner: Option<String>,

        /// Name of the repository used during template generation
        #[arg(long, requires_all = ["owner", "remote"])]
        repository: Option<String>,
    },

    /// Commit changelog from latest tag to HEAD and create new tag
    #[command(group = ArgGroup::new("bump-spec").required(true))]
    Bump {
        /// Manually set the target version
        #[arg(long, group = "bump-spec")]
        version: Option<String>,

        /// Automatically suggest the target version
        #[arg(short, long, group = "bump-spec")]
        auto: bool,

        /// Increment the major version
        #[arg(short = 'M', long, group = "bump-spec")]
        major: bool,

        /// Increment the minor version
        #[arg(short, long, group = "bump-spec")]
        minor: bool,

        /// Increment the patch version
        #[arg(short, long, group = "bump-spec")]
        patch: bool,

        /// Set the pre-release version
        #[arg(long)]
        pre: Option<String>,

        /// Specify the bump profile hooks to run
        #[arg(short = 'H', long, value_parser = hook_profiles())]
        hook_profile: Option<String>,

        /// Specify which package to bump for monorepo
        #[arg(long, value_parser = packages())]
        package: Option<String>,

        /// Dry-run: print the target version. No action taken
        #[arg(short, long)]
        dry_run: bool,
    },

    /// Install cog config files
    Init {
        /// Path to initialized dir
        #[arg(default_value = ".")]
        path: PathBuf,
    },

    /// Add git hooks to the repository
    InstallHook {
        /// Type of hook to install
        #[arg(value_parser = ["commit-msg", "pre-push", "all"])]
        hook_type: String,
    },

    /// Generate shell completions
    GenerateCompletions {
        /// Shell to generate completions for
        #[arg(value_enum)]
        shell: Shell,
    },

    /// Generate manpage
    #[command(hide = true)]
    GenerateManpage { cmd: String },
}

#[derive(Args)]
struct CommitArgs {
    /// Conventional commit type
    #[arg(name = "type", value_name = "TYPE", value_parser = commit::commit_types())]
    typ: String,

    /// Commit description
    message: String,

    /// Conventional commit scope
    scope: Option<String>,

    /// Create a BREAKING CHANGE commit
    #[arg(short = 'B', long)]
    breaking_change: bool,

    /// Open commit message in an editor
    #[arg(short, long)]
    edit: bool,

    /// Sign this commit
    #[arg(short, long)]
    sign: bool,
}

fn main() -> Result<()> {
    let cli = Cli::parse();

    init_logs(cli.verbose, cli.quiet);

    match cli.command {
        Command::Bump {
            version,
            auto,
            major,
            minor,
            patch,
            pre,
            hook_profile,
            package,
            dry_run,
        } => {
            let mut cocogitto = CocoGitto::get()?;

            let increment = match version {
                Some(version) => VersionIncrement::Manual(version),
                None if auto => VersionIncrement::Auto,
                None if major => VersionIncrement::Major,
                None if minor => VersionIncrement::Minor,
                None if patch => VersionIncrement::Patch,
                _ => unreachable!(),
            };

            let is_monorepo = !SETTINGS.packages.is_empty();

            if is_monorepo {
                if increment == VersionIncrement::Auto && package.is_none() {
                    cocogitto.create_monorepo_version(
                        pre.as_deref(),
                        hook_profile.as_deref(),
                        dry_run,
                    )?
                } else if let Some(package_name) = package {
                    // Safe unwrap here, package name is validated by clap
                    let package = SETTINGS.packages.get(&package_name).unwrap();
                    cocogitto.create_package_version(
                        (&package_name, package),
                        increment,
                        pre.as_deref(),
                        hook_profile.as_deref(),
                        dry_run,
                    )?
                } else {
                    bail!("Cannot bump monorepo manually, use `--package` to update a specific package.")
                }
            } else {
                cocogitto.create_version(
                    increment,
                    pre.as_deref(),
                    hook_profile.as_deref(),
                    dry_run,
                )?
            }
        }
        Command::Verify {
            message,
            ignore_merge_commits,
        } => {
            let ignore_merge_commits = ignore_merge_commits || SETTINGS.ignore_merge_commits;
            let author = CocoGitto::get()
                .map(|cogito| cogito.get_committer().unwrap())
                .ok();

            conv_commit::verify(author, &message, ignore_merge_commits)?;
        }
        Command::Check {
            from_latest_tag,
            ignore_merge_commits,
        } => {
            let cocogitto = CocoGitto::get()?;
            let from_latest_tag = from_latest_tag || SETTINGS.from_latest_tag;
            let ignore_merge_commits = ignore_merge_commits || SETTINGS.ignore_merge_commits;
            cocogitto.check(from_latest_tag, ignore_merge_commits)?;
        }
        Command::Edit { from_latest_tag } => {
            let cocogitto = CocoGitto::get()?;
            let from_latest_tag = from_latest_tag || SETTINGS.from_latest_tag;
            cocogitto.check_and_edit(from_latest_tag)?;
        }
        Command::Log {
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
        Command::Changelog {
            pattern,
            at,
            template,
            remote,
            owner,
            repository,
        } => {
            let cocogitto = CocoGitto::get()?;

            let context = RemoteContext::try_new(remote, repository, owner)
                .or_else(|| SETTINGS.get_template_context());
            let template = template.as_ref().or(SETTINGS.changelog.template.as_ref());
            let template = if let Some(template) = template {
                Template::from_arg(template, context)?
            } else {
                Template::default()
            };

            let pattern = pattern.as_deref().map(RevspecPattern::from);

            let result = match at {
                Some(at) => cocogitto.get_changelog_at_tag(&at, template)?,
                None => {
                    let changelog = cocogitto.get_changelog(pattern.unwrap_or_default(), true)?;
                    changelog.into_markdown(template)?
                }
            };
            println!("{}", result);
        }
        Command::Init { path } => {
            cocogitto::init(&path)?;
        }
        Command::InstallHook { hook_type } => {
            let cocogitto = CocoGitto::get()?;
            match hook_type.as_str() {
                "commit-msg" => cocogitto.install_hook(HookKind::PrepareCommit)?,
                "pre-push" => cocogitto.install_hook(HookKind::PrePush)?,
                "all" => cocogitto.install_hook(HookKind::All)?,
                _ => unreachable!(),
            }
        }
        Command::GenerateCompletions { shell } => {
            clap_complete::generate(shell, &mut Cli::command(), "cog", &mut std::io::stdout());
        }
        Command::GenerateManpage { cmd } => {
            let cog_cmd = Cli::command();
            let cmd = match cmd.as_str() {
                "cog" => cog_cmd,
                cmd => cog_cmd
                    .find_subcommand(cmd)
                    .expect("Requested non-existent subcommand")
                    .clone(),
            };
            let man = clap_mangen::Man::new(cmd);
            man.render(&mut std::io::stdout())?;
        }
        Command::Commit(CommitArgs {
            typ,
            message,
            scope,
            breaking_change,
            edit,
            sign,
        }) => {
            let cocogitto = CocoGitto::get()?;
            let (body, footer, breaking) = if edit {
                commit::edit_message(&typ, &message, scope.as_deref(), breaking_change)?
            } else {
                (None, None, breaking_change)
            };

            cocogitto.conventional_commit(&typ, scope, message, body, footer, breaking, sign)?;
        }
    }

    Ok(())
}

fn init_logs(verbose: u8, quiet: bool) {
    let verbosity = if verbose == 0 { 2 } else { verbose - 1 };
    stderrlog::new()
        .module(module_path!())
        .modules(vec!["cocogitto"])
        .quiet(quiet)
        .verbosity(verbosity as usize)
        .show_level(false)
        .show_module_names(false)
        .init()
        .unwrap();
}
