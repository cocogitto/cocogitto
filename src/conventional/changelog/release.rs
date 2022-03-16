use chrono::{NaiveDateTime, Utc};
use conventional_commit_parser::commit::Footer;
use serde::Serialize;

use crate::conventional::commit::Commit;
use crate::git::oid::OidOf;
use crate::git::revspec::CommitRange;
use crate::settings;
use colored::Colorize;

#[derive(Debug, Serialize)]
pub struct Release<'a> {
    pub version: OidOf,
    pub from: OidOf,
    pub date: NaiveDateTime,
    pub commits: Vec<ChangelogCommit<'a>>,
    pub previous: Option<Box<Release<'a>>>,
}

impl<'a> From<CommitRange<'a>> for Release<'a> {
    fn from(commit_range: CommitRange<'a>) -> Self {
        let mut commits = vec![];

        for commit in commit_range.commits {
            // Ignore merge commits
            if let Some(message) = commit.message() {
                if message.starts_with("Merge") {
                    continue;
                }
            }

            match Commit::from_git_commit(&commit) {
                Ok(commit) => commits.push(ChangelogCommit::from(commit)),
                Err(err) => {
                    let err = err.to_string().red();
                    eprintln!("{}", err);
                }
            };
        }

        Release {
            version: commit_range.to,
            from: commit_range.from,
            date: Utc::now().naive_utc(),
            commits,
            previous: None,
        }
    }
}

#[derive(Debug)]
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
    use anyhow::Result;
    use chrono::NaiveDateTime;
    use conventional_commit_parser::commit::{CommitType, ConventionalCommit, Footer};
    use git2::Oid;
    use indoc::indoc;
    use pretty_assertions::assert_eq;

    use crate::conventional::changelog::release::{ChangelogCommit, Release};
    use crate::conventional::changelog::renderer::Renderer;
    use crate::conventional::changelog::template::{RemoteContext, Template, TemplateKind};
    use crate::conventional::commit::Commit;
    use crate::git::oid::OidOf;
    use crate::git::tag::Tag;

    #[test]
    fn should_render_default_template() -> Result<()> {
        // Arrange
        let release = Release::fixture();
        let renderer = Renderer::default();

        // Act
        let changelog = renderer.render(release)?;

        // Assert
        assert_eq!(
            changelog,
            indoc! {
                "## 1.0.0 - 2015-09-05
                #### Bug Fixes
                - **(parser)** fix parser implementation - (17f7e23) - *oknozor*
                #### Features
                - **(parser)** implement the changelog generator - (17f7e23) - *oknozor*
                - awesome feature - (17f7e23) - Paul Delafosse
                "
            }
        );

        Ok(())
    }

    #[test]
    fn should_render_full_hash_template() -> Result<()> {
        // Arrange
        let release = Release::fixture();
        let renderer = Renderer::try_new(Template {
            context: None,
            kind: TemplateKind::FullHash,
        })?;

        // Act
        let changelog = renderer.render(release)?;

        // Assert
        assert_eq!(
            changelog,
            indoc! {
                "#### Bug Fixes
                - 17f7e23081db15e9318aeb37529b1d473cf41cbe - **(parser)** fix parser implementation - @oknozor
                #### Features
                - 17f7e23081db15e9318aeb37529b1d473cf41cbe - **(parser)** implement the changelog generator - @oknozor
                - 17f7e23081db15e9318aeb37529b1d473cf41cbe - awesome feature - Paul Delafosse

                "
            }
        );

        Ok(())
    }

    #[test]
    fn should_render_github_template() -> Result<()> {
        // Arrange
        let release = Release::fixture();
        let renderer = Renderer::try_new(Template {
            context: RemoteContext::try_new(
                Some("github.com".into()),
                Some("cocogitto".into()),
                Some("cocogitto".into()),
            ),
            kind: TemplateKind::Remote,
        })?;

        // Act
        let changelog = renderer.render(release)?;

        // Assert
        assert_eq!(
            changelog,
            indoc! {
                "## [1.0.0](https://github.com/cocogitto/cocogitto/compare/0.1.0..1.0.0) - 2015-09-05
                #### Bug Fixes
                - **(parser)** fix parser implementation - ([17f7e23](https://github.com/cocogitto/cocogitto/commit/17f7e23081db15e9318aeb37529b1d473cf41cbe)) - [@oknozor](https://github.com/oknozor)
                #### Features
                - **(parser)** implement the changelog generator - ([17f7e23](https://github.com/cocogitto/cocogitto/commit/17f7e23081db15e9318aeb37529b1d473cf41cbe)) - [@oknozor](https://github.com/oknozor)
                - awesome feature - ([17f7e23](https://github.com/cocogitto/cocogitto/commit/17f7e23081db15e9318aeb37529b1d473cf41cbe)) - Paul Delafosse
                "
            }
        );

        Ok(())
    }

    impl Release<'_> {
        pub fn fixture() -> Release<'static> {
            let date =
                NaiveDateTime::parse_from_str("2015-09-05 23:56:04", "%Y-%m-%d %H:%M:%S").unwrap();

            let paul_delafosse = "Paul Delafosse";
            let a_commit_hash = "17f7e23081db15e9318aeb37529b1d473cf41cbe";
            let version = Tag::new(
                "1.0.0",
                Some(Oid::from_str("9bb5facac5724bc81385fdd740fedbb49056da00").unwrap()),
            )
            .unwrap();
            let from = Tag::new(
                "0.1.0",
                Some(Oid::from_str("fae3a288a1bc69b14f85a1d5fe57cee1964acd60").unwrap()),
            )
            .unwrap();
            Release {
                version: OidOf::Tag(version),
                from: OidOf::Tag(from),
                date,
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
                                    ..Default::default()
                                }],
                                is_breaking_change: false,
                            },
                            author: paul_delafosse.to_string(),
                            date,
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
                                    ..Default::default()
                                }],
                                is_breaking_change: false,
                            },
                            author: paul_delafosse.to_string(),
                            date,
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
                                    ..Default::default()
                                }],
                                is_breaking_change: false,
                            },
                            author: "James Delleck".to_string(),
                            date,
                        },
                    },
                ],
                previous: None,
            }
        }
    }
}
