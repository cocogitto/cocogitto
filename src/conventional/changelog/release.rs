use chrono::NaiveDateTime;
use conventional_commit_parser::commit::Footer;
use serde::Serialize;

use crate::conventional::commit::Commit;

use crate::{settings, OidOf};

#[derive(Serialize)]
pub struct Release<'a> {
    pub version: OidOf,
    pub from: OidOf,
    pub date: NaiveDateTime,
    pub commits: Vec<ChangelogCommit<'a>>,
}

pub struct ChangelogCommit<'a> {
    pub author_username: Option<&'a str>,
    pub commit: Commit,
}

impl From<Commit> for ChangelogCommit<'_> {
    fn from(commit: Commit) -> Self {
        let author_username = settings::commit_username(&commit.author);

        ChangelogCommit {
            author_username,
            commit,
        }
    }
}

#[derive(Serialize)]
pub struct ChangelogFooter<'a> {
    token: &'a str,
    content: &'a str,
}

impl<'a> From<&'a Footer> for ChangelogFooter<'a> {
    fn from(footer: &'a Footer) -> Self {
        Self {
            token: footer.token.as_str(),
            content: footer.content.as_str(),
        }
    }
}

#[cfg(test)]
mod test {
    use chrono::Utc;
    use conventional_commit_parser::commit::{CommitType, ConventionalCommit, Footer};

    use speculoos::prelude::*;

    use crate::conventional::changelog::release::{ChangelogCommit, Release};
    use crate::conventional::changelog::renderer::Renderer;
    use crate::conventional::commit::Commit;
    use crate::git::tag::Tag;
    use crate::OidOf;
    use git2::Oid;
    use indoc::indoc;

    #[test]
    fn should_render_default_template() {
        let paul_delafosse = "Paul Delafosse";
        let a_commit_hash = "17f7e23081db15e9318aeb37529b1d473cf41cbe";
        let version = Tag::new(
            "1.0.0",
            Oid::from_str("9bb5facac5724bc81385fdd740fedbb49056da00").unwrap(),
        )
        .unwrap();
        let from = Tag::new(
            "0.1.0",
            Oid::from_str("fae3a288a1bc69b14f85a1d5fe57cee1964acd60").unwrap(),
        )
        .unwrap();
        let version = Release {
            version: OidOf::Tag(version),
            from: OidOf::Tag(from),
            date: Utc::now().naive_utc(),
            commits: vec![
                ChangelogCommit {
                    author_username: Some("oknozor"),
                    commit: Commit {
                        oid: a_commit_hash.to_string(),
                        message: ConventionalCommit {
                            commit_type: CommitType::BugFix,
                            scope: Some("parser".to_string()),
                            summary: "fix parser implementation".to_string(),
                            body: Some("the body".to_string()),
                            footers: vec![Footer {
                                token: "token".to_string(),
                                content: "content".to_string(),
                            }],
                            is_breaking_change: false,
                        },
                        author: paul_delafosse.to_string(),
                        date: Utc::now().naive_utc(),
                    },
                },
                ChangelogCommit {
                    author_username: None,
                    commit: Commit {
                        oid: a_commit_hash.to_string(),
                        message: ConventionalCommit {
                            commit_type: CommitType::Feature,
                            scope: None,
                            summary: "awesome feature".to_string(),
                            body: Some("the body".to_string()),
                            footers: vec![Footer {
                                token: "token".to_string(),
                                content: "content".to_string(),
                            }],
                            is_breaking_change: false,
                        },
                        author: paul_delafosse.to_string(),
                        date: Utc::now().naive_utc(),
                    },
                },
                ChangelogCommit {
                    author_username: Some("oknozor"),
                    commit: Commit {
                        oid: a_commit_hash.to_string(),
                        message: ConventionalCommit {
                            commit_type: CommitType::Feature,
                            scope: Some("parser".to_string()),
                            summary: "implement the changelog generator".to_string(),
                            body: Some("the body".to_string()),
                            footers: vec![Footer {
                                token: "token".to_string(),
                                content: "content".to_string(),
                            }],
                            is_breaking_change: false,
                        },
                        author: "James Delleck".to_string(),
                        date: Utc::now().naive_utc(),
                    },
                },
            ],
        };

        let renderer = Renderer::default();
        let changelog = renderer.render(&version);

        assert_that!(changelog).is_ok().is_equal_to(
            indoc! {
                "## 1.0.0 - 2021-10-31
                #### Bug Fixes
                - **(parser)** fix parser implementation - (17f7e23) - *oknozor*
                #### Features
                - **(parser)** implement the changelog generator - (17f7e23) - *oknozor*
                - awesome feature - (17f7e23) - Paul Delafosse"
            }
            .to_string(),
        );
    }
}
