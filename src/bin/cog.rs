mod cog_commit;

use cocogitto::conventional::changelog::template::{RemoteContext, Template};
use cocogitto::conventional::commit;
use cocogitto::conventional::version::VersionIncrement;
use cocogitto::git::hook::HookKind;
use cocogitto::git::revspec::RevspecPattern;
use cocogitto::log::filter::{CommitFilter, CommitFilters};
use cocogitto::log::output::Output;
use cocogitto::{CocoGitto, SETTINGS};

use anyhow::{Context, Result};
use clap::{CommandFactory, Parser};
use cocogitto::cli::{Cli, CogCommand, CommitArgs};

fn main() -> Result<()> {
    let cli = Cli::parse();

    init_logs(cli.verbose, cli.quiet);

    match cli.command {
        CogCommand::Bump {
            version,
            auto,
            major,
            minor,
            patch,
            pre,
            hook_profile,
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

            cocogitto.create_version(increment, pre.as_deref(), hook_profile.as_deref(), dry_run)?
        }
        CogCommand::Verify {
            message,
            ignore_merge_commits,
        } => {
            let ignore_merge_commits = ignore_merge_commits || SETTINGS.ignore_merge_commits;
            let author = CocoGitto::get()
                .map(|cogito| cogito.get_committer().unwrap())
                .ok();

            commit::verify(author, &message, ignore_merge_commits)?;
        }
        CogCommand::Check {
            from_latest_tag,
            ignore_merge_commits,
        } => {
            let cocogitto = CocoGitto::get()?;
            let ignore_merge_commits = ignore_merge_commits || SETTINGS.ignore_merge_commits;
            cocogitto.check(from_latest_tag, ignore_merge_commits)?;
        }
        CogCommand::Edit { from_latest_tag } => {
            let cocogitto = CocoGitto::get()?;
            cocogitto.check_and_edit(from_latest_tag)?;
        }
        CogCommand::Log {
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
        CogCommand::Changelog {
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
        CogCommand::Init { path } => {
            cocogitto::init(&path)?;
        }
        CogCommand::InstallHook { hook_type } => {
            let cocogitto = CocoGitto::get()?;
            match hook_type.as_str() {
                "commit-msg" => cocogitto.install_hook(HookKind::PrepareCommit)?,
                "pre-push" => cocogitto.install_hook(HookKind::PrePush)?,
                "all" => cocogitto.install_hook(HookKind::All)?,
                _ => unreachable!(),
            }
        }
        CogCommand::GenerateCompletions { shell } => {
            clap_complete::generate(shell, &mut Cli::command(), "cog", &mut std::io::stdout());
        }
        CogCommand::Commit(CommitArgs {
            typ,
            message,
            scope,
            breaking_change,
            edit,
        }) => {
            let cocogitto = CocoGitto::get()?;
            let (body, footer, breaking) = if edit {
                cog_commit::edit_message(&typ, &message, scope.as_deref(), breaking_change)?
            } else {
                (None, None, breaking_change)
            };

            cocogitto.conventional_commit(&typ, scope, message, body, footer, breaking)?;
        }
    }

    Ok(())
}

fn init_logs(verbose: i8, quiet: bool) {
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
