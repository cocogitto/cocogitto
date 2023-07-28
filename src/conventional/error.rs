use crate::git::error::{Git2Error, TagError};
use anyhow::anyhow;
use colored::Colorize;
use conventional_commit_parser::error::ParseError;
use serde::de::StdError;
use std::fmt;
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

#[derive(Debug)]
pub enum BumpError {
    Git2Error(Git2Error),
    TagError(TagError),
    SemVerError(semver::Error),
    FmtError(fmt::Error),
    NoCommitFound,
}

impl Display for BumpError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "failed to bump version\n")?;
        match self {
            BumpError::Git2Error(err) => writeln!(f, "\t{err}"),
            BumpError::TagError(err) => writeln!(f, "\t{err}"),
            BumpError::SemVerError(err) => writeln!(f, "\t{err}"),
            BumpError::FmtError(err) => writeln!(f, "\t{err}"),
            BumpError::NoCommitFound => writeln!(
                f,
                r#"cause: No conventional commit found to bump current version.
    Only feature, bug fix and breaking change commits will trigger an automatic bump.

suggestion: Please see https://conventionalcommits.org/en/v1.0.0/#summary for more information.
    Alternatively consider using `cog bump <--version <VERSION>|--auto|--major|--minor>`
"#
            ),
        }
    }
}

impl From<Git2Error> for BumpError {
    fn from(err: Git2Error) -> Self {
        Self::Git2Error(err)
    }
}

impl From<TagError> for BumpError {
    fn from(err: TagError) -> Self {
        Self::TagError(err)
    }
}

impl From<semver::Error> for BumpError {
    fn from(err: semver::Error) -> Self {
        Self::SemVerError(err)
    }
}

impl From<fmt::Error> for BumpError {
    fn from(err: fmt::Error) -> Self {
        Self::FmtError(err)
    }
}

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

impl StdError for ConventionalCommitError {}
impl StdError for BumpError {}
