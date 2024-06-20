use anyhow::bail;
use cocogitto_commit::CommitType;
use cog_command::CogCommand;
use std::fs;
use std::path::PathBuf;

pub struct CogVerifyCommand {
    pub message: Option<String>,
    pub file: Option<PathBuf>,
    pub ignore_merge_commits: bool,
    pub allowed_commits: Vec<CommitType>,
}

impl CogCommand for CogVerifyCommand {
    fn execute(self) -> anyhow::Result<()> {
        let author = Self::repository()
            .ok()
            .and_then(|repository| repository.get_author().ok());

        let commit_message = match (self.message, self.file) {
            (Some(message), None) => message,
            (None, Some(file_path)) => {
                if !file_path.exists() {
                    bail!("File {file_path:#?} does not exist");
                }

                match fs::read_to_string(file_path) {
                    Err(e) => bail!("Could not read the file ({e})"),
                    Ok(msg) => msg,
                }
            }
            (None, None) => unreachable!(),
            (Some(_), Some(_)) => unreachable!(),
        };

        cocogitto_commit::verify(
            author,
            &commit_message,
            self.ignore_merge_commits,
            self.allowed_commits.as_slice(),
        )?;

        Ok(())
    }
}
