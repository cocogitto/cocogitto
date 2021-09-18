#![cfg(not(tarpaulin_include))]
use anyhow::{Context, Result};
use clap::{App, AppSettings, Arg, Shell, SubCommand};

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
    AppSettings::VersionlessSubcommands,
    AppSettings::DeriveDisplayOrder,
];

const BUMP: &str = "bump";
const CHECK: &str = "check";
const EDIT: &str = "edit";
const LOG: &str = "log";
const VERIFY: &str = "verify";
const CHANGELOG: &str = "changelog";
const INIT: &str = "init";
const INSTALL_GIT_HOOK: &str = "install-hook";
const GENERATE_COMPLETIONS: &str = "generate-completions";

fn main() -> Result<()> {
    let matches = app().get_matches();

    if let Some(subcommand) = matches.subcommand_name() {
        match subcommand {
            BUMP => {
                let mut cocogitto = CocoGitto::get()?;
                let subcommand = matches.subcommand_matches(BUMP).unwrap();

                let increment = if let Some(version) = subcommand.value_of("version") {
                    VersionIncrement::Manual(version.to_string())
                } else if subcommand.is_present("auto") {
                    VersionIncrement::Auto
                } else if subcommand.is_present("major") {
                    VersionIncrement::Major
                } else if subcommand.is_present("patch") {
                    VersionIncrement::Patch
                } else if subcommand.is_present("minor") {
                    VersionIncrement::Minor
                } else {
                    unreachable!()
                };

                let pre = subcommand.value_of("pre");
                let hooks_profile = subcommand.value_of("hook-profile");

                // TODO mode to cli
                cocogitto.create_version(increment, pre, hooks_profile)?
            }
            VERIFY => {
                let subcommand = matches.subcommand_matches(VERIFY).unwrap();
                let message = subcommand.value_of("message").unwrap();
                let author = CocoGitto::get()
                    .map(|cogito| cogito.get_committer().unwrap())
                    .ok();

                commit::verify(author, message)?;
            }

            CHECK => {
                let cocogitto = CocoGitto::get()?;
                let subcommand = matches.subcommand_matches(CHECK).unwrap();
                let from_tag = subcommand.is_present("from-latest-tag");
                cocogitto.check(from_tag)?;
            }
            EDIT => {
                let cocogitto = CocoGitto::get()?;
                cocogitto.check_and_edit()?;
            }
            LOG => {
                let cocogitto = CocoGitto::get()?;

                let repo_tag_name = cocogitto.get_repo_tag_name();
                let repo_tag_name = repo_tag_name.as_deref().unwrap_or("cog log");

                let mut output = Output::builder()
                    .with_pager_from_env("PAGER")
                    .with_file_name(repo_tag_name)
                    .build()?;

                let subcommand = matches.subcommand_matches(LOG).unwrap();

                let mut filters = vec![];
                if let Some(commit_types) = subcommand.values_of("type") {
                    filters.extend(
                        commit_types.map(|commit_type| CommitFilter::Type(commit_type.into())),
                    );
                }

                if let Some(scopes) = subcommand.values_of("scope") {
                    filters.extend(scopes.map(|scope| CommitFilter::Scope(scope.to_string())));
                }

                if let Some(authors) = subcommand.values_of("author") {
                    filters.extend(authors.map(|author| CommitFilter::Author(author.to_string())));
                }

                if subcommand.is_present("breaking-change") {
                    filters.push(CommitFilter::BreakingChange);
                }

                if subcommand.is_present("no-error") {
                    filters.push(CommitFilter::NoError);
                }

                let filters = CommitFilters(filters);

                let content = cocogitto.get_log(filters)?;
                output
                    .handle()?
                    .write_all(content.as_bytes())
                    .context("failed to write log into the pager")?;
            }
            CHANGELOG => {
                let cocogitto = CocoGitto::get()?;
                let subcommand = matches.subcommand_matches(CHANGELOG).unwrap();
                let at = subcommand.value_of("at");
                let result = match at {
                    Some(at) => cocogitto.get_colored_changelog_at_tag(at)?,
                    None => {
                        let from = subcommand.value_of("from");
                        let to = subcommand.value_of("to");
                        cocogitto.get_colored_changelog(from, to)?
                    }
                };
                println!("{}", result);
            }

            INIT => {
                let subcommand = matches.subcommand_matches(INIT).unwrap();
                let init_path = subcommand.value_of("path").unwrap(); // safe unwrap via clap default value
                cocogitto::init(init_path)?;
            }

            INSTALL_GIT_HOOK => {
                let subcommand = matches.subcommand_matches(INSTALL_GIT_HOOK).unwrap();
                let hook_type = subcommand.value_of("hook-type").unwrap(); // safe unwrap via clap default value
                let cocogitto = CocoGitto::get()?;
                match hook_type {
                    "pre-commit" => cocogitto.install_hook(HookKind::PrepareCommit)?,
                    "pre-push" => cocogitto.install_hook(HookKind::PrePush)?,
                    "all" => cocogitto.install_hook(HookKind::All)?,
                    _ => unreachable!(),
                }
            }

            GENERATE_COMPLETIONS => {
                let generate_subcommand = matches.subcommand_matches(GENERATE_COMPLETIONS).unwrap();
                let for_shell = match generate_subcommand.value_of("type").unwrap() {
                    "bash" => Shell::Bash,
                    "elvish" => Shell::Elvish,
                    "fish" => Shell::Fish,
                    "zsh" => Shell::Zsh,
                    _ => unreachable!(),
                };
                app().gen_completions_to("cog", for_shell, &mut std::io::stdout());
            }

            _ => unreachable!(),
        }
    }
    Ok(())
}

