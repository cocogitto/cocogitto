use crate::commit::Commit;
use colored::*;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum CocoGittoError {
    #[error("{level} - {commit_message} - ({shorthand})\n\t {cause}\n")]
    CommitFormatError {
        level: String,
        shorthand: String,
        commit_message: String,
        cause: String,
    },
}
