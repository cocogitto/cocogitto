#![cfg(not(tarpaulin_include))]
mod cog_commit;

use cocogitto::CocoGitto;

use anyhow::Result;
use clap::{AppSettings, IntoApp, Parser};
use clap_complete::Shell;

/// A command line tool to create conventional commits
///
/// Deprecated in favor of the `cog commit` utility
#[derive(Parser)]
#[clap(setting = AppSettings::DeriveDisplayOrder)]
#[clap(
    version,
    name = "Coco",
    author = "Paul D. <paul.delafosse@protonmail.com>"
)]
struct Cli {
    /// Conventional commit type
    #[clap(name = "type", value_name = "TYPE", possible_values = cog_commit::commit_types(), default_value_if("completion", None, Some("chore")))]
    typ: String,

    /// Commit description
    #[clap(default_value_if("completion", None, Some("")))]
    message: String,

    /// Conventional commit scope
    scope: Option<String>,

    /// Create a BREAKING CHANGE commit
    #[clap(short = 'B', long)]
    breaking_change: bool,

    /// Open commit message in an editor
    #[clap(short, long)]
    edit: bool,

    /// Generate shell completions
    #[clap(long, conflicts_with = "type")]
    completion: Option<Shell>,
}

fn main() -> Result<()> {
    let cli = Cli::parse();

    eprintln!("Warning: `coco` is deprecated, use `cog commit` instead");

    if let Some(shell) = cli.completion {
        clap_complete::generate(shell, &mut Cli::into_app(), "coco", &mut std::io::stdout());
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
