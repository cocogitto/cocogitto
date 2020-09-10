use crate::commit::CommitType::*;
use anyhow::Result;
use colored::*;
use git2::Commit as Git2Commit;
use serde::export::Formatter;
use std::cmp::Ordering;
use std::fmt;

#[derive(Debug, Eq, PartialEq)]
pub struct Commit {
    pub(crate) shorthand: String,
    pub(crate) commit_type: CommitType,
    pub(crate) scope: Option<String>,
    pub(crate) description: String,
    pub(crate) author: String,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum SortCommit {
    ByDate,
    ByType,
    ByScope,
    ByTypeAndScope,
}

impl Commit {
    pub fn from_git_commit(commit: Git2Commit) -> Result<Self> {
        let shorthand = commit
            .as_object()
            .short_id()
            .unwrap()
            .as_str()
            .unwrap()
            .to_string();
        let commit = commit.to_owned();
        let message = commit.message();
        let message = message.unwrap().to_owned();

        print!("Parsing commit : {} - {}", shorthand, message);

        let author = commit.author().name().unwrap_or_else(|| "").to_string();
        let split: Vec<&str> = message.split(": ").to_owned().collect();

        if split.len() <= 1 {
            return Err(anyhow!("Skipping commit : invalid commit format".red()));
        }

        let description = split[1].to_owned().replace('\n', " ");

        let left_part: Vec<&str> = split[0].split("(").collect();

        let commit_type = CommitType::from(left_part[0]);

        if let CommitType::Unknown(type_str) = commit_type {
            return Err(anyhow!(
                "Skipping commit : unknown commit type `{}`",
                type_str
            ));
        };

        let scope = left_part
            .get(1)
            .map(|scope| scope[0..scope.len() - 1].to_owned());

        Ok(Commit {
            shorthand,
            commit_type,
            scope,
            description,
            author,
        })
    }

    pub fn to_markdown(&self) -> String {
        format!(
            "{} - {} - {}\n",
            self.shorthand.yellow(),
            self.description,
            self.author.blue()
        )
    }
}

#[derive(Eq, PartialEq, Debug)]
pub enum CommitType {
    Feature,
    BugFix,
    Chore,
    Revert,
    Performances,
    Documentation,
    Style,
    Refactoring,
    Test,
    Build,
    Ci,
    Unknown(String),
    Custom(String, String),
}

impl CommitType {
    pub(crate) fn get_markdown_title(&self) -> &str {
        match self {
            Feature => "Feature",
            BugFix => "Bug Fixes",
            Chore => "Miscellaneous Chores",
            Revert => "Revert",
            Performances => "Performance Improvements",
            Documentation => "Documentation",
            Style => "Style",
            Refactoring => "Refactoring",
            Test => "Tests",
            Build => "Build System",
            Ci => "Continuous Integration",
            Custom(_, value) => value,
            Unknown(_) => unreachable!(),
        }
    }

    fn get_key_string(&self) -> String {
        match &self {
            Feature => "feat".to_string(),
            BugFix => "fix".to_string(),
            Chore => "chore".to_string(),
            Revert => "revert".to_string(),
            Performances => "perf".to_string(),
            Documentation => "docs".to_string(),
            Style => "style".to_string(),
            Refactoring => "refactor".to_string(),
            Test => "test".to_string(),
            Build => "build".to_string(),
            Ci => "ci".to_string(),
            Custom(key, _) => key.to_owned(),
            Unknown(_) => unreachable!(),
        }
    }
}

impl From<&str> for CommitType {
    fn from(commit_type: &str) -> Self {
        match commit_type {
            "feat" => Feature,
            "fix" => BugFix,
            "chore" => Chore,
            "revert" => Revert,
            "perf" => Performances,
            "docs" => Documentation,
            "style" => Style,
            "refactor" => Refactoring,
            "test" => Test,
            "build" => Build,
            "ci" => Ci,
            other => Unknown(other.to_string()),
        }
    }
}

impl fmt::Display for CommitType {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.get_key_string())
    }
}

impl PartialOrd for Commit {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        self.scope.partial_cmp(&other.scope)
    }
}

impl Ord for Commit {
    fn cmp(&self, other: &Self) -> Ordering {
        self.scope.cmp(&other.scope)
    }
}

#[cfg(test)]
mod test {
    use super::Commit;

    #[test]
    fn should_map_conventional_commit_message_to_struct() {
        // Arrange
        let message = "feat(database): add postgresql driver";

        // Act
        let commit = Commit::from_raw_message(message);

        // Assert
        assert_eq!(commit.commit_type, "feat".to_owned());
        assert_eq!(commit.scope, Some("database".to_owned()));
        assert_eq!(commit.description, "add postgresql driver".to_owned());
    }
}
