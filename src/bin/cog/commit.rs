use std::fmt::Write;
use std::path::Path;
use std::{fs, io};

use cocogitto::COMMITS_METADATA;

use anyhow::{bail, Result};
use clap::builder::PossibleValuesParser;
use conventional_commit_parser::commit::Separator;
use itertools::Itertools;

pub fn commit_types() -> PossibleValuesParser {
    let types = COMMITS_METADATA
        .keys()
        .map(|commit_type| -> &str { commit_type.as_ref() })
        .sorted();

    types.into()
}

pub fn prepare_edit_message<P: AsRef<Path>>(
    typ: &str,
    message: &str,
    scope: Option<&str>,
    breaking: bool,
    path: P,
) -> io::Result<String> {
    let template = prepare_edit_template(typ, message, scope, breaking);
    fs::write(path, &template)?;
    Ok(template)
}

pub fn edit_message<P: AsRef<Path>>(
    path: P,
    breaking: bool,
) -> Result<(Option<String>, Option<String>, bool)> {
    edit::edit_file(path.as_ref())?;
    let edited = fs::read_to_string(path.as_ref())?;

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

const PRE_HEADER_DEFAULT: &str = "# Enter the commit message for your changes.
# Lines starting with # will be ignored, and empty body/footer are allowed.
# Once you are done, save the changes and exit the editor.
# Remove all non-comment lines to abort.
#
";
const BREAKING_WARNING_LINE: &str = "\n# WARNING: This will be marked as a breaking change!";
const PRE_BODY_DEFAULT: &str = "# Message body\n";
const PRE_FOOTER_DEFAULT: &str = "# Message footer\n# For example, foo: bar\n";

fn prepare_header(typ: &str, message: &str, scope: Option<&str>) -> String {
    let mut header = typ.to_string();

    if let Some(scope) = scope {
        write!(&mut header, "({scope})").unwrap();
    }

    write!(&mut header, ": {message}").unwrap();

    header
}

fn try_load_git_template() -> Option<String> {
    let git_config = git2::Config::open_default().ok()?;
    let git_template_entry = git_config.get_entry("commit.template").ok()?;
    let git_template_path = git_template_entry.value()?;
    let git_normalized_template_path = shellexpand::tilde(git_template_path).to_string();

    fs::read_to_string(git_normalized_template_path).ok()
}

fn parse_comments(git_template: Option<String>) -> (String, String, String) {
    let template = git_template.unwrap_or_default();
    let normalized = template.replace("\r\n", "\n").replace("\r", "\n");
    let paragraphs: Vec<_> = normalized
        .split("\n\n")
        .map(|s| s.trim())
        .filter(|s| !s.is_empty())
        .collect();

    match paragraphs.as_slice() {
        [] => (
            PRE_HEADER_DEFAULT.to_string(),
            PRE_BODY_DEFAULT.to_string(),
            PRE_FOOTER_DEFAULT.to_string(),
        ),
        [pre_header] => (pre_header.to_string(), String::new(), String::new()),
        [pre_header, pre_body] => (pre_header.to_string(), pre_body.to_string(), String::new()),
        [pre_header, pre_body @ .., pre_footer] => (
            pre_header.to_string(),
            pre_body.join("\n"),
            pre_footer.to_string(),
        ),
    }
}

fn prepare_edit_template(typ: &str, message: &str, scope: Option<&str>, breaking: bool) -> String {
    let git_template = try_load_git_template();
    let (mut template, pre_body, pre_footer) = parse_comments(git_template);

    let header = prepare_header(typ, message, scope);

    if breaking {
        template.push_str(BREAKING_WARNING_LINE);
    }

    write!(
        &mut template,
        "\n{header}\n\n{pre_body}\n\n{pre_footer}\n\n"
    )
    .unwrap();

    template
}

#[cfg(test)]
mod test {
    use super::*;
    use assert_cmd::{self, Command};
    use cmd_lib::run_cmd;
    use cocogitto::test_helpers;
    use sealed_test::prelude::*;

    #[test]
    /// Defining scopes in cog.toml and using an undefined one should error.
    fn test_parse_comments_none() {
        let (h, b, f) = parse_comments(None);
        assert_eq!(h, PRE_HEADER_DEFAULT, "Header comments must be default");
        assert_eq!(b, PRE_BODY_DEFAULT, "Body comments must be default");
        assert_eq!(f, PRE_FOOTER_DEFAULT, "Footer comments must be default");
    }

    #[test]
    fn test_parse_comments_empty() {
        let (h, b, f) = parse_comments(Some(String::new()));
        assert_eq!(h, PRE_HEADER_DEFAULT, "Header comments must be default");
        assert_eq!(b, PRE_BODY_DEFAULT, "Body comments must be default");
        assert_eq!(f, PRE_FOOTER_DEFAULT, "Footer comments must be default");
    }

    #[test]
    fn test_parse_comments_only_header() {
        let template = "# Header\n\n";
        let (h, b, f) = parse_comments(Some(String::from(template)));
        assert_eq!(
            h, "# Header",
            "Header comments must be parsed from template"
        );
        assert_eq!(b, "", "Body comments must be empty");
        assert_eq!(f, "", "Footer comments must be empty");
    }

    #[test]
    fn test_parse_comments_header_and_body() {
        let template = "# Header\n\n# Body\n\n";
        let (h, b, f) = parse_comments(Some(String::from(template)));
        assert_eq!(
            h, "# Header",
            "Header comments must be parsed from template"
        );
        assert_eq!(b, "# Body", "Body comments must be parsed from template");
        assert_eq!(f, "", "Footer comments must be empty");
    }

    #[test]
    fn test_parse_comments_all_parts() {
        let template = "# Header\n\n# Body\n\n# Footer\n\n";
        let (h, b, f) = parse_comments(Some(String::from(template)));
        assert_eq!(
            h, "# Header",
            "Header comments must be parsed from template"
        );
        assert_eq!(b, "# Body", "Body comments must be parsed from template");
        assert_eq!(
            f, "# Footer",
            "Footer comments must be parsed from template"
        );
    }

    #[test]
    fn test_parse_comments_multiple_body_parts() {
        let template = "# Header\n\n# Body 1\n\n# Body 2\n\n# Footer\n\n";
        let (h, b, f) = parse_comments(Some(String::from(template)));
        assert_eq!(
            h, "# Header",
            "Header comments must be parsed from template"
        );
        assert_eq!(
            b, "# Body 1\n# Body 2",
            "Body multiple comments must be parsed from template and combined"
        );
        assert_eq!(
            f, "# Footer",
            "Footer comments must be parsed from template"
        );
    }

    #[sealed_test]
    fn test_try_load_git_template_not_setup() {
        git_init()?;
        let template = try_load_git_template();
        assert_eq!(template, None, "Git template must be None");
    }

    #[sealed_test]
    fn test_try_load_git_template_setup_but_no_file() {
        git_init()?;
        // run_cmd!(git config --local commit.tempalte "./template");
        let template = try_load_git_template();
        assert_eq!(template, None, "Git template setup, but can't read file");
    }

    #[sealed_test]
    fn test_try_load_git_template_ok() {
        git_init()?;
        // run_cmd!(git config --local commit.tempalte "./template");
        // fs:write("./template", "# Template file")?;
        let template = try_load_git_template();
        assert_eq!(template, Some("# Template file"), "Template file read ok");
    }
}
