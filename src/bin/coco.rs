#![cfg(not(tarpaulin_include))]
use std::fmt::Write;

use cocogitto::{CocoGitto, COMMITS_METADATA};

use anyhow::{bail, Result};
use conventional_commit_parser::commit::Footer;
use itertools::Itertools;
use structopt::clap::{AppSettings, Shell};
use structopt::StructOpt;

const APP_SETTINGS: &[AppSettings] = &[
    AppSettings::UnifiedHelpMessage,
    AppSettings::ColoredHelp,
    AppSettings::DeriveDisplayOrder,
];

const EDIT_TEMPLATE: &str = "# Enter the commit message for your changes.
# Lines starting with # will be ignored, and empty body/footer are allowed.
# Once you are done, save the changes and exit the editor.
# Remove all non-comment lines to abort.
#
";

fn prepare_header(typ: &str, message: &str, scope: Option<&str>) -> String {
    let mut header = typ.to_string();

    if let Some(scope) = scope {
        write!(&mut header, "({})", scope).unwrap();
    }

    write!(&mut header, ": {}", message).unwrap();

    header
}

fn prepare_edit_template(typ: &str, message: &str, scope: Option<&str>, breaking: bool) -> String {
    let mut template: String = EDIT_TEMPLATE.into();
    let header = prepare_header(typ, message, scope);

    if breaking {
        template.push_str("# WARNING: This will be marked as a breaking change!\n");
    }

    write!(
        &mut template,
        "{}\n\n# Message body\n\n\n# Message footer\n# For example, foo: bar\n\n\n",
        header
    )
    .unwrap();

    template
}

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

    if let Some(shell) = cli.completion {
        Cli::clap().gen_completions_to("coco", shell, &mut std::io::stdout());
    } else {
        let cocogitto = CocoGitto::get()?;

        let (body, footer, breaking) = if cli.edit {
            let template = prepare_edit_template(
                &cli.typ,
                &cli.message,
                cli.scope.as_deref(),
                cli.breaking_change,
            );

            let edited = edit::edit(&template)?;

            if edited.lines().all(|line| {
                let trimmed = line.trim_start();
                trimmed.is_empty() || trimmed.starts_with('#')
            }) {
                bail!("Aborted commit message edit");
            }

            let content = edited
                .lines()
                .filter(|&line| !line.trim_start().starts_with('#'))
                .join("\n");

            let mut cc = conventional_commit_parser::parse(content.trim())?;

            let footer = if !cc.footers.is_empty() {
                let Footer { token, content } = cc.footers.swap_remove(0);
                Some(format!("{}: {}", token, content.trim()))
            } else {
                None
            };

            (
                cc.body.map(|s| s.trim().to_string()),
                footer,
                cc.is_breaking_change || cli.breaking_change,
            )
        } else {
            (None, None, cli.breaking_change)
        };

        cocogitto.conventional_commit(&cli.typ, cli.scope, cli.message, body, footer, breaking)?;
    }

    Ok(())
}
