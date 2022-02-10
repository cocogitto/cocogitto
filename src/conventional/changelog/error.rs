use serde::de::StdError;
use std::fmt::{Debug, Display, Formatter};
use std::io;
use std::path::PathBuf;

#[derive(Debug)]
pub enum ChangelogError {
    TemplateNotFound(PathBuf),
    TeraError(tera::Error),
    WriteError(io::Error),
    SeparatorNotFound(PathBuf),
}

impl Display for ChangelogError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            ChangelogError::TemplateNotFound(path) => {
                writeln!(f, "Changelog template not found at {path:?}")
            }
            ChangelogError::TeraError(err) => writeln!(f, "Error rendering changelog: {err}"),
            ChangelogError::WriteError(err) => writeln!(f, "Failed to write changelog: {err}"),
            ChangelogError::SeparatorNotFound(path) => writeln!(
                f,
                "Cannot find default separator '- - -' in {}",
                path.as_path().display()
            ),
        }
    }
}

impl From<io::Error> for ChangelogError {
    fn from(err: io::Error) -> Self {
        Self::WriteError(err)
    }
}

impl From<tera::Error> for ChangelogError {
    fn from(err: tera::Error) -> Self {
        Self::TeraError(err)
    }
}

impl StdError for ChangelogError {}
