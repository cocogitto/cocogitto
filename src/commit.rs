use crate::commit::CommitType::*;
use crate::error::CocoGittoError::CommitFormatError;
use anyhow::Result;
use chrono::{NaiveDateTime, Utc};
use colored::*;
use git2::Commit as Git2Commit;
use serde::export::Formatter;
use std::cmp::Ordering;
use std::fmt;

#[derive(Debug, Eq, PartialEq)]
pub struct Commit {
    pub(crate) shorthand: String,
    pub(crate) message: CommitMessage,
    pub(crate) author: String,
    pub(crate) date: NaiveDateTime,
}

#[derive(Debug, Eq, PartialEq)]
pub struct CommitMessage {
    pub(crate) commit_type: CommitType,
    pub(crate) scope: Option<String>,
    pub(crate) body: Option<String>,
    pub(crate) footer: Option<String>,
    pub(crate) description: String,
    pub(crate) is_breaking_change: bool,
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
    pub fn from_git_commit(commit: &Git2Commit) -> Result<Self> {
        let shorthand = commit
            .as_object()
            .short_id()
            .unwrap()
            .as_str()
            .unwrap()
            .to_string();

        let commit = commit.to_owned();
        let date = NaiveDateTime::from_timestamp(commit.time().seconds(), 0);
        let message = commit.message();
        let git2_message = message.unwrap().to_owned();
        let author = commit.author().name().unwrap_or_else(|| "").to_string();
        let message = Commit::parse_commit_message(&git2_message);

        let result = match message {
            Ok(message) => Ok(Commit {
                shorthand,
                message,
                author,
                date,
            }),
            Err(err) => {
                let message = git2_message.replace("\n", "");
                let commit_message = if message.len() > 80 {
                    format!("{}{}", &message[0..80], "...").red()
                } else {
                    git2_message.red()
                }
                .to_string();
                let cause = format!("{} {}", "cause:".red(), err);
                let level = "ERROR".red().bold().to_string();
                Err(anyhow!(CommitFormatError {
                    level,
                    shorthand,
                    commit_message,
                    cause
                }))
            }
        };

        result
    }

    // Todo extract to ParseError
    pub(crate) fn parse_commit_message(message: &str) -> Result<CommitMessage> {
        let type_separator = message.find(": ");
        if type_separator.is_none() {
            return Err(anyhow!(
                "invalid commit format : missing `{}` separator",
                ": ".yellow()
            ));
        }

        let idx = type_separator.unwrap();

        let mut type_and_scope = &message[0..idx];
        let mut is_breaking_change = type_and_scope.chars().last() == Some('!');

        if is_breaking_change {
            type_and_scope = &type_and_scope[0..type_and_scope.len() - 1];
        }

        let commit_type;

        let scope: Option<String> = if let Some(left_par_idx) = type_and_scope.find("(") {
            commit_type = CommitType::from(&type_and_scope[0..left_par_idx]);

            Some(
                type_and_scope
                    .find(")")
                    .ok_or(anyhow!("missing closing parenthesis"))
                    .map(|right_par_idx| {
                        type_and_scope[left_par_idx + 1..right_par_idx].to_string()
                    })?,
            )
        } else {
            commit_type = CommitType::from(type_and_scope);
            None
        };

        let contents = &message[idx + 2..message.len()];
        let contents: Vec<&str> = contents.split('\n').collect();

        let description = contents.get(0).map(|desc| desc.to_string());

        if description.is_none() {
            return Err(anyhow!("missing commit description"));
        }

        let description = description.unwrap();

        let body = contents.get(1).map(|desc| desc.to_string());

        let footer = contents.get(2).map(|desc| desc.to_string());

        if let Some(footer) = &footer {
            is_breaking_change = is_breaking_change
                || footer.contains("BREAKING CHANGE")
                || footer.contains("BREAKING-CHANGE")
        }

        if let CommitType::Unknown(type_str) = commit_type {
            return Err(anyhow!("unknown commit type `{}`", type_str.red()));
        };

        Ok(CommitMessage {
            description,
            commit_type,
            scope,
            body,
            footer,
            is_breaking_change,
        })
    }

    pub fn to_markdown(&self) -> String {
        format!(
            "{} - {} - {}\n",
            self.shorthand.yellow(),
            self.message.description,
            self.author.blue()
        )
    }
    pub fn get_log(&self) -> String {
        let message_display = self.message.description.replace("\n", " ");
        let message_display = if message_display.len() > 80 {
            format!("{}{}", &message_display[0..80], "...").yellow()
        } else {
            message_display.yellow()
        };

        let author_format = "Author:".green().bold();
        let type_format = "Type:".green().bold();
        let scope_format = "Scope:".green().bold();
        let breaking_change = if self.message.is_breaking_change {
            format!("{} - ", "BREAKING CHANGE".red().bold())
        } else {
            "".to_string()
        };
        let now = Utc::now().naive_utc();
        let elapsed = now - self.date;
        let elapsed = if elapsed.num_weeks() > 0 {
            let week = if elapsed.num_weeks() == 1 {
                "week"
            } else {
                "weeks"
            };
            format!("{} {} ago", elapsed.num_weeks(), week)
        } else if elapsed.num_days() > 0 {
            let day = if elapsed.num_days() == 1 {
                "day"
            } else {
                "days"
            };
            format!("{} {} ago", elapsed.num_days(), day)
        } else if elapsed.num_hours() > 0 {
            let hour = if elapsed.num_hours() == 1 {
                "hour"
            } else {
                "hours"
            };
            format!("{} {} ago", elapsed.num_hours(), hour)
        } else if elapsed.num_minutes() > 0 {
            let minute = if elapsed.num_minutes() == 1 {
                "minute"
            } else {
                "minutes"
            };
            format!("{} {} ago", elapsed.num_minutes(), minute)
        } else if elapsed.num_seconds() > 0 {
            let second = if elapsed.num_seconds() == 1 {
                "second"
            } else {
                "seconds"
            };
            format!("{} {} ago", elapsed.num_seconds(), second)
        } else {
            "now".to_string()
        };

        format!(
            "{}{} ({}) - {}\n\t{} {}\n\t{} {}\n\t{} {}\n",
            breaking_change,
            message_display,
            &self.shorthand.bold(),
            elapsed,
            author_format,
            &self.author,
            type_format,
            &self.message.commit_type,
            scope_format,
            &self.message.scope.as_ref().unwrap_or(&"none".to_string()),
        )
    }
}

impl fmt::Display for Commit {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.get_log())
    }
}

#[derive(Eq, PartialEq, Debug)]
pub(crate) enum CommitType {
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

    pub(crate) fn get_key_str(&self) -> String {
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
        write!(f, "{}", self.get_key_str())
    }
}

impl PartialOrd for Commit {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        self.message.scope.partial_cmp(&other.message.scope)
    }
}

impl Ord for Commit {
    fn cmp(&self, other: &Self) -> Ordering {
        self.message.scope.cmp(&other.message.scope)
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
