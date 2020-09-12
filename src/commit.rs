use crate::commit::CommitType::*;
use crate::error::ErrorKind::CommitFormat;
use crate::COMMITS_METADATA;
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

#[derive(Hash, Eq, PartialEq, Debug, Clone)]
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
    Custom(String),
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

#[derive(Debug, Deserialize, Clone)]
pub struct CommitConfig {
    pub changelog_title: String,
    pub help_message: String,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum SortCommit {
    ByDate,
    ByType,
    ByScope,
    ByTypeAndScope,
}

impl CommitConfig {
    pub(crate) fn new(changelog_title: &str, help_message: &str) -> Self {
        CommitConfig {
            changelog_title: changelog_title.to_string(),
            help_message: help_message.to_string(),
        }
    }
}

impl Commit {
    pub(crate) fn from_git_commit(commit: &Git2Commit) -> Result<Self> {
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
        let author = commit.author().name().unwrap_or("").to_string();
        let message = Commit::parse_commit_message(&git2_message);

        match message {
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
                Err(anyhow!(CommitFormat {
                    level,
                    shorthand,
                    commit_message,
                    cause
                }))
            }
        }
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
        let mut is_breaking_change = type_and_scope.ends_with('!');

        if is_breaking_change {
            type_and_scope = &type_and_scope[0..type_and_scope.len() - 1];
        }

        let commit_type_str;

        let scope: Option<String> = if let Some(left_par_idx) = type_and_scope.find('(') {
            commit_type_str = &type_and_scope[0..left_par_idx];

            Some(
                type_and_scope
                    .find(')')
                    .ok_or_else(|| anyhow!("missing closing parenthesis"))
                    .map(|right_par_idx| {
                        type_and_scope[left_par_idx + 1..right_par_idx].to_string()
                    })?,
            )
        } else {
            commit_type_str = type_and_scope;
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

        let commit_type = CommitType::from(commit_type_str);
        let allowed_commit = COMMITS_METADATA.get(&commit_type);

        if allowed_commit.is_none() {
            return Err(anyhow!("unknown commit type `{}`", commit_type_str.red()));
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

impl CommitType {
    pub fn get_key_str(&self) -> &str {
        match &self {
            Feature => "feat",
            BugFix => "fix",
            Chore => "chore",
            Revert => "revert",
            Performances => "perf",
            Documentation => "docs",
            Style => "style",
            Refactoring => "refactor",
            Test => "test",
            Build => "build",
            Ci => "ci",
            Custom(key) => key,
        }
    }
}

impl ToString for CommitMessage {
    fn to_string(&self) -> String {
        let mut message = String::new();
        message.push_str(&self.commit_type.get_key_str());

        if let Some(scope) = &self.scope {
            message.push_str(&format!("({})", scope));
        }

        if self.is_breaking_change {
            message.push('!');
        }

        message.push_str(&format!(": {}", &self.description));

        if let Some(body) = &self.body {
            message.push_str(&format!("\n\n{}", body));
        }

        if let Some(footer) = &self.footer {
            message.push_str(&format!("\n\n{}", footer));
        }

        message
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
            other => Custom(other.to_string()),
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
    use crate::commit::CommitType;

    #[test]
    fn should_map_conventional_commit_message_to_struct() {
        // Arrange
        let message = "feat(database): add postgresql driver";

        // Act
        let commit = Commit::parse_commit_message(message);

        // Assert
        let commit = commit.unwrap();
        assert_eq!(commit.commit_type, CommitType::Feature);
        assert_eq!(commit.scope, Some("database".to_owned()));
        assert_eq!(commit.description, "add postgresql driver".to_owned());
    }
}
