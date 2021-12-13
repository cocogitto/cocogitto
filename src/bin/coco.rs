#![cfg(not(tarpaulin_include))]
mod cog_commit;

use cocogitto::CocoGitto;

use anyhow::Result;
use structopt::clap::{AppSettings, Shell};
use structopt::StructOpt;

const APP_SETTINGS: &[AppSettings] = &[
    AppSettings::UnifiedHelpMessage,
    AppSettings::ColoredHelp,
    AppSettings::DeriveDisplayOrder,
];

/// A command line tool to create conventional commits
///
/// Deprecated in favor of the `cog commit` utility
#[derive(StructOpt)]
#[structopt(name = "Coco", author = "Paul D. <paul.delafosse@protonmail.com>", settings = APP_SETTINGS)]
struct Cli {
    /// Conventional commit type
    #[structopt(name = "type", possible_values = &cog_commit::commit_types(), default_value_if("completion", None, "chore"))]
    typ: String,

    /// Commit description
    #[structopt(default_value_if("completion", None, ""))]
    message: String,

    /// Conventional commit scope
    scope: Option<String>,

    /// Create a BREAKING CHANGE commit
    #[structopt(short = "B", long)]
    breaking_change: bool,

    /// Open commit message in an editor
    #[structopt(short, long)]
    edit: bool,

    /// Generate shell completions
    #[structopt(long, conflicts_with = "type")]
    completion: Option<Shell>,
}

fn main() -> Result<()> {
    let cli = Cli::from_args();

    eprintln!("Warning: `coco` is deprecated, use `cog commit` instead");

    if let Some(shell) = cli.completion {
        Cli::clap().gen_completions_to("coco", shell, &mut std::io::stdout());
    } else {
        let cocogitto = CocoGitto::get()?;

        let (body, footer, breaking) = if cli.edit {
            cog_commit::edit_message(
                &cli.typ,
                &cli.message,
                cli.scope.as_deref(),
                cli.breaking_change,
            )?
        } else {
            (None, None, cli.breaking_change)
        };

        cocogitto.conventional_commit(&cli.typ, cli.scope, cli.message, body, footer, breaking)?;
    }

    Ok(())
}
