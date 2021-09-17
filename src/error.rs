use colored::*;
use std::fmt;
use std::fmt::Formatter;
use thiserror::Error;

#[derive(Error, Debug)]
pub(crate) enum ErrorKind {
    #[error("{level} - {commit_message} - ({shorthand})\n\t{cause}\n\t{additional_info}")]
    CommitFormat {
        level: String,
        shorthand: String,
        commit_message: String,
        cause: String,
        additional_info: String,
    },
    #[error("On branch {branch}\nNothing to commit")]
    NothingToCommitWithBranch { branch: String },
    #[error("Nothing to commit")]
    NothingToCommit,
    #[error("{level}:\n\t{cause}\n")]
    Semver { level: String, cause: String },
    #[error("{level}:\n\t{cause}\n")]
    Git { level: String, cause: String },
}

// This is not meant to be unwrapped like other errors
// just to emit a warning on hook failure
pub(crate) struct PreHookError {
    pub(crate) cause: String,
    pub(crate) version: String,
    pub(crate) stash_number: u32,
}

impl fmt::Display for PreHookError {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        let header = format!(
            "Error: {} `{}` {}",
            "prehook run".red(),
            self.cause,
            "failed".red()
        );
        let stash_ref = format!("`cog_bump_{}`", self.version);
        let suggestion = format!(
            "\tAll changes made during hook runs have been stashed on {}\n\
        \tyou can run `git stash apply stash@{}` to restore these changes.",
            stash_ref, self.stash_number
        );
        write!(f, "{}\n{}", header, suggestion)
    }
}
