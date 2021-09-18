#![cfg(not(tarpaulin_include))]
use anyhow::Result;
use clap::{App, AppSettings, Arg, Shell, SubCommand};
use cocogitto::CocoGitto;
use cocogitto::COMMITS_METADATA;

const APP_SETTINGS: &[AppSettings] = &[
    AppSettings::ArgsNegateSubcommands,
    AppSettings::SubcommandsNegateReqs,
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

const GENERATE_COMPLETIONS: &str = "generate-completions";

fn main() -> Result<()> {
    let matches = app().get_matches();

    if let Some(subcommand) = matches.subcommand_matches(GENERATE_COMPLETIONS) {
        let for_shell = match subcommand.value_of("type").unwrap() {
            "bash" => Shell::Bash,
            "elvish" => Shell::Elvish,
            "fish" => Shell::Fish,
            "zsh" => Shell::Zsh,
            _ => unreachable!(),
        };
        app().gen_completions_to("coco", for_shell, &mut std::io::stdout());
    } else {
        let cocogitto = CocoGitto::get()?;

        let commit_type = matches.value_of("type").unwrap().to_string();
        let message = matches.value_of("message").unwrap().to_string();
        let scope = matches.value_of("scope").map(|scope| scope.to_string());
        let body = matches.value_of("body").map(|body| body.to_string());
        let footers = matches.value_of("footer").map(|footer| footer.to_string());
        let breaking_change = matches.is_present("breaking-change");

        cocogitto.conventional_commit(
            &commit_type,
            scope,
            message,
            body,
            footers,
            breaking_change,
        )?;
    }

    Ok(())
}

fn app<'a, 'b>() -> App<'a, 'b> {
    let keys: Vec<&str> = COMMITS_METADATA
        .iter()
        .map(|(commit_type, _)| commit_type.as_ref())
        .collect();

    App::new("Coco")
        .settings(APP_SETTINGS)
        .version(env!("CARGO_PKG_VERSION"))
        .author("Paul D. <paul.delafosse@protonmail.com>")
        .about("A command line tool to create conventional commits")
        .arg(
            Arg::with_name("type")
                .help("The type of the commit message")
                .possible_values(&keys)
                .required(true),
        )
        .arg(
            Arg::with_name("message")
                .help("The type of the commit message")
                .required(true),
        )
        .arg(Arg::with_name("scope").help("The scope of the commit message"))
        .arg(Arg::with_name("body").help("The body of the commit message"))
        .arg(Arg::with_name("footer").help("The footer of the commit message"))
        .arg(
            Arg::with_name("breaking-change")
                .help("BREAKING CHANGE commit")
                .short("B")
                .long("breaking-change"),
        )
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
