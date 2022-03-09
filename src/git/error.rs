use crate::git::status::Statuses;
use colored::Colorize;
use git2::Error;
use serde::de::StdError;
use std::fmt;
use std::fmt::{Display, Formatter};

#[derive(Debug)]
pub enum Git2Error {
    NothingToCommit {
        statuses: Option<Statuses>,
        branch: Option<String>,
    },
    ChangesNeedToBeCommitted(Statuses),
    FailedToInitializeRepository(git2::Error),
    FailedToOpenRepository(git2::Error),
    GitAddError(git2::Error),
    UnableToGetHead(git2::Error),
    PeelToCommitError(git2::Error),
    StashError(git2::Error),
    StatusError(git2::Error),
    CommitNotFound(git2::Error),
    Other(git2::Error),
    NoTagFound,
    CommitterNotFound,
}

#[derive(Debug)]
pub enum TagError {
    SemVerError {
        tag: String,
        err: semver::Error,
    },
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

impl PartialEq for TagError {
    fn eq(&self, _other: &Self) -> bool {
        matches!(self, _other)
    }
}

impl TagError {
    pub fn not_found(tag: &str, err: git2::Error) -> Self {
        TagError::NotFound {
            tag: tag.to_string(),
            err,
        }
    }

    pub fn semver(tag: &str, err: semver::Error) -> Self {
        TagError::SemVerError {
            tag: tag.to_string(),
            err,
        }
    }
}

impl Display for Git2Error {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Git2Error::NothingToCommit { branch, statuses } => {
                if let Some(branch) = branch {
                    writeln!(f, "On branch {}\n", branch)?;
                }

                match statuses {
                    Some(statuses) if !statuses.0.is_empty() => write!(f, "{}", statuses),
                    _ => writeln!(
                        f,
                        "nothing to commit (create/copy files and use \"git add\" to track)"
                    ),
                }
            }
            Git2Error::Other(_) => writeln!(f, "fatal error"),
            Git2Error::FailedToInitializeRepository(_) => {
                writeln!(f, "failed to initialize repository")
            }
            Git2Error::FailedToOpenRepository(_) => {
                writeln!(f, "failed to open repository")
            }
            Git2Error::GitAddError(_) => {
                writeln!(f, "failed to add content to index")
            }
            Git2Error::UnableToGetHead(_) => {
                writeln!(f, "failed to get repository HEAD")
            }
            Git2Error::PeelToCommitError(_) => {
                writeln!(f, "failed to peel git object to commit",)
            }
            Git2Error::CommitNotFound(_) => writeln!(f, "commit not found"),
            Git2Error::CommitterNotFound => writeln!(f, "unable to get committer"),
            Git2Error::NoTagFound => writeln!(f, "no tag found"),
            Git2Error::StashError(_) => writeln!(f, "git stash failed"),
            Git2Error::StatusError(_) => writeln!(f, "failed to get git statuses"),
            Git2Error::ChangesNeedToBeCommitted(statuses) => writeln!(
                f,
                "{}{}",
                "Cannot create tag: changes need to be committed".red(),
                statuses
            ),
        }?;

        match self {
            Git2Error::FailedToInitializeRepository(err)
            | Git2Error::FailedToOpenRepository(err)
            | Git2Error::GitAddError(err)
            | Git2Error::UnableToGetHead(err)
            | Git2Error::PeelToCommitError(err)
            | Git2Error::StashError(err)
            | Git2Error::StatusError(err)
            | Git2Error::Other(err)
            | Git2Error::CommitNotFound(err) => writeln!(f, "\ncause: {}", err),
            _ => fmt::Result::Ok(()),
        }
    }
}

impl Display for TagError {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            TagError::SemVerError { tag, err } => {
                writeln!(f, "tag `{}` is not SemVer compliant", tag)?;
                writeln!(f, "\tcause: {}", err)
            }
            TagError::InvalidPrefixError { prefix, tag } => {
                writeln!(f, "Expected a tag with prefix {}, got {}", prefix, tag)
            }
            TagError::NotFound { tag, err } => {
                writeln!(f, "tag {} not found", tag)?;
                writeln!(f, "\tcause: {}", err)
            }
            TagError::NoTag => writeln!(f, "unable to get any tag"),
            TagError::NoMatchFound { pattern, err } => {
                match pattern {
                    None => writeln!(f, "no tag found")?,
                    Some(pattern) => writeln!(f, "no tag matching pattern {}", pattern)?,
                }
                writeln!(f, "\tcause: {}", err)
            }
        }
    }
}

impl From<git2::Error> for Git2Error {
    fn from(err: Error) -> Self {
        Git2Error::Other(err)
    }
}

impl StdError for Git2Error {}
