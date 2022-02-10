use crate::git::status::Statuses;
use colored::Colorize;
use git2::Error;
use serde::de::StdError;
use std::fmt;
use std::fmt::{Display, Formatter};

#[derive(Debug)]
pub enum Git2Error {
    NothingToCommitWithBranch { branch: String },
    NothingToCommit,
    FailedToInitializeRepository(git2::Error),
    FailedToOpenRepository(git2::Error),
    GitAddError(git2::Error),
    UnableToGetHead(git2::Error),
    PeelToCommitError(git2::Error),
    StashError(git2::Error),
    StatusError(git2::Error),
    CommitNotFound(git2::Error),
    ChangesNeedToBeCommitted(Statuses),
    CommitterNotFound,
    NoTagFound,
    Other(git2::Error),
}

#[derive(Debug)]
pub enum TagError {
    SemvVerError(semver::Error),
    InvalidPrefixError {
        prefix: String,
        tag: String,
    },
    NotFound {
        tag: String,
        err: git2::Error,
    },
    NoMatchFound {
        err: git2::Error,
        pattern: Option<String>,
    },
    NoTag,
}

impl StdError for TagError {}

impl From<semver::Error> for TagError {
    fn from(err: semver::Error) -> Self {
        TagError::SemvVerError(err)
    }
}

impl TagError {
    pub fn not_found(tag: &str, err: git2::Error) -> Self {
        TagError::NotFound {
            tag: tag.to_string(),
            err,
        }
    }
}

impl Display for Git2Error {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Git2Error::NothingToCommitWithBranch { branch } => {
                writeln!(f, "On branch {branch}\nNothing to commit")
            }
            Git2Error::NothingToCommit => writeln!(f, "Nothing to commit"),
            Git2Error::Other(err) => writeln!(f, "Unexpected git error: {err}"),
            Git2Error::FailedToInitializeRepository(err) => {
                writeln!(f, "Failed to initialize repository: {err}")
            }
            Git2Error::FailedToOpenRepository(err) => {
                writeln!(f, "Failed to open repository: {err}")
            }
            Git2Error::GitAddError(err) => {
                writeln!(f, "Error while adding content to index: {err}")
            }
            Git2Error::UnableToGetHead(err) => {
                writeln!(f, "Error trying to get repository HEAD: {err}")
            }
            Git2Error::PeelToCommitError(err) => {
                writeln!(f, "Error trying too peel commit: {err}",)
            }
            Git2Error::CommitNotFound(err) => writeln!(f, "Commit not found: {err}"),
            Git2Error::CommitterNotFound => writeln!(f, "Unable to get committer"),
            Git2Error::NoTagFound => writeln!(f, "No tag found"),
            Git2Error::StashError(err) => writeln!(f, "Git stash failed: {err}"),
            Git2Error::StatusError(err) => writeln!(f, "Failed to get git statuses: {err}"),
            Git2Error::ChangesNeedToBeCommitted(statuses) => writeln!(
                f,
                "{}{statuses}",
                "Cannot create tag: changes need to be committed".red()
            ),
        }
    }
}

impl Display for TagError {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            TagError::SemvVerError(err) => writeln!(f, "Error parsing version number: {err}"),
            TagError::InvalidPrefixError { prefix, tag } => {
                writeln!(f, "Expected a tag with prefix {prefix}, got {tag}")
            }
            TagError::NotFound { tag, err } => writeln!(f, "Tag {tag} not found: {err}"),
            TagError::NoTag => writeln!(f, "Unable to get any tag"),
            TagError::NoMatchFound { err, pattern } => match pattern {
                None => writeln!(f, "No tag found: {err}"),
                Some(pattern) => writeln!(f, "No tag matching pattern {pattern}: {err}"),
            },
        }
    }
}

impl From<git2::Error> for Git2Error {
    fn from(err: Error) -> Self {
        Git2Error::Other(err)
    }
}

impl StdError for Git2Error {}
