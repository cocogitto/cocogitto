use anyhow::Result;
use clap::{App, AppSettings, Arg, SubCommand};
use cocogitto::CocoGitto;

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

const PUBLISH: &str = "publish";
const CHECK: &str = "check";
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
                .about("")
                .arg(Arg::with_name("edit"))
        )
        .subcommand(
            SubCommand::with_name(PUBLISH)
                .settings(SUBCOMMAND_SETTINGS)
                .about("")
                .arg(Arg::with_name("minor")
                    .help("")
                    .short("m")
                    .long("minor")
                    .required_unless_one(&["major", "patch"])
                )
                .arg(Arg::with_name("major")
                    .help("")
                    .short("M")
                    .long("major")
                    .required_unless_one(&["minor", "patch"])
                )
                .arg(Arg::with_name("patch")
                    .help("")
                    .short("p")
                    .long("patch")
                    .required_unless_one(&["minor", "major"])
                ))
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
            PUBLISH => todo!(),
            CHECK => cocogitto.check()?,
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

                    cocogitto.conventional_commit(commit_type, scope, message)?;
                }
            }
        }
    }
    Ok(())
}
