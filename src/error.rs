use crate::OidOf;
use colored::*;
use std::fmt;
use std::fmt::{Debug, Display, Formatter};

#[derive(Debug)]
pub(crate) enum CocogittoError {
    CommitFormat {
        oid: String,
        summary: String,
        author: String,
        cause: String,
    },
    CommitTypeNotAllowed {
        oid: String,
        summary: String,
        commit_type: String,
        author: String,
    },
    NothingToCommitWithBranch {
        branch: String,
    },
    NothingToCommit,
    Semver {
        level: String,
        cause: String,
    },
    Git {
        level: String,
        cause: String,
    },
}

impl Display for CocogittoError {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            CocogittoError::CommitFormat {
                summary,
                oid,
                author,
                cause,
            } => {
                let error_header = "Errored commit:".bold().red();
                let author = format!("<{}>", author).blue();
                writeln!(
                    f,
                    "{header}{oid} {author}\n\t{message_title}'{summary}'\n\t{cause_title}{cause}",
                    header = error_header,
                    oid = oid,
                    author = author,
                    message_title = "Commit message:".yellow().bold(),
                    summary = summary.italic(),
                    cause_title = "Cause:".yellow().bold(),
                    cause = cause
                )
            }
            CocogittoError::CommitTypeNotAllowed {
                summary,
                commit_type,
                oid,
                author,
            } => {
                let error_header = "Errored commit:".bold().red();
                let author = format!("<{}>", author).blue();
                writeln!(
                    f,
                    "{header}{oid} {author}\n\t{message}'{summary}'\n\t{cause}Commit type `{commit_type}` not allowed",
                    header = error_header,
                    oid = oid,
                    author = author,
                    message = "Commit message:".yellow().bold(),
                    cause = "Cause:".yellow().bold(),
                    summary = summary.italic(),
                    commit_type = commit_type.red()
                )
            }
            CocogittoError::NothingToCommitWithBranch { branch } => {
                writeln!(f, "On branch {}\nNothing to commit", branch)
            }
            CocogittoError::NothingToCommit => {
                writeln!(f, "Nothing to commit")
            }
            CocogittoError::Semver { cause, level } => {
                writeln!(f, "{}:\n\t{}\n", level, cause)
            }
            CocogittoError::Git { level, cause } => {
                writeln!(f, "{}:\n\t{}\n", level, cause)
            }
        }
    }
}

#[derive(Debug)]
pub(crate) struct CogCheckReport {
    pub from: OidOf,
    pub errors: Vec<anyhow::Error>,
}

impl Display for CogCheckReport {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        let header = format!(
            "\nFound {} non compliant commits in {}..HEAD:\n",
            self.errors.len(),
            self.from
        )
        .red()
        .bold();

        writeln!(f, "{}", header)?;

        for err in &self.errors {
            let underline = format!("{:>57}", " ").underline();
            writeln!(f, "{:>5}\n", underline)?;
            write!(f, "{}", err)?;
        }
        Ok(())
    }
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
