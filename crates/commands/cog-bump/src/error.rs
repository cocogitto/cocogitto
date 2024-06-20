use std::fmt::{self, Display, Formatter};

use colored::*;

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
