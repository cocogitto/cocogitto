use crate::error::CocogittoError::CommitFormat;
use crate::SETTINGS;
use anyhow::Result;
use chrono::{NaiveDateTime, Utc};
use colored::*;
use conventional_commit_parser::commit::ConventionalCommit;
use conventional_commit_parser::error::ParseError;
use git2::Commit as Git2Commit;
use std::cmp::Ordering;
use std::fmt;
use std::fmt::Formatter;

#[derive(Debug, Eq, PartialEq)]
pub struct Commit {
    pub(crate) oid: String,
    pub(crate) message: ConventionalCommit,
    pub(crate) author: String,
    pub(crate) date: NaiveDateTime,
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

        // FIXME:Why suddenly commit message start and finish with '\n'
        let message = git2_message.trim_end().trim_start();
        let conventional_commit = conventional_commit_parser::parse(message);

        match conventional_commit {
            Ok(message) => Ok(Commit {
                oid,
                message,
                author,
                date,
            }),
            Err(err) => {
                let message = git2_message.trim_end();
                let summary = Commit::short_summary_from_str(message);

                Err(anyhow!(CommitFormat {
                    oid,
                    summary,
                    author,
                    cause: err.to_string()
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

    pub fn to_markdown(&self, colored: bool) -> String {
        if colored {
            format!(
                "{} - {} - {}\n",
                self.shorthand().yellow(),
                self.message.summary,
                self.author.blue()
            )
        } else {
            let username = SETTINGS
                .authors
                .iter()
                .find(|author| author.signature == self.author);
            let github_author = username.map(|user| {
                format!(
                    "[{username}](https://github.com/{username})",
                    username = &user.username
                )
            });
            let oid = SETTINGS.github.as_ref().map(|remote_url| {
                format!("[{}]({}/commit/{})", &self.oid[0..6], remote_url, &self.oid)
            });
            format!(
                "{} - {} - {}\n\n",
                oid.unwrap_or_else(|| self.oid[0..6].into()),
                self.message.summary,
                github_author.unwrap_or_else(|| self.author.to_string())
            )
        }
    }

    pub fn get_log(&self) -> String {
        let summary = &self.message.summary;
        let message_display = Commit::short_summary_from_str(summary).yellow();
        let author_format = "Author:".green().bold();
        let type_format = "Type:".green().bold();
        let scope_format = "Scope:".green().bold();
        let breaking_change = self.format_breaking_change();
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

    fn format_breaking_change(&self) -> String {
        if self.message.is_breaking_change {
            format!("{} - ", "BREAKING CHANGE".red().bold())
        } else {
            "".to_string()
        }
    }

    pub(crate) fn format_summary(&self) -> String {
        match &self.message.scope {
            None => format!("{}: {}", self.message.commit_type, self.message.summary,),
            Some(scope) => {
                format!(
                    "{}({}): {}",
                    self.message.commit_type, scope, self.message.summary,
                )
            }
        }
    }

    fn short_summary_from_str(summary: &str) -> String {
        if summary.len() > 80 {
            // display a maximum of 80 char (77 char + ...)
            let message = summary.chars().take(77).collect::<String>();
            format!("{}{}", message, "...")
        } else {
            summary.to_string()
        }
    }
}

impl fmt::Display for Commit {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.get_log())
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

pub fn verify(author: Option<String>, message: &str) -> Result<(), ParseError> {
    let commit = conventional_commit_parser::parse(message);

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
    use crate::conventional::commit::{verify, Commit};
    use chrono::NaiveDateTime;
    use conventional_commit_parser::commit::{CommitType, ConventionalCommit};
    use speculoos::prelude::*;

    #[test]
    fn should_map_conventional_commit_message_to_struct() {
        // Arrange
        let message = "feat(database): add postgresql driver";

        // Act
        let commit = conventional_commit_parser::parse(message);

        // Assert
        let commit = commit.unwrap();
        assert_that!(commit.commit_type).is_equal_to(CommitType::Feature);
        assert_that!(commit.scope).is_equal_to(Some("database".to_owned()));
        assert_that!(commit.summary).is_equal_to("add postgresql driver".to_owned());
        assert_that!(commit.is_breaking_change).is_false();
        assert_that!(commit.body).is_none();
        assert_that!(commit.footers).is_empty();
    }

    #[test]
    fn should_verify_message_ok() {
        // Arrange
        let message = "feat(database): add postgresql driver";

        // Act
        let result = verify(Some("toml".into()), message);

        // Assert
        assert_that!(result).is_ok();
    }

    #[test]
    fn should_verify_message_err() {
        // Arrange
        let message = "feat add postgresql driver";

        // Act
        let result = verify(Some("toml".into()), message);

        // Assert
        assert_that!(result).is_err();
    }

    #[test]
    fn format_summary() {
        // Arrange
        let commit = Commit {
            oid: "1234567".to_string(),
            message: ConventionalCommit {
                commit_type: CommitType::BugFix,
                scope: Some("scope".to_string()),
                summary: "this is the message".to_string(),
                body: None,
                footers: vec![],
                is_breaking_change: false,
            },

            author: "".to_string(),
            date: NaiveDateTime::from_timestamp(0, 0),
        };

        // Act
        let summary = commit.format_summary();

        // Assert
        assert_that!(summary).is_equal_to("fix(scope): this is the message".to_string());
    }

    #[test]
    fn format_summary_without_scope() {
        // Arrange
        let commit = Commit {
            oid: "1234567".to_string(),
            message: ConventionalCommit {
                commit_type: CommitType::BugFix,
                scope: None,
                summary: "this is the message".to_string(),
                body: None,
                footers: vec![],
                is_breaking_change: false,
            },

            author: "".to_string(),
            date: NaiveDateTime::from_timestamp(0, 0),
        };

        // Act
        let summary = commit.format_summary();

        // Assert
        assert_that!(summary).is_equal_to("fix: this is the message".to_string());
    }
}
