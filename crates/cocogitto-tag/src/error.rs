use serde::de::StdError;
use std::fmt;
use std::fmt::{Display, Formatter};

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

impl Display for TagError {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            TagError::SemVerError { tag, err } => {
                writeln!(f, "tag `{tag}` is not SemVer compliant")?;
                writeln!(f, "\tcause: {err}")
            }
            TagError::InvalidPrefixError { prefix, tag } => {
                writeln!(f, "Expected a tag with prefix {prefix}, got {tag}")
            }
            TagError::NotFound { tag, err } => {
                writeln!(f, "tag {tag} not found")?;
                writeln!(f, "\tcause: {err}")
            }
            TagError::NoTag => writeln!(f, "unable to get any tag"),
            TagError::NoMatchFound { pattern, err } => {
                match pattern {
                    None => writeln!(f, "no tag found")?,
                    Some(pattern) => writeln!(f, "no tag matching pattern {pattern}")?,
                }
                writeln!(f, "\tcause: {err}")
            }
        }
    }
}
