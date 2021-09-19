#![cfg(not(tarpaulin_include))]
use anyhow::Result;
use structopt::clap::{AppSettings, Shell};
use structopt::StructOpt;

use cocogitto::CocoGitto;
use cocogitto::COMMITS_METADATA;

const APP_SETTINGS: &[AppSettings] = &[
    AppSettings::UnifiedHelpMessage,
    AppSettings::ColoredHelp,
    AppSettings::DeriveDisplayOrder,
];

fn metadata_as_commit_types() -> Vec<&'static str> {
    COMMITS_METADATA
        .iter()
        .map(|(commit_type, _)| commit_type.as_ref())
        .collect()
}

/// A command line tool to create conventional commits
#[derive(StructOpt)]
#[structopt(name = "Coco", author = "Paul D. <paul.delafosse@protonmail.com>", settings = APP_SETTINGS)]
struct Cli {
    /// Conventional commit type
    #[structopt(name = "type", possible_values = &metadata_as_commit_types(), default_value_if("completion", None, "chore"))]
    typ: String,

    /// Commit description
    #[structopt(default_value_if("completion", None, ""))]
    message: String,

    /// Conventional commit scope
    scope: Option<String>,

    /// Commit message body
    body: Option<String>,

    /// Commit message footer
    footer: Option<String>,

    /// Create a BREAKING CHANGE commit
    #[structopt(short = "B", long)]
    breaking_change: bool,

    /// Generate shell completions
    #[structopt(long, conflicts_with = "type")]
    completion: Option<Shell>,
}

fn main() -> Result<()> {
    let cli = Cli::from_args();

    if let Some(shell) = cli.completion {
        Cli::clap().gen_completions_to("coco", shell, &mut std::io::stdout());
    } else {
        let cocogitto = CocoGitto::get()?;

        cocogitto.conventional_commit(
            &cli.typ,
            cli.scope,
            cli.message,
            cli.body,
            cli.footer,
            cli.breaking_change,
        )?;
    }

    Ok(())
}
