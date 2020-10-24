use crate::git::status::Statuses;
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
    #[error("{statuses}")]
    NothingToCommit { statuses: Statuses },
    #[error("{level}:\n\t{cause}\n")]
    Semver { level: String, cause: String },
    #[error("{level}:\n\t{cause}\n")]
    Git { level: String, cause: String },
}
