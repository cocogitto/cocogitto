use crate::conventional::commit::CommitType::*;
use crate::error::ErrorKind::CommitFormat;
use crate::AUTHORS;
use crate::COMMITS_METADATA;
use crate::REMOTE_URL;
use anyhow::Result;
use chrono::{NaiveDateTime, Utc};
use colored::*;
use git2::Commit as Git2Commit;
use std::cmp::Ordering;
use std::fmt;
use std::fmt::Formatter;

#[derive(Debug, Eq, PartialEq)]
pub struct Commit {
    pub(crate) oid: String,
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

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct CommitConfig {
    pub changelog_title: String,
}

impl CommitConfig {
    pub(crate) fn new(changelog_title: &str) -> Self {
        CommitConfig {
            changelog_title: changelog_title.to_string(),
        }
    }
}

impl Commit {
    pub(crate) fn from_git_commit(commit: &Git2Commit) -> Result<Self> {
        let oid = commit.id().to_string();

        let commit = commit.to_owned();
        let date = NaiveDateTime::from_timestamp(commit.time().seconds(), 0);
        let message = commit.message();
        let git2_message = message.unwrap().to_owned();
        let author = commit.author().name().unwrap_or("").to_string();
        let message = Commit::parse_commit_message(&git2_message);

        match message {
            Ok(message) => Ok(Commit {
                oid,
                message,
                author,
                date,
            }),
            Err(err) => {
                let additional_info = if commit.parent_count() == 0 {
                    format!(
                        "{} Init commit or commit with no parent cannot be edited",
                        "warning:".yellow()
                    )
                } else {
                    "".to_string()
                };

                let message = git2_message.trim_end();
                let commit_message = if message.len() > 80 {
                    format!("{}{}", &message[0..80], "...").red()
                } else {
                    message.red()
                }
                .to_string();

                let cause = format!("{} {}", "cause:".magenta(), err);
                let level = "ERROR".red().bold().to_string();
                Err(anyhow!(CommitFormat {
                    level,
                    shorthand: oid[0..6].into(),
                    commit_message,
                    additional_info,
                    cause
                }))
            }
        }
    }

    pub(crate) fn shorthand(&self) -> &str {
        if self.oid != "not committed" {
            &self.oid[0..6]
        } else {
            &self.oid
        }
    }

    // Todo extract to ParseError
    pub(crate) fn parse_commit_message(message: &str) -> Result<CommitMessage> {
        let type_separator = message.find(": ");
        ensure!(
            type_separator.is_some(),
            "invalid commit format: missing `{}` separator",
            ": ".yellow()
        );

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

        ensure!(description.is_some(), "missing commit description");

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

        ensure!(
            allowed_commit.is_some(),
            "unknown commit type `{}`",
            commit_type_str.red()
        );

        Ok(CommitMessage {
            commit_type,
            scope,
            body,
            footer,
            description,
            is_breaking_change,
        })
    }

    pub fn to_markdown(&self, colored: bool) -> String {
        if colored {
            format!(
                "{} - {} - {}\n",
                self.shorthand().yellow(),
                self.message.description,
                self.author.blue()
            )
        } else {
            let username = AUTHORS
                .iter()
                .find(|author| author.signature == self.author);
            let github_author = username.map(|username| {
                format!(
                    "[{}](https://github.com/{})",
                    &username.username, &username.username
                )
            });
            let oid = REMOTE_URL.as_ref().map(|remote_url| {
                format!("[{}]({}/commit/{})", &self.oid[0..6], remote_url, &self.oid)
            });
            format!(
                "{} - {} - {}\n\n",
                oid.unwrap_or_else(|| self.oid[0..6].into()),
                self.message.description,
                github_author.unwrap_or_else(|| self.author.to_string())
            )
        }
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
            self.shorthand().bold(),
            elapsed,
            author_format,
            self.author,
            type_format,
            self.message.commit_type,
            scope_format,
            self.message.scope.as_deref().unwrap_or("none"),
        )
    }
}

impl fmt::Display for Commit {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.get_log())
    }
}

impl AsRef<str> for CommitType {
    fn as_ref(&self) -> &str {
        match self {
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

impl ToString for CommitMessage {
    fn to_string(&self) -> String {
        let mut message = String::new();
        message.push_str(self.commit_type.as_ref());

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

impl fmt::Display for CommitType {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.as_ref())
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

pub fn verify(author: Option<String>, message: &str) -> Result<()> {
    let commit = Commit::parse_commit_message(message);

    match commit {
        Ok(message) => {
            println!(
                "{}",
                Commit {
                    oid: "not committed".to_string(),
                    message,
                    date: Utc::now().naive_utc(),
                    author: author.unwrap_or_else(|| "Unknown".to_string()),
                }
            );
            Ok(())
        }
        Err(err) => Err(err),
    }
}

#[cfg(test)]
mod test {
    use super::Commit;
    use crate::conventional::commit::{verify, CommitType};

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
        assert!(!commit.is_breaking_change);
        assert!(commit.body.is_none());
        assert!(commit.footer.is_none());
    }

    #[test]
    fn should_verify_message_ok() {
        // Arrange
        let message = "feat(database): add postgresql driver";

        // Act
        let result = verify(Some("toml".into()), message);

        // Assert
        assert!(result.is_ok());
    }

    #[test]
    fn should_verify_message_err() {
        // Arrange
        let message = "feat add postgresql driver";

        // Act
        let result = verify(Some("toml".into()), message);

        // Assert
        assert!(result.is_err());
    }
}
