#![cfg(not(tarpaulin_include))]
use std::fmt::Write;

use cocogitto::COMMITS_METADATA;

use anyhow::{bail, Result};
use conventional_commit_parser::commit::Separator;
use itertools::Itertools;

pub fn commit_types() -> Vec<&'static str> {
    COMMITS_METADATA
        .iter()
        .map(|(commit_type, _)| commit_type.as_ref())
        .collect()
}

pub fn edit_message(
    typ: &str,
    message: &str,
    scope: Option<&str>,
    breaking: bool,
) -> Result<(Option<String>, Option<String>, bool)> {
    let template = prepare_edit_template(typ, message, scope, breaking);

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

    let cc = conventional_commit_parser::parse(content.trim())?;

    let footers: Option<String> = if cc.footers.is_empty() {
        None
    } else {
        Some(
            cc.footers
                .iter()
                .map(|footer| {
                    let separator = match footer.token_separator {
                        Separator::Colon => ": ",
                        Separator::Hash => " #",
                        Separator::ColonWithNewLine => " \n",
                    };
                    format!("{}{}{}", footer.token, separator, footer.content)
                })
                .join("\n"),
        )
    };

    Ok((
        cc.body.map(|s| s.trim().to_string()),
        footers,
        cc.is_breaking_change || breaking,
    ))
}

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
