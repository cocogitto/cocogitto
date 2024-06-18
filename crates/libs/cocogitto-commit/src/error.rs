use anyhow::anyhow;
use colored::Colorize;
use conventional_commit_parser::error::ParseError;
use std::fmt::{Display, Formatter};

#[derive(Debug, Clone)]
pub enum ConventionalCommitError {
    CommitFormat {
        oid: String,
        summary: String,
        author: String,
        cause: ParseError,
    },
    CommitTypeNotAllowed {
        oid: String,
        summary: String,
        commit_type: String,
        author: String,
    },
    ParseError(ParseError),
}

impl From<ParseError> for ConventionalCommitError {
    fn from(value: ParseError) -> Self {
        ConventionalCommitError::ParseError(value)
    }
}

impl std::error::Error for ConventionalCommitError {}

impl Display for ConventionalCommitError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            ConventionalCommitError::CommitFormat {
                summary,
                oid,
                author,
                cause,
            } => {
                let error_header = "Errored commit: ".bold().red();
                let author = format!("<{author}>").blue();
                let cause = anyhow!(cause.clone());
                let cause = format!("{cause:?}")
                    .lines()
                    .collect::<Vec<&str>>()
                    .join("\n\t");

                writeln!(
                    f,
                    "{}{} {}\n\t{message_title}'{summary}'\n\t{cause_title}{}",
                    error_header,
                    oid,
                    author,
                    cause,
                    message_title = "Commit message: ".yellow().bold(),
                    summary = summary.italic(),
                    cause_title = "Error: ".yellow().bold(),
                )
            }
            ConventionalCommitError::CommitTypeNotAllowed {
                summary,
                commit_type,
                oid,
                author,
            } => {
                let error_header = "Errored commit: ".bold().red();
                let author = format!("<{author}>").blue();
                writeln!(
                    f,
                    "{}{} {}\n\t{message}'{summary}'\n\t{cause}Commit type `{commit_type}` not allowed",
                    error_header,
                    oid,
                    author,
                    message = "Commit message:".yellow().bold(),
                    cause = "Error:".yellow().bold(),
                    summary = summary.italic(),
                    commit_type = commit_type.red()
                )
            }
            ConventionalCommitError::ParseError(err) => {
                let err = anyhow!(err.clone());
                writeln!(f, "{err:?}")
            }
        }
    }
}
