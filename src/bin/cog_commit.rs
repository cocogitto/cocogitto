use std::fmt::Write;

use anyhow::{bail, Result};
use conventional_commit_parser::commit::{CommitType, Separator};
use itertools::Itertools;

use cocogitto::COMMITS_METADATA;

pub fn commit_types() -> Vec<&'static str> {
    COMMITS_METADATA
        .iter()
        .map(|(commit_type, _)| commit_type.as_ref())
        .collect()
}

pub fn explain_commit_type(commit_type: &str) -> String {
    let learn_more_msg = "Learn more about supported commit types :\r
- The Conventional Commit Specification : https://www.conventionalcommits.org \r
- The Angular Commit Convention : https://github.com/angular/angular/blob/master/CONTRIBUTING.md#type \r
- Custom commit types : https://docs.cocogitto.io/guide/#custom-commit-types";

    let semantic_versioning_minor_msg = "Semantic versioning : this correlates with MINOR";
    let semantic_versioning_patch_msg = "Semantic versioning : this correlates with PATCH";
    let semantic_versioning_no_correlation_msg = "Semantic versioning : no correlation";

    let commit_type = CommitType::from(commit_type);

    match commit_type {
        CommitType::BugFix => format!("fix: patches a bug in your codebase\n\n{}\n\n{}", semantic_versioning_minor_msg, learn_more_msg),
        CommitType::Feature => format!("feat: introduces a new feature to the codebase\n\n{}\n\n{}", semantic_versioning_patch_msg, learn_more_msg),
        CommitType::Chore => format!("chore: miscellaneous chores\n\n{}\n\n{}", semantic_versioning_no_correlation_msg, learn_more_msg),
        CommitType::Build => format!("build: changes that affect the build system or external dependencies (example: Maven, NPM, cargo)\n\n{}\n\n{}", semantic_versioning_no_correlation_msg, learn_more_msg),
        CommitType::Performances => format!("perf: a code change that improves performance\n\n{}\n\n{}", semantic_versioning_no_correlation_msg, learn_more_msg),
        CommitType::Ci => format!("ci: changes to the CI configuration files and scripts (example: Travis, Circle, Github Actions)\n\n{}\n\n{}", semantic_versioning_no_correlation_msg, learn_more_msg),
        CommitType::Documentation => format!("docs:Documentation only changes\n\n {}\n\n{}", semantic_versioning_no_correlation_msg, learn_more_msg),
        CommitType::Refactor => format!("refactor: a code change that neither fixes a bug nor adds a feature\n\n{}\n\n{}", semantic_versioning_no_correlation_msg, learn_more_msg),
        CommitType::Style => format!("style: a code change that improves performance\n\n{}\n\n{}", semantic_versioning_no_correlation_msg, learn_more_msg),
        CommitType::Test => format!("test: a adding missing tests or correcting existing tests\n\n{}\n\n{}", semantic_versioning_no_correlation_msg, learn_more_msg),
        CommitType::Revert => format!("revert: a rollback of a previous change\nSee here how conventional commit handles reverts : https://www.conventionalcommits.org/en/v1.0.0/#how-does-conventional-commits-handle-revert-commits\n\n{}\n\n{}", semantic_versioning_no_correlation_msg, learn_more_msg),
        unknown => format!("Commit type {} is unknown {}", unknown, learn_more_msg)
    }
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