fn app<'a, 'b>() -> App<'a, 'b> {
    let hook_profiles: Vec<&str> = HOOK_PROFILES
        .iter()
        .map(|profile| profile.as_ref())
        .collect();

    let check_command = SubCommand::with_name(CHECK)
        .settings(SUBCOMMAND_SETTINGS)
        .about("Verify all commit message against the conventional commit specification")
        .arg(
            Arg::with_name("from-latest-tag")
                .help("Check commit history, starting from the latest tag to HEAD")
                .short("l")
                .takes_value(false)
                .long("from-latest-tag"),
        )
        .display_order(1);

    let edit_command = SubCommand::with_name(EDIT)
        .settings(SUBCOMMAND_SETTINGS)
        .about("Interactively rename invalid commit message")
        .display_order(2);

    let log_command = SubCommand::with_name(LOG)
        .settings(SUBCOMMAND_SETTINGS)
        .about("Like git log but for conventional commits")
        .arg(
            Arg::with_name("breaking-change")
                .help("filter BREAKING CHANGE commit")
                .short("B")
                .long("breaking-change"),
        )
        .arg(
            Arg::with_name("type")
                .help("filter on commit type")
                .short("t")
                .takes_value(true)
                .multiple(true)
                .long("type"),
        )
        .arg(
            Arg::with_name("author")
                .help("filter on commit author")
                .short("a")
                .takes_value(true)
                .multiple(true)
                .long("author"),
        )
        .arg(
            Arg::with_name("scope")
                .help("filter on commit scope")
                .short("s")
                .takes_value(true)
                .multiple(true)
                .long("scope"),
        )
        .arg(
            Arg::with_name("no-error")
                .help("omit error on the commit log")
                .short("e")
                .long("no-error"),
        )
        .display_order(3);

    let verify_command = SubCommand::with_name(VERIFY)
        .settings(SUBCOMMAND_SETTINGS)
        .about("Verify a single commit message")
        .arg(Arg::with_name("message").help("The commit message"))
        .display_order(4);

    let changelog_command = SubCommand::with_name(CHANGELOG)
        .settings(SUBCOMMAND_SETTINGS)
        .about("Display a changelog for a given commit oid range")
        .arg(
            Arg::with_name("from")
                .help("Generate the changelog from this commit or tag ref, default latest tag")
                .short("f")
                .long("from")
                .conflicts_with("at")
                .takes_value(true),
        )
        .arg(
            Arg::with_name("to")
                .help("Generate the changelog to this commit or tag ref, default HEAD")
                .short("t")
                .long("to")
                .conflicts_with("at")
                .takes_value(true),
        )
        .arg(
            Arg::with_name("at")
                .help("Generate the changelog for a specific git tag")
                .short("at")
                .long("at")
                .takes_value(true)
                .conflicts_with_all(&["from", "to"]),
        )
        .display_order(5);

    let bump_command = SubCommand::with_name(BUMP)
        .settings(SUBCOMMAND_SETTINGS)
        .about("Commit changelog from latest tag to HEAD and create a new tag")
        .arg(
            Arg::with_name("version")
                .help("Manually set the next version")
                .short("v")
                .takes_value(true)
                .long("version")
                .required_unless_one(&["auto", "major", "patch", "minor"]),
        )
        .arg(
            Arg::with_name("auto")
                .help("Atomatically suggest the next version")
                .short("a")
                .long("auto")
                .required_unless_one(&["version", "major", "patch", "minor"]),
        )
        .arg(
            Arg::with_name("major")
                .help("Increment the major version")
                .short("M")
                .long("major")
                .required_unless_one(&["version", "auto", "patch", "minor"]),
        )
        .arg(
            Arg::with_name("patch")
                .help("Increment the patch version")
                .short("p")
                .long("patch")
                .required_unless_one(&["version", "auto", "major", "minor"]),
        )
        .arg(
            Arg::with_name("minor")
                .help("Increment the minor version")
                .short("m")
                .long("minor")
                .required_unless_one(&["version", "auto", "patch", "major"]),
        )
        .arg(
            Arg::with_name("pre")
                .help("Set the pre-release version")
                .long("pre")
                .takes_value(true),
        )
        .arg(
            Arg::with_name("hook-profile")
                .help("Specify the bump profile hooks to run")
                .short("hp")
                .long("hook-profile")
                .possible_values(&hook_profiles)
                .takes_value(true),
        )
        .display_order(6);

    let init_subcommand = SubCommand::with_name(INIT)
        .settings(SUBCOMMAND_SETTINGS)
        .arg(
            Arg::with_name("path")
                .help("path to init, default")
                .default_value("."),
        )
        .about("Install cog config files");

    let install_git_hook = SubCommand::with_name(INSTALL_GIT_HOOK)
        .settings(SUBCOMMAND_SETTINGS)
        .about("Add git hooks to the repository")
        .arg(
            Arg::with_name("hook-type")
                .help("Type of hook to install")
                .takes_value(true)
                .required(true)
                .possible_values(&["pre-commit", "pre-push", "all"]),
        )
        .display_order(7);

    App::new("Cog")
        .settings(APP_SETTINGS)
        .version(env!("CARGO_PKG_VERSION"))
        .author("Paul D. <paul.delafosse@protonmail.com>")
        .about("A command line tool for the conventional commits and semver specifications")
        .subcommands([
            verify_command,
            init_subcommand,
            check_command,
            edit_command,
            log_command,
            changelog_command,
            bump_command,
            install_git_hook,
        ])
        .subcommand(
            SubCommand::with_name(GENERATE_COMPLETIONS)
                .settings(SUBCOMMAND_SETTINGS)
                .about("Generate shell completions")
                .arg(
                    Arg::with_name("type")
                        .possible_values(&["bash", "elvish", "fish", "zsh"])
                        .required(true)
                        .takes_value(true)
                        .help("Type of completions to generate"),
                ),
        )
}
