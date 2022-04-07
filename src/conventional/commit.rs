use std::cmp::Ordering;
use std::fmt::{self, Formatter};

use crate::conventional::error::ConventionalCommitError;
use crate::SETTINGS;
use chrono::{NaiveDateTime, Utc};
use colored::*;
use conventional_commit_parser::commit::ConventionalCommit;
use git2::Commit as Git2Commit;
use log::info;
use serde::{Deserialize, Serialize};

#[derive(Debug, Eq, PartialEq)]
pub struct Commit {
    pub(crate) oid: String,
    pub(crate) message: ConventionalCommit,
    pub(crate) author: String,
    pub(crate) date: NaiveDateTime,
}

#[derive(Debug, Deserialize, Serialize, Clone, Eq, PartialEq)]
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
    pub(crate) fn from_git_commit(commit: &Git2Commit) -> Result<Self, ConventionalCommitError> {
        let oid = commit.id().to_string();

        let commit = commit.to_owned();
        let date = NaiveDateTime::from_timestamp(commit.time().seconds(), 0);
        let message = commit.message();
        let git2_message = message.unwrap().to_owned();
        let author = commit.author().name().unwrap_or("").to_string();

        let message = git2_message.trim_end().trim_start();
        let conventional_commit = conventional_commit_parser::parse(message);

        match conventional_commit {
            Ok(message) => {
                let commit = Commit {
                    oid,
                    message,
                    author,
                    date,
                };

                match &SETTINGS.commit_types().get(&commit.message.commit_type) {
                    Some(_) => Ok(commit),
                    None => Err(ConventionalCommitError::CommitTypeNotAllowed {
                        oid: commit.oid.to_string(),
                        summary: format_summary(&commit.message),
                        commit_type: commit.message.commit_type.to_string(),
                        author: commit.author,
                    }),
                }
            }
            Err(cause) => {
                let message = git2_message.trim_end();
                let summary = Commit::short_summary_from_str(message);
                Err(ConventionalCommitError::CommitFormat {
                    oid,
                    summary,
                    author,
                    cause,
                })
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

pub fn verify(
    author: Option<String>,
    message: &str,
    ignore_merge_commit: bool,
) -> Result<(), ConventionalCommitError> {
    // Strip away comments from git message before parsing
    let msg: String = message
        .lines()
        .filter(|line| !line.trim_start().starts_with('#'))
        .collect::<Vec<&str>>()
        .join("\n");

    let msg = msg.trim();

    if msg.starts_with("Merge ") && ignore_merge_commit {
        info!("{}", "Merge commit was ignored".yellow());
        return Ok(());
    }

    let commit = conventional_commit_parser::parse(msg);

    match commit {
        Ok(commit) => match &SETTINGS.commit_types().get(&commit.commit_type) {
            Some(_) => {
                info!(
                    "{}",
                    Commit {
                        oid: "not committed".to_string(),
                        message: commit,
                        date: Utc::now().naive_utc(),
                        author: author.unwrap_or_else(|| "Unknown".to_string()),
                    }
                );
                Ok(())
            }
            None => Err(ConventionalCommitError::CommitTypeNotAllowed {
                oid: "not committed".to_string(),
                summary: format_summary(&commit),
                commit_type: commit.commit_type.to_string(),
                author: author.unwrap_or_else(|| "Unknown".to_string()),
            }),
        },
        Err(err) => Err(ConventionalCommitError::ParseError(err)),
    }
}

pub(crate) fn format_summary(commit: &ConventionalCommit) -> String {
    match &commit.scope {
        None => format!("{}: {}", commit.commit_type, commit.summary,),
        Some(scope) => {
            format!("{}({}): {}", commit.commit_type, scope, commit.summary,)
        }
    }
}

#[cfg(test)]
mod test {
    use crate::conventional::commit::{format_summary, verify, Commit};

    use chrono::NaiveDateTime;
    use cmd_lib::run_fun;

    use crate::Repository;
    use anyhow::Result;
    use conventional_commit_parser::commit::{CommitType, ConventionalCommit, Footer, Separator};
    use git2::Oid;
    use indoc::indoc;
    use sealed_test::prelude::*;
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
    fn should_map_conventional_commit_message_with_multiple_scope_to_struct() {
        // Arrange
        let message = indoc!(
            "feat(database): add postgresql driver

            The body

            footer: 123
            footer2 #456"
        );

        // Act
        let commit = conventional_commit_parser::parse(message);

        // Assert
        let commit = commit.unwrap();
        assert_that!(commit.commit_type).is_equal_to(CommitType::Feature);
        assert_that!(commit.scope).is_equal_to(Some("database".to_owned()));
        assert_that!(commit.summary).is_equal_to("add postgresql driver".to_owned());
        assert_that!(commit.is_breaking_change).is_false();
        assert_that!(commit.body)
            .is_some()
            .is_equal_to("The body".to_string());
        assert_that!(commit.footers).is_equal_to(vec![
            Footer {
                token: "footer".to_string(),
                content: "123".to_string(),
                ..Default::default()
            },
            Footer {
                token: "footer2".to_string(),
                content: "456".to_string(),
                token_separator: Separator::Hash,
            },
        ]);

        assert_that!(commit.to_string()).is_equal_to(&message.to_string())
    }

    #[test]
    fn should_verify_message_ok() {
        // Arrange
        let message = "feat(database): add postgresql driver";

        // Act
        let result = verify(Some("toml".into()), message, false);

        // Assert
        assert_that!(result).is_ok();
    }

    #[test]
    fn should_verify_message_with_comments_ok() {
        // Arrange
        let message = indoc!(
            "# testing a commit with a comment
            feat(database): add postgresql driver

            # Enter message body here
            The body"
        );

        // Act
        let result = verify(Some("toml".into()), message, false);

        // Assert
        assert_that!(result).is_ok();
    }

    #[test]
    fn should_verify_message_err() {
        // Arrange
        let message = "feat add postgresql driver";

        // Act
        let result = verify(Some("toml".into()), message, false);

        // Assert
        assert_that!(result).is_err();
    }

    #[test]
    fn verify_with_unknown_commit_type_fails() {
        // Arrange
        let message = "post: add postgresql driver";

        // Act
        let result = verify(Some("toml".into()), message, false);

        // Assert
        assert_that!(result).is_err();
    }

    #[test]
    fn verify_with_comment_and_trailing_whitespace_succeeds() -> Result<()> {
        let message = indoc!(
            "fix: test

            # Please enter the commit message for your changes. Lines starting
            # with '#' will be ignored, and an empty message aborts the commit.
            #
            # On branch master
            # Changes to be committed:
            #       modified:   file
            #
            "
        );

        let outcome = verify(None, message, false);

        assert_that!(outcome).is_ok();
        Ok(())
    }

    #[test]
    fn should_format_summary() {
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
        let summary = format_summary(&commit.message);

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
        let summary = format_summary(&commit.message);

        // Assert
        assert_that!(summary).is_equal_to("fix: this is the message".to_string());
    }

    #[sealed_test]
    fn should_map_conventional_commit() {
        // Arrange
        let oid = run_fun!(
            git init;
            git commit --allow-empty -q -m "feat: a commit";
            git log --format=%H -n 1;
        )
        .unwrap();

        let oid = Oid::from_str(&oid).unwrap();
        let repo = Repository::open(".").unwrap();
        let commit = repo.0.find_commit(oid).expect("Unable to find commit");

        // Act
        let commit = Commit::from_git_commit(&commit);

        // Assert
        assert_that!(commit).is_ok();
    }

    #[sealed_test]
    fn map_conventional_commit_should_fail_with_invalid_type() {
        // Arrange
        let oid_str = run_fun!(
            git init;
            git commit --allow-empty -q -m "toto: a commit";
            git log --format=%H -n 1;
        )
        .unwrap();

        let oid = Oid::from_str(&oid_str).unwrap();
        let repo = Repository::open(".").unwrap();
        let commit = repo.0.find_commit(oid).expect("Unable to find commit");

        // Act
        let commit = Commit::from_git_commit(&commit);

        // Assert
        assert_that!(commit).is_err();
    }

    #[sealed_test]
    fn map_conventional_commit_should_fail() {
        // Arrange
        let oid_str = run_fun!(
            git init;
            git commit --allow-empty -q -m "a commit";
            git log --format=%H -n 1;
        )
        .unwrap();

        let oid = Oid::from_str(&oid_str).unwrap();
        let repo = Repository::open(".").unwrap();
        let commit = repo.0.find_commit(oid).expect("Unable to find commit");

        // Act
        let commit = Commit::from_git_commit(&commit);

        // Assert
        assert_that!(commit).is_err();
    }
}
