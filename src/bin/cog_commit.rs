use std::fmt::Write;

use anyhow::{bail, Result};
use colored::Colorize;
use conventional_commit_parser::commit::{CommitType, Separator};
use itertools::Itertools;

use cocogitto::settings::Settings;
use cocogitto::{COMMITS_METADATA, SETTINGS};

pub fn commit_types() -> Vec<&'static str> {
    COMMITS_METADATA
        .iter()
        .map(|(commit_type, _)| commit_type.as_ref())
        .collect()
}

pub fn explain_commit_type(commit_type: &str) -> String {
    let semantic_versioning_minor_msg = format!(
        "{} : this correlates with {}",
        "Semantic versioning".italic(),
        "MINOR".bold()
    );
    let semantic_versioning_patch_msg = format!(
        "{} : this correlates with {}",
        "Semantic versioning".italic(),
        "PATCH".bold()
    );
    let semantic_versioning_no_correlation_msg =
        format!("{} : no correlation", "Semantic versioning".italic());

    let commit_type = CommitType::from(commit_type);

    match commit_type {
        CommitType::BugFix => {
            get_formatted_explain_msg(&commit_type, &semantic_versioning_patch_msg)
        }
        CommitType::Feature => {
            get_formatted_explain_msg(&commit_type, &semantic_versioning_minor_msg)
        }
        _ => get_formatted_explain_msg(&commit_type, &semantic_versioning_no_correlation_msg),
    }
}

fn get_formatted_explain_msg(
    commit_type: &CommitType,
    semantic_versioning_correlation_msg: &str,
) -> String {
    let learn_more_msg = "Learn more about supported commit types :\r
- The Conventional Commit Specification : https://www.conventionalcommits.org \r
- The Angular Commit Convention : https://github.com/angular/angular/blob/master/CONTRIBUTING.md#type \r
- Custom commit types : https://docs.cocogitto.io/guide/#custom-commit-types";

    format!(
        "{}: {}\n\n{}\n\n{}",
        commit_type.as_ref().yellow().bold(),
        get_commit_type_description(commit_type),
        semantic_versioning_correlation_msg,
        learn_more_msg
    )
}

fn get_commit_type_description(commit_type: &CommitType) -> String {
    COMMITS_METADATA
        .get(commit_type)
        .map(|v| {
            v.description
                .clone()
                .unwrap_or("no description for this commit type".to_string())
        })
        .unwrap_or("no commit of this type exists".to_string())
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

#[cfg(test)]
mod test {
    use speculoos::prelude::*;

    use crate::explain_commit_type;

    #[test]
    fn given_known_type_should_not_map_to_default_message() {
        let string = explain_commit_type("fix");

        assert_that!(string).does_not_contain(&"is unknown");
    }

    #[test]
    fn given_unknown_commit_type_should_map_to_default_message() {
        let string = explain_commit_type("toto");

        assert_that!(string).starts_with(&"Commit type toto is unknown");
    }
}
