use crate::conventional::commit::Commit;
use crate::git::oid::OidOf;
use crate::git::rev::CommitIter;
use crate::{settings, SETTINGS};
use chrono::{NaiveDateTime, Utc};
use colored::Colorize;
use conventional_commit_parser::commit::{Footer, Separator};
use serde::Serialize;

use crate::conventional::changelog::error::ChangelogError;
use log::warn;

#[derive(Debug, Serialize)]
pub struct Release {
    pub version: OidOf,
    pub from: OidOf,
    pub date: NaiveDateTime,
    pub commits: Vec<ChangelogCommit>,
    pub previous: Option<Box<Release>>,
}

impl TryFrom<CommitIter<'_>> for Release {
    type Error = ChangelogError;

    fn try_from(commits: CommitIter<'_>) -> Result<Self, Self::Error> {
        let mut releases = vec![];
        let mut commit_iter = commits.into_iter().rev().peekable();

        while let Some((_oid, _commit)) = commit_iter.peek() {
            let mut release_commits = vec![];

            for (oid, commit) in commit_iter.by_ref() {
                if matches!(oid, OidOf::Tag(_)) {
                    release_commits.push((oid, commit));
                    break;
                }
                release_commits.push((oid, commit));
            }

            release_commits.reverse();
            releases.push(release_commits);
        }

        let mut current = None;

        for release in releases {
            let next = Release {
                version: release.first().unwrap().0.clone(),
                from: current
                    .as_ref()
                    .map(|current: &Release| current.version.clone())
                    .unwrap_or(release.last().unwrap().0.clone()),
                date: chrono::DateTime::from_timestamp(
                    release.first().unwrap().1.time().seconds(),
                    0,
                )
                .map(|dt| dt.naive_utc())
                .unwrap_or_else(|| Utc::now().naive_utc()),
                commits: release
                    .iter()
                    .filter(|(_commit, commit)| commit.message().is_some())
                    .filter(|(_commit, commit)| {
                        if SETTINGS.ignore_merge_commits {
                            !commit.message().unwrap().starts_with("Merge")
                        } else {
                            true
                        }
                    })
                    .filter(|(_commit, commit)| {
                        if SETTINGS.ignore_fixup_commits {
                            !commit.message().unwrap().starts_with("fixup!")
                                && !commit.message().unwrap().starts_with("squash!")
                                && !commit.message().unwrap().starts_with("amend!")
                        } else {
                            true
                        }
                    })
                    .filter_map(|(_, commit)| match Commit::from_git_commit(commit) {
                        Ok(commit) => {
                            if !commit.should_omit() {
                                Some(ChangelogCommit::from(commit))
                            } else {
                                None
                            }
                        }
                        Err(err) => {
                            let err = err.to_string().red();
                            warn!("{}", err);
                            None
                        }
                    })
                    .collect(),
                previous: current.map(Box::new),
            };

            current = Some(next);
        }

        current.ok_or(ChangelogError::EmptyRelease)
    }
}

#[derive(Debug)]
pub struct ChangelogCommit {
    pub author_username: Option<String>,
    pub committer_username: Option<String>,
    pub co_authors: Vec<String>,
    pub github_closes_numbers: Vec<String>,
    pub commit: Commit,
}

impl From<Commit> for ChangelogCommit {
    fn from(commit: Commit) -> Self {
        let author_username = settings::commit_username(&commit.author);

        ChangelogCommit {
            author_username,
            commit,
            committer_username: None,
            co_authors: vec![],
            github_closes_numbers: vec![],
        }
    }
}

#[derive(Serialize)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum ChangelogFooter<'a> {
    GithubCoAuthoredBy { user: &'a str },
    GithubCloses { gh_reference: &'a str },
    Footer { token: &'a str, content: &'a str },
}

impl<'a> From<&'a Footer> for ChangelogFooter<'a> {
    fn from(footer: &'a Footer) -> Self {
        match footer.token.as_str().to_lowercase().as_str() {
            "co-authored-by" if footer.token_separator == Separator::Colon => {
                let user = footer
                    .content
                    .split('<')
                    .next()
                    .map(str::trim)
                    .unwrap_or(footer.content.as_str());
                Self::GithubCoAuthoredBy { user }
            }
            "close" | "closes" | "closed" | "fix" | "fixes" | "fixed" | "resolve" | "resolves"
            | "resolved"
                if footer.token_separator == Separator::Hash =>
            {
                Self::GithubCloses {
                    gh_reference: footer.content.as_str(),
                }
            }
            _ => Self::Footer {
                token: footer.content.as_str(),
                content: footer.content.as_str(),
            },
        }
    }
}
