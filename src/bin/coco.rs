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

fn main() -> Result<()> {
    let cocogitto = CocoGitto::get()?;

    let matches =  App::new("Cocogito")
        .settings(APP_SETTINGS)
        .version(env!("CARGO_PKG_VERSION"))
        .author("Paul D. <paul.delafosse@protonmail.com>")
        .about("A conventional commit compliant, changelog and commit generator")
        .long_about("Conventional Commit Git Terminal Overlord is a tool to help you use the conventional commit specification")
        .subcommands(CocoGitto::get_commit_metadata()
            .iter()
            .map(|(commit_type, commit_config)| {
                SubCommand::with_name(commit_type.get_key_str())
                    .settings(SUBCOMMAND_SETTINGS)
                    .about(commit_config.help_message.as_str())
                    .help(commit_config.help_message.as_str())
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
            .collect::<Vec<App>>()
        ).get_matches();

    if let Some(commit_subcommand) = matches.subcommand_name() {
        if let Some(args) = matches.subcommand_matches(commit_subcommand) {
            let message = args.value_of("message").unwrap().to_string();
            let scope = args.value_of("scope").map(|scope| scope.to_string());
            let body = args.value_of("body").map(|body| body.to_string());
            let footer = args.value_of("footer").map(|footer| footer.to_string());
            let breaking_change = args.is_present("breaking-change");
            cocogitto.conventional_commit(
                commit_subcommand,
                scope,
                message,
                body,
                footer,
                breaking_change,
            )?;
        }
    }
    Ok(())
}
