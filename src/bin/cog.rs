use anyhow::{Context, Result};
use clap::{App, AppSettings, Arg, SubCommand};
use cocogitto::commit::CommitType;
use cocogitto::filter::{CommitFilter, CommitFilters};
use cocogitto::version::VersionIncrement;
use cocogitto::CocoGitto;
use cocogitto::{changelog::WriterMode, output::Output};
use std::process::exit;

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

fn main() -> Result<()> {
    let check_command = SubCommand::with_name(CHECK)
        .settings(SUBCOMMAND_SETTINGS)
        .about("Verify all commit message against the conventional commit specification")
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
                .takes_value(true),
        )
        .arg(
            Arg::with_name("to")
                .help("Generate the changelog to this commit or tag ref, default HEAD")
                .short("t")
                .long("to")
                .takes_value(true),
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
        .display_order(6);

    let init_subcommand = SubCommand::with_name(INIT)
        .settings(SUBCOMMAND_SETTINGS)
        .arg(
            Arg::with_name("path")
                .help("path to init, default")
                .default_value("."),
        )
        .about("Install cog config files");

    let matches = App::new("Cogitto")
        .settings(APP_SETTINGS)
        .version(env!("CARGO_PKG_VERSION"))
        .author("Paul D. <paul.delafosse@protonmail.com>")
        .about("A conventional commit compliant, changelog and commit generator")
        .long_about("Conventional Commit Git Terminal Overlord is a tool to help you use the conventional commit specification")
        .subcommands(vec![verify_command, init_subcommand, check_command, edit_command, log_command, changelog_command, bump_command])
        .get_matches();

    if let Some(subcommand) = matches.subcommand_name() {
        match subcommand {
            BUMP => {
                let cocogitto = CocoGitto::get()?;
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

                // TODO mode to cli
                cocogitto.create_version(increment, WriterMode::Prepend, pre)?
            }
            VERIFY => {
                let subcommand = matches.subcommand_matches(VERIFY).unwrap();
                let message = subcommand.value_of("message").unwrap();
                let author = CocoGitto::get()
                    .map(|cogito| cogito.get_committer().unwrap())
                    .ok();

                match cocogitto::verify(author, message) {
                    Ok(()) => exit(0),
                    Err(err) => {
                        eprintln!("{}", err);
                        exit(1);
                    }
                }
            }

            CHECK => {
                let cocogitto = CocoGitto::get()?;
                cocogitto.check()?
            }
            EDIT => {
                let cocogitto = CocoGitto::get()?;
                cocogitto.check_and_edit()?;
            }
            LOG => {
                let cocogitto = CocoGitto::get()?;

                let mut output = Output::builder()
                    .with_pager_from_env("PAGER")
                    // TODO: replace with "repo_name:latest_tag"?
                    .with_file_name("cog log")
                    .build()?;

                let subcommand = matches.subcommand_matches(LOG).unwrap();

                let mut filters = vec![];
                if let Some(commit_types) = subcommand.values_of("type") {
                    commit_types.for_each(|commit_type| {
                        filters.push(CommitFilter::Type(CommitType::from(commit_type)));
                    });
                }

                if let Some(scopes) = subcommand.values_of("scope") {
                    scopes.for_each(|scope| {
                        filters.push(CommitFilter::Scope(scope.to_string()));
                    });
                }

                if let Some(authors) = subcommand.values_of("author") {
                    authors.for_each(|author| {
                        filters.push(CommitFilter::Author(author.to_string()));
                    });
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
                let from = subcommand.value_of("from");
                let to = subcommand.value_of("to");
                let result = cocogitto.get_colored_changelog(from, to)?;
                println!("{}", result);
            }

            INIT => {
                let subcommand = matches.subcommand_matches(INIT).unwrap();
                let init_path = subcommand.value_of("path").unwrap(); // safe unwrap via clap default value
                cocogitto::init(init_path)?;
            }
            _ => unreachable!(),
        }
    }
    Ok(())
}
