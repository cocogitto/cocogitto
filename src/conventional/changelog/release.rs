use chrono::{NaiveDateTime, Utc};
use conventional_commit_parser::commit::Footer;
use serde::Serialize;

use crate::conventional::commit::Commit;
use crate::git::oid::OidOf;
use crate::git::revspec::CommitRange;
use crate::{settings, SETTINGS};
use colored::Colorize;
use git2::Oid;
use log::warn;

#[derive(Debug, Serialize)]
pub struct Release<'a> {
    pub version: OidOf,
    pub from: OidOf,
    pub date: NaiveDateTime,
    pub commits: Vec<ChangelogCommit<'a>>,
    pub previous: Option<Box<Release<'a>>>,
}

impl Release<'_> {
    pub fn drain_to_target(&mut self, target: &Oid) {
        let target_idx = self
            .commits
            .iter()
            .enumerate()
            .find(|(_idx, commit)| commit.commit.oid == target.to_string())
            .map(|(idx, _)| idx);

        match target_idx {
            None => {
                if let Some(previous) = &mut self.previous {
                    previous.drain_to_target(target)
                }
            }
            Some(idx) => {
                if self.commits.get(idx + 1).is_some() {
                    self.commits.drain(idx + 1..);
                }
            }
        }
    }

    pub fn contains_oid(&self, oid: &Oid) -> bool {
        self.commits
            .iter()
            .any(|commit| commit.commit.oid == oid.to_string())
    }
}

