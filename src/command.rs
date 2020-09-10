use crate::commit::CommitType;
use crate::CocoGitto;
use anyhow::Result;

pub trait Command {
    fn execute(&self, cocogitto: CocoGitto) -> Result<()>;
}

pub struct CommitCommand {
    pub commit_type: CommitType,
    pub scope: Option<String>,
    pub message: String,
}

impl CommitCommand {
    fn conventional_commit(&self) -> String {
        match &self.scope {
            Some(scope) => format!(
                "{commit_type}({scope}): {message}",
                commit_type = self.commit_type,
                scope = scope,
                message = &self.message,
            ),
            None => format!(
                "{commit_type}: {message}",
                commit_type = &self.commit_type,
                message = &self.message,
            ),
        }
    }
}
impl Command for CommitCommand {
    fn execute(&self, cocogitto: CocoGitto) -> Result<()> {
        cocogitto.repository.commit(self.conventional_commit())
    }
}
