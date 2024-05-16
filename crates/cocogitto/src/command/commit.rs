use crate::CocoGitto;
use crate::CommitHook::CommitMessage;
use anyhow::Result;
use cocogitto_commit::{validate_and_get_message, Commit};
use log::info;
use std::fs;

#[derive(Default)]
pub struct CommitOptions<'a> {
    pub commit_type: &'a str,
    pub scope: Option<String>,
    pub summary: String,
    pub body: Option<String>,
    pub footer: Option<String>,
    pub breaking: bool,
    pub sign: bool,
    pub add_files: bool,
    pub update_files: bool,
}

impl CocoGitto {
    pub fn conventional_commit(&self, opts: CommitOptions) -> Result<()> {
        let conventional_message = validate_and_get_message(
            opts.commit_type,
            opts.scope,
            opts.summary,
            opts.body,
            opts.footer,
            opts.breaking,
        )?;

        if opts.add_files {
            self.repository.add_all()?;
        }

        if opts.update_files {
            self.repository.update_all()?;
        }

        // Git commit
        let sign = opts.sign || self.repository.gpg_sign();
        fs::write(self.prepare_edit_message_path(), &conventional_message)?;
        self.run_commit_hook(CommitMessage)?;
        let oid = self.repository.commit(&conventional_message, sign, false)?;

        // Pretty print a conventional commit summary
        let commit = self.repository.0.find_commit(oid)?;
        let commit = Commit::from_git_commit(&commit)?;
        info!("{}", commit);

        Ok(())
    }
}
