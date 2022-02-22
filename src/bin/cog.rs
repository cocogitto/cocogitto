mod cog_commit;

use std::path::PathBuf;

use cocogitto::conventional::changelog::template::{RemoteContext, Template};
use cocogitto::conventional::commit;
use cocogitto::conventional::version::VersionIncrement;
use cocogitto::git::hook::HookKind;
use cocogitto::git::revspec::RevspecPattern;
use cocogitto::log::filter::{CommitFilter, CommitFilters};
use cocogitto::log::output::Output;
use cocogitto::{CocoGitto, SETTINGS};

use anyhow::{Context, Result};
use clap::{AppSettings, ArgGroup, Args, CommandFactory, Parser};
use clap_complete::Shell;

fn hook_profiles() -> Vec<&'static str> {
    SETTINGS
        .bump_profiles
        .keys()
        .map(|profile| profile.as_ref())
        .collect()
}

/// A command line tool for the conventional commits and semver specifications
#[derive(Parser)]
#[clap(global_setting = AppSettings::DeriveDisplayOrder)]
#[clap(
    version,
    name = "Cog",
    author = "Paul D. <paul.delafosse@protonmail.com>"
)]
enum Cli {
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
        #[clap(short, long, group = "bump-spec")]
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
struct CommitArgs {
    /// Conventional commit type
    #[clap(name = "type", value_name = "TYPE", possible_values = cog_commit::commit_types())]
    typ: String,

    /// Commit description
    message: String,

    /// Conventional commit scope
    scope: Option<String>,

    /// Create a BREAKING CHANGE commit
    #[clap(short = 'B', long)]
    breaking_change: bool,

    /// Open commit message in an editor
    #[clap(short, long)]
    edit: bool,

    /// Sign this commit
    #[clap(short, long)]
    sign: bool,
}

fn main() -> Result<()> {
    let cli = Cli::parse();

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

            let increment = match version {
                Some(version) => VersionIncrement::Manual(version),
                None if auto => VersionIncrement::Auto,
                None if major => VersionIncrement::Major,
                None if minor => VersionIncrement::Minor,
                None if patch => VersionIncrement::Patch,
                _ => unreachable!(),
            };

            cocogitto.create_version(increment, pre.as_deref(), hook_profile.as_deref())?
        }
        Cli::Verify {
            message,
            ignore_merge_commits,
        } => {
            let ignore_merge_commits = ignore_merge_commits || SETTINGS.ignore_merge_commits;
            let author = CocoGitto::get()
                .map(|cogito| cogito.get_committer().unwrap())
                .ok();

            commit::verify(author, &message, ignore_merge_commits)?;
        }
        Cli::Check {
            from_latest_tag,
            ignore_merge_commits,
        } => {
            let cocogitto = CocoGitto::get()?;
            let ignore_merge_commits = ignore_merge_commits || SETTINGS.ignore_merge_commits;
            cocogitto.check(from_latest_tag, ignore_merge_commits)?;
        }
        Cli::Edit { from_latest_tag } => {
            let cocogitto = CocoGitto::get()?;
            cocogitto.check_and_edit(from_latest_tag)?;
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
        Cli::Changelog {
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
        Cli::Init { path } => {
            cocogitto::init(&path)?;
        }
        Cli::InstallHook { hook_type } => {
            let cocogitto = CocoGitto::get()?;
            match hook_type.as_str() {
                "commit-msg" => cocogitto.install_hook(HookKind::PrepareCommit)?,
                "pre-push" => cocogitto.install_hook(HookKind::PrePush)?,
                "all" => cocogitto.install_hook(HookKind::All)?,
                _ => unreachable!(),
            }
        }
        Cli::GenerateCompletions { shell } => {
            clap_complete::generate(shell, &mut Cli::command(), "cog", &mut std::io::stdout());
        }
        Cli::Commit(CommitArgs {
            typ,
            message,
            scope,
            breaking_change,
            edit,
            sign,
        }) => {
            let cocogitto = CocoGitto::get()?;
            let (body, footer, breaking) = if edit {
                cog_commit::edit_message(&typ, &message, scope.as_deref(), breaking_change)?
            } else {
                (None, None, breaking_change)
            };

            cocogitto.conventional_commit(&typ, scope, message, body, footer, breaking, sign)?;
        }
    }

    Ok(())
}