impl<'a> From<CommitRange<'a>> for Release<'a> {
    fn from(commit_range: CommitRange<'a>) -> Self {
        let mut commits = vec![];

        for commit in commit_range.commits {
            // Ignore merge commits
            if commit.parent_count() > 1 && SETTINGS.ignore_merge_commits {
                continue;
            }

            match Commit::from_git_commit(&commit) {
                Ok(commit) => {
                    if !commit.should_omit() {
                        commits.push(ChangelogCommit::from(commit))
                    }
                }
                Err(err) => {
                    let err = err.to_string().red();
                    warn!("{}", err);
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
    use crate::conventional::changelog::template::{
        MonoRepoContext, PackageBumpContext, PackageContext, RemoteContext, Template, TemplateKind,
    };
    use crate::conventional::commit::Commit;
    use crate::git::oid::OidOf;
    use crate::git::tag::Tag;

    #[test]
    fn should_render_default_template() -> Result<()> {
        // Arrange
        let release = Release::fixture();
        let mut renderer = Renderer::default();

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
        let mut renderer = Renderer::try_new(Template {
            remote_context: None,
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
        let mut renderer = Renderer::try_new(Template {
            remote_context: RemoteContext::try_new(
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

    #[test]
    fn should_render_template_monorepo() -> Result<()> {
        // Arrange
        let release = Release::fixture();
        let renderer = Renderer::try_new(Template {
            remote_context: None,
            kind: TemplateKind::MonorepoDefault,
        })?;

        let mut renderer = monorepo_renderer(renderer)?;

        // Act
        let changelog = renderer.render(release)?;

        // Assert
        assert_eq!(
            changelog,
            indoc! {
                "## 1.0.0 - 2015-09-05
                ### Package updates
                - one bumped to 0.1.0
                - two bumped to 0.2.0
                ### Global changes
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
    fn should_render_template_monorepo_for_manual_bump() -> Result<()> {
        // Arrange
        let release = Release::fixture();
        let renderer = Renderer::try_new(Template {
            remote_context: None,
            kind: TemplateKind::MonorepoDefault,
        })?;

        let mut renderer = monorepo_manual_bump_rendered(renderer)?;

        // Act
        let changelog = renderer.render(release)?;

        // Assert
        assert_eq!(
            changelog,
            indoc! {
                "## 1.0.0 - 2015-09-05
                ### Packages
                - one locked to 0.1.0
                - two locked to 0.2.0
                ### Global changes
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
    fn should_render_full_hash_template_monorepo() -> Result<()> {
        // Arrange
        let release = Release::fixture();
        let renderer = Renderer::try_new(Template {
            remote_context: None,
            kind: TemplateKind::MonorepoFullHash,
        })?;

        let mut renderer = monorepo_renderer(renderer)?;

        // Act
        let changelog = renderer.render(release)?;

        // Assert
        assert_eq!(
            changelog,
            indoc! {
                "### Package updates
                - one bumped to 0.1.0
                - two bumped to 0.2.0
                ### Global changes
                #### Bug Fixes
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
    fn should_render_full_hash_template_manual_monorepo() -> Result<()> {
        // Arrange
        let release = Release::fixture();
        let renderer = Renderer::try_new(Template {
            remote_context: None,
            kind: TemplateKind::MonorepoFullHash,
        })?;

        let mut renderer = monorepo_manual_bump_rendered(renderer)?;

        // Act
        let changelog = renderer.render(release)?;

        // Assert
        assert_eq!(
            changelog,
            indoc! {
                "### Packages
                - one locked to 0.1.0
                - two locked to 0.2.0
                ### Global changes
                #### Bug Fixes
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
    fn should_render_remote_template_monorepo() -> Result<()> {
        // Arrange
        let release = Release::fixture();
        let renderer = Renderer::try_new(Template {
            remote_context: RemoteContext::try_new(
                Some("github.com".into()),
                Some("cocogitto".into()),
                Some("cocogitto".into()),
            ),
            kind: TemplateKind::MonorepoRemote,
        })?;

        let mut renderer = monorepo_renderer(renderer)?;

        // Act
        let changelog = renderer.render(release)?;

        // Assert
        assert_eq!(
            changelog,
            indoc! {
                "## [1.0.0](https://github.com/cocogitto/cocogitto/compare/0.1.0..1.0.0) - 2015-09-05
                ### Package updates
                - [0.1.0](crates/one) bumped to [0.1.0](https://github.com/cocogitto/cocogitto/compare/0.2.0..0.1.0)
                - [0.2.0](crates/two) bumped to [0.2.0](https://github.com/cocogitto/cocogitto/compare/0.3.0..0.2.0)
                ### Global changes
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

    #[test]
    fn should_render_template_package() -> Result<()> {
        // Arrange
        let release = Release::fixture();
        let renderer = Renderer::try_new(Template {
            remote_context: None,
            kind: TemplateKind::PackageDefault,
        })?;

        let mut renderer = package_renderer(renderer)?;

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
    fn should_render_full_hash_template_package() -> Result<()> {
        // Arrange
        let release = Release::fixture();
        let renderer = Renderer::try_new(Template {
            remote_context: None,
            kind: TemplateKind::PackageFullHash,
        })?;

        let mut renderer = package_renderer(renderer)?;

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
    fn should_render_remote_template_package() -> Result<()> {
        // Arrange
        let release = Release::fixture();
        let renderer = Renderer::try_new(Template {
            remote_context: RemoteContext::try_new(
                Some("github.com".into()),
                Some("cocogitto".into()),
                Some("cocogitto".into()),
            ),
            kind: TemplateKind::PackageRemote,
        })?;

        let mut renderer = package_renderer(renderer)?;

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

    #[test]
    fn should_render_remote_template_monorepo_for_manual_bump() -> Result<()> {
        // Arrange
        let release = Release::fixture();
        let renderer = Renderer::try_new(Template {
            remote_context: RemoteContext::try_new(
                Some("github.com".into()),
                Some("cocogitto".into()),
                Some("cocogitto".into()),
            ),
            kind: TemplateKind::MonorepoRemote,
        })?;

        let mut renderer = monorepo_manual_bump_rendered(renderer)?;

        // Act
        let changelog = renderer.render(release)?;

        // Assert
        assert_eq!(
            changelog,
            indoc! {
                "## [1.0.0](https://github.com/cocogitto/cocogitto/compare/0.1.0..1.0.0) - 2015-09-05
                ### Packages
                - [0.1.0](crates/one) locked to [0.1.0](https://github.com/cocogitto/cocogitto/tree/0.1.0)
                - [0.2.0](crates/two) locked to [0.2.0](https://github.com/cocogitto/cocogitto/tree/0.2.0)
                ### Global changes
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
            let version = Tag::from_str(
                "1.0.0",
                Some(Oid::from_str("9bb5facac5724bc81385fdd740fedbb49056da00").unwrap()),
            )
            .unwrap();
            let from = Tag::from_str(
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
                            conventional: ConventionalCommit {
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
                            conventional: ConventionalCommit {
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
                            conventional: ConventionalCommit {
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

    fn monorepo_renderer(renderer: Renderer) -> Result<Renderer> {
        let renderer = renderer.with_monorepo_context(MonoRepoContext {
            package_lock: false,
            packages: vec![
                PackageBumpContext {
                    package_name: "one",
                    package_path: "crates/one",
                    version: OidOf::Tag(Tag::from_str(
                        "0.1.0",
                        Some(Oid::from_str("fae3a288a1bc69b14f85a1d5fe57cee1964acd60").unwrap()),
                    )?),
                    from: Some(OidOf::Tag(Tag::from_str(
                        "0.2.0",
                        Some(Oid::from_str("fae3a288a1bc69b14f85a1d5fe57cee1964acd60").unwrap()),
                    )?)),
                },
                PackageBumpContext {
                    package_name: "two",
                    package_path: "crates/two",
                    version: OidOf::Tag(Tag::from_str(
                        "0.2.0",
                        Some(Oid::from_str("fae3a288a1bc69b14f85a1d5fe57cee1964acd60").unwrap()),
                    )?),
                    from: Some(OidOf::Tag(Tag::from_str(
                        "0.3.0",
                        Some(Oid::from_str("fae3a288a1bc69b14f85a1d5fe57cee1964acd60").unwrap()),
                    )?)),
                },
            ],
        });

        Ok(renderer)
    }

    fn monorepo_manual_bump_rendered(renderer: Renderer) -> Result<Renderer> {
        let renderer = renderer.with_monorepo_context(MonoRepoContext {
            package_lock: true,
            packages: vec![
                PackageBumpContext {
                    package_name: "one",
                    package_path: "crates/one",
                    version: OidOf::Tag(Tag::from_str(
                        "0.1.0",
                        Some(Oid::from_str("fae3a288a1bc69b14f85a1d5fe57cee1964acd60").unwrap()),
                    )?),
                    from: None,
                },
                PackageBumpContext {
                    package_name: "two",
                    package_path: "crates/two",
                    version: OidOf::Tag(Tag::from_str(
                        "0.2.0",
                        Some(Oid::from_str("fae3a288a1bc69b14f85a1d5fe57cee1964acd60").unwrap()),
                    )?),
                    from: None,
                },
            ],
        });

        Ok(renderer)
    }

    fn package_renderer(renderer: Renderer) -> Result<Renderer> {
        let renderer = renderer.with_package_context(PackageContext {
            package_name: "one",
        });

        Ok(renderer)
    }
}
