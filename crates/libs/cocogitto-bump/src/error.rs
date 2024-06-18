use cocogitto_git::error::Git2Error;
use cocogitto_tag::error::TagError;
use std::fmt;
use std::fmt::{Display, Formatter};

#[derive(Debug)]
pub enum BumpError {
    Git2Error(Git2Error),
    TagError(TagError),
    FmtError(fmt::Error),
    NoCommitFound,
}

impl Display for BumpError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "failed to bump version\n")?;
        match self {
            BumpError::Git2Error(err) => writeln!(f, "\t{err}"),
            BumpError::TagError(err) => writeln!(f, "\t{err}"),
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

impl From<fmt::Error> for BumpError {
    fn from(err: fmt::Error) -> Self {
        Self::FmtError(err)
    }
}

impl std::error::Error for BumpError {}
