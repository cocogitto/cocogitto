use std::fmt::{self, Debug, Display, Formatter};

use crate::git::oid::OidOf;

use crate::conventional::error::ConventionalCommitError;
use colored::*;

#[derive(Debug)]
pub(crate) struct CogCheckReport {
    pub from: OidOf,
    pub errors: Vec<ConventionalCommitError>,
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

        writeln!(f, "{header}")?;

        for err in &self.errors {
            let underline = format!("{:>57}", " ").underline();
            writeln!(f, "{underline:>5}\n")?;
            write!(f, "{err}")?;
        }
        Ok(())
    }
}

// This is not meant to be unwrapped like other errors
// just to emit a warning on hook failure
pub(crate) struct BumpError {
    pub(crate) cause: String,
    pub(crate) version: String,
    pub(crate) stash_number: u32,
}

impl Display for BumpError {
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
        write!(f, "{header}\n{suggestion}")
    }
}
