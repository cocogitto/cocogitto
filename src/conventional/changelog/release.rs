use chrono::NaiveDateTime;
use conventional_commit_parser::commit::Footer;
use serde::Serialize;

use crate::conventional::commit::Commit;

use crate::settings;

#[derive(Serialize)]
pub struct Release<'a> {
    pub version: Option<String>,
    pub date: NaiveDateTime,
    pub commits: Vec<ChangelogCommit<'a>>,
    pub previous: Option<Box<Release<'a>>>,
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
    use indoc::indoc;

    #[test]
    fn should_render_default_template() {
        let paul_delafosse = "Paul Delafosse";
        let a_commit_hash = "17f7e23081db15e9318aeb37529b1d473cf41cbe";
        let version = Release {
            version: Some("1.0.0".to_string()),
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
            previous: Some(Box::new(Release {
                version: Some("0.3.0".to_string()),
                date: Utc::now().naive_utc(),
                commits: vec![],
                previous: None,
            })),
        };

        let renderer = Renderer::default();
        let changelog = renderer.render(&version);

        assert_that!(changelog).is_ok().is_equal_to(
            indoc! {
                "## 1.0.0 - 2021-10-28
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
