use anyhow::Result;
use clap::{App, AppSettings, Arg, SubCommand};
use cocogitto::{CocoGitto, VersionIncrement};
use moins::Moins;
use std::process::exit;

const APP_SETTINGS: &[AppSettings] = &[
    AppSettings::SubcommandRequiredElseHelp,
    AppSettings::UnifiedHelpMessage,
    AppSettings::ColoredHelp,
    AppSettings::VersionlessSubcommands,
];

const SUBCOMMAND_SETTINGS: &[AppSettings] = &[
    AppSettings::UnifiedHelpMessage,
    AppSettings::ColoredHelp,
    AppSettings::VersionlessSubcommands,
];

const BUMP: &str = "bump";
const CHECK: &str = "check";
const LOG: &str = "log";
const VERIFY: &str = "verify";
const CHANGELOG: &str = "changelog";

fn main() -> Result<()> {
    let cocogitto = CocoGitto::get().unwrap_or_else(|err| panic!("{}", err));
    let commit_types = cocogitto.commit_types();

    let commit_subcommands = commit_types
        .iter()
        .map(|commit_type| {
            SubCommand::with_name(commit_type)
                .settings(SUBCOMMAND_SETTINGS)
                .about("Create prefixed commit")
                .help("Create a pre-formatted commit")
                .arg(Arg::with_name("message").help("The commit message"))
                .arg(Arg::with_name("scope").help("The scope of the commit message"))
                .arg(Arg::with_name("body").help("The body of the commit message"))
                .arg(Arg::with_name("footer").help("The footer of the commit message"))
                .arg(
                    Arg::with_name("breaking-change")
                        .help("BREAKING CHANGE commit")
                        .short("B")
                        .long("breaking-change"),
                )
        })
        .collect::<Vec<App>>();

    let matches = App::new("Coco Gitto")
        .settings(APP_SETTINGS)
        .version(env!("CARGO_PKG_VERSION"))
        .author("Paul D. <paul.delafosse@protonmail.com>")
        .about("A conventional commit compliant, changelog and commit generator")
        .long_about("Conventional Commit Git Terminal Overlord is a tool to help you use the conventional commit specification")
        .subcommand(
            SubCommand::with_name(CHECK)
                .about("Verify all commit message against the conventional commit specification")
                .arg(Arg::with_name("edit")
                    .help("Interactively rename invalid commit message")
                    .short("e")
                    .long("edit")
                )
        )
        .subcommand(SubCommand::with_name(LOG).about("Like git log but for conventional commits"))
        .subcommand(
            SubCommand::with_name(VERIFY)
                .about("Verify a single commit message")
                .arg(Arg::with_name("message").help("The commit message"))
        )
        .subcommand(
            SubCommand::with_name(BUMP)
                .settings(SUBCOMMAND_SETTINGS)
                .about("Commit changelog from latest tag to HEAD and create a new tag")
                .arg(Arg::with_name("version")
                    .help("Manually set the next version")
                    .short("v")
                    .takes_value(true)
                    .long("version")
                    .required_unless_one(&["auto", "major", "patch", "minor"])
                )
                .arg(Arg::with_name("auto")
                    .help("Atomatically suggest the next version")
                    .short("a")
                    .long("auto")
                    .required_unless_one(&["version", "major", "patch", "minor"])
                )
                .arg(Arg::with_name("major")
                    .help("Increment the major version")
                    .short("M")
                    .long("major")
                    .required_unless_one(&["version", "auto", "patch", "minor"])
                )
                .arg(Arg::with_name("patch")
                    .help("Increment the patch version")
                    .short("p")
                    .long("patch")
                    .required_unless_one(&["version", "auto", "major", "minor"])
                )
                .arg(Arg::with_name("minor")
                    .help("Increment the minor version")
                    .short("m")
                    .long("minor")
                    .required_unless_one(&["version", "auto", "patch", "major"])
                )
        )
        .subcommand(SubCommand::with_name(CHANGELOG)
            .settings(SUBCOMMAND_SETTINGS)
            .about("Display a changelog for a given commit oid range")
            .arg(Arg::with_name("from")
                .help("Generate the changelog from this commit or tag ref, default latest tag")
                .short("f")
                .long("from")
                .takes_value(true)
            )
            .arg(Arg::with_name("to")
                .help("Generate the changelog to this commit or tag ref, default HEAD")
                .short("t")
                .long("to")
                .takes_value(true)))
        .subcommands(commit_subcommands)
        .get_matches();

    if let Some(subcommand) = matches.subcommand_name() {
        match subcommand {
            BUMP => {
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

                cocogitto.create_version(increment)?
            }
            VERIFY => {
                let subcommand = matches.subcommand_matches(VERIFY).unwrap();
                let message = subcommand.value_of("message").unwrap();

                match CocoGitto::verify(message) {
                    Ok(()) => exit(0),
                    Err(err) => {
                        eprintln!("{}", err);
                        exit(1);
                    }
                }
            }

            CHECK => {
                let subcommand = matches.subcommand_matches(CHECK).unwrap();
                if subcommand.is_present("edit") {
                    cocogitto.check_and_edit()?;
                } else {
                    cocogitto.check()?
                }
            }
            LOG => {
                let mut content = cocogitto.get_log()?;
                Moins::run(&mut content, None);
            }
            CHANGELOG => {
                let subcommand = matches.subcommand_matches(CHANGELOG).unwrap();
                let from = subcommand.value_of("from");
                let to = subcommand.value_of("to");
                let result = cocogitto.get_changelog(from, to)?;
                println!("{}", result);
            }

            commit_type => {
                if let Some(args) = matches.subcommand_matches(commit_type) {
                    let message = args.value_of("message").unwrap().to_string();
                    let scope = args.value_of("scope").map(|scope| scope.to_string());
                    let body = args.value_of("body").map(|body| body.to_string());
                    let footer = args.value_of("footer").map(|footer| footer.to_string());
                    let breaking_change = args.is_present("breaking-change");
                    cocogitto.conventional_commit(
                        commit_type,
                        scope,
                        message,
                        body,
                        footer,
                        breaking_change,
                    )?;
                }
            }
        }
    }
    Ok(())
}
