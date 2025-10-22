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
pub struct Release<'a> {
    pub version: OidOf,
    pub from: OidOf,
    pub date: NaiveDateTime,
    pub commits: Vec<ChangelogCommit<'a>>,
    pub previous: Option<Box<Release<'a>>>,
}

impl TryFrom<CommitIter<'_>> for Release<'_> {
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
                date: Utc::now().naive_local(),
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
                Self::GithubCoAuthoredBy {
                    user: footer.content.as_str(),
                }
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
                token: &footer.content.as_str(),
                content: footer.content.as_str(),
            },
        }
    }
}

#[cfg(test)]
mod test {
    use chrono::NaiveDateTime;
    use conventional_commit_parser::commit::{CommitType, ConventionalCommit, Footer, Separator};
    use git2::Oid;
    use indoc::indoc;
    use itertools::Itertools;
    use pretty_assertions::assert_eq;
    use speculoos::prelude::*;

    use crate::conventional::changelog::context::{
        MonoRepoContext, PackageBumpContext, RemoteContext,
    };
    use crate::conventional::changelog::release::{ChangelogCommit, Release};
    use crate::conventional::changelog::template::Template;
    use crate::conventional::commit::Commit;
    use crate::git::oid::OidOf;
    use crate::git::repository::Repository;

    use crate::git::tag::Tag;

    macro_rules! assert_doc_eq {
        ($changelog:expr, $doc:literal) => {
            assert_eq!(
                $changelog.split('\n').map(|line| line.trim()).join("\n"),
                indoc!($doc).split('\n').map(|line| line.trim()).join("\n")
            )
        };
    }

    macro_rules! changelog_test {
        (
            $test_name:ident,
            $release_fixture:expr,
            $template:expr,
            $expected:literal $(,)?
        ) => {
            #[test]
            fn $test_name() -> anyhow::Result<()> {
                let release = $release_fixture.build();
                let changelog = $template.render(release)?;
                assert_doc_eq!(changelog, $expected);
                Ok(())
            }
        };
        (
            $test_name:ident,
            $release_fixture:expr,
            $template:expr,
            $expected:literal,
            $context:expr $(,)?
        ) => {
            #[test]
            fn $test_name() -> anyhow::Result<()> {
                let release = $release_fixture.build();
                let changelog = $template.with_context($context).render(release)?;
                assert_doc_eq!(changelog, $expected);
                Ok(())
            }
        };
    }
    #[test]
    fn should_get_a_release() -> anyhow::Result<()> {
        let repo = Repository::open(".")?;
        let iter = repo.revwalk("..")?;
        let release = Release::try_from(iter);
        assert_that!(release)
            .is_ok()
            .matches(|r| !r.commits.is_empty());
        Ok(())
    }

    changelog_test!(
        should_render_default_template,
        ReleaseFixture::default(),
        Template::from_arg("default", None)?,
        "## 1.0.0 - 2015-09-05
        #### Features
        - (**parser**) implement the changelog generator - (17f7e23) - *oknozor*
        - awesome feature - (17f7e23) - Paul Delafosse
        #### Bug Fixes
        - (**parser**) fix parser implementation - (17f7e23) - *oknozor*
        "
    );

    changelog_test!(
        should_render_full_hash_template,
        ReleaseFixture::default(),
        Template::from_arg("full_hash", default_remote_context())?,
        "#### Features
        - 17f7e23081db15e9318aeb37529b1d473cf41cbe - (**parser**) implement the changelog generator - @oknozor
        - 17f7e23081db15e9318aeb37529b1d473cf41cbe - awesome feature - Paul Delafosse

        #### Bug Fixes
        - 17f7e23081db15e9318aeb37529b1d473cf41cbe - (**parser**) fix parser implementation - @oknozor
        "
    );

    changelog_test!(
        should_render_github_template,
        ReleaseFixture::default(),
        Template::from_arg("remote", default_remote_context())?,
        "## [1.0.0](https://github.com/cocogitto/cocogitto/compare/0.1.0..1.0.0) - 2015-09-05
        #### Features
        - (**parser**) implement the changelog generator - ([17f7e23](https://github.com/cocogitto/cocogitto/commit/17f7e23081db15e9318aeb37529b1d473cf41cbe)) - [@oknozor](https://github.com/oknozor)
        - awesome feature - ([17f7e23](https://github.com/cocogitto/cocogitto/commit/17f7e23081db15e9318aeb37529b1d473cf41cbe)) - Paul Delafosse
        #### Bug Fixes
        - (**parser**) fix parser implementation - ([17f7e23](https://github.com/cocogitto/cocogitto/commit/17f7e23081db15e9318aeb37529b1d473cf41cbe)) - [@oknozor](https://github.com/oknozor)
        "
    );

    changelog_test!(
        should_render_template_monorepo,
        ReleaseFixture::default(),
        Template::from_arg("monorepo_default", default_remote_context())?
            .with_context(monorepo_context()),
        "## 1.0.0 - 2015-09-05
        ### Package updates
        - one bumped to 0.1.0
        - two bumped to 0.2.0
        ### Global changes
        #### Features
        - (**parser**) implement the changelog generator - (17f7e23) - *oknozor*
        - awesome feature - (17f7e23) - Paul Delafosse
        #### Bug Fixes
        - (**parser**) fix parser implementation - (17f7e23) - *oknozor*
        "
    );

    changelog_test!(
        should_render_template_monorepo_for_manual_bump,
        ReleaseFixture::default(),
        Template::from_arg("monorepo_default", None)?.with_context(default_pacakage_context()),
        "## 1.0.0 - 2015-09-05
        ### Packages
        - one locked to 0.1.0
        - two locked to 0.2.0
        ### Global changes
        #### Features
        - (**parser**) implement the changelog generator - (17f7e23) - *oknozor*
        - awesome feature - (17f7e23) - Paul Delafosse
        #### Bug Fixes
        - (**parser**) fix parser implementation - (17f7e23) - *oknozor*
        "
    );

    changelog_test!(
        should_render_full_hash_template_monorepo,
        ReleaseFixture::default(),
        Template::from_arg("monorepo_full_hash", default_remote_context())?
            .with_context(monorepo_context()),
        "### Package updates
        - one bumped to 0.1.0
        - two bumped to 0.2.0
        ### Global changes
        #### Features
        - 17f7e23081db15e9318aeb37529b1d473cf41cbe - (**parser**) implement the changelog generator - @oknozor
        - 17f7e23081db15e9318aeb37529b1d473cf41cbe - awesome feature - Paul Delafosse

        #### Bug Fixes
        - 17f7e23081db15e9318aeb37529b1d473cf41cbe - (**parser**) fix parser implementation - @oknozor
        "
    );

    changelog_test!(
        should_render_full_hash_template_manual_monorepo,
        ReleaseFixture::default(),
        Template::from_arg("monorepo_full_hash", default_remote_context())?
            .with_context(default_pacakage_context()),
        "### Packages
        - one locked to 0.1.0
        - two locked to 0.2.0
        ### Global changes
        #### Features
        - 17f7e23081db15e9318aeb37529b1d473cf41cbe - (**parser**) implement the changelog generator - @oknozor
        - 17f7e23081db15e9318aeb37529b1d473cf41cbe - awesome feature - Paul Delafosse

        #### Bug Fixes
        - 17f7e23081db15e9318aeb37529b1d473cf41cbe - (**parser**) fix parser implementation - @oknozor
        "
    );

    changelog_test!(
        should_render_remote_template_monorepo,
        ReleaseFixture::default(),
        Template::from_arg("monorepo_remote", default_remote_context())?
            .with_context(monorepo_context()),
        "## [1.0.0](https://github.com/cocogitto/cocogitto/compare/0.1.0..1.0.0) - 2015-09-05
        ### Package updates
        - [0.1.0](crates/one) bumped to [0.1.0](https://github.com/cocogitto/cocogitto/compare/0.2.0..0.1.0)
        - [0.2.0](crates/two) bumped to [0.2.0](https://github.com/cocogitto/cocogitto/compare/0.3.0..0.2.0)
        ### Global changes
        #### Features
        - (**parser**) implement the changelog generator - ([17f7e23](https://github.com/cocogitto/cocogitto/commit/17f7e23081db15e9318aeb37529b1d473cf41cbe)) - [@oknozor](https://github.com/oknozor)
        - awesome feature - ([17f7e23](https://github.com/cocogitto/cocogitto/commit/17f7e23081db15e9318aeb37529b1d473cf41cbe)) - Paul Delafosse
        #### Bug Fixes
        - (**parser**) fix parser implementation - ([17f7e23](https://github.com/cocogitto/cocogitto/commit/17f7e23081db15e9318aeb37529b1d473cf41cbe)) - [@oknozor](https://github.com/oknozor)
        "
    );

    changelog_test!(
        should_render_template_package,
        ReleaseFixture::default(),
        Template::from_arg("package_default", default_remote_context())?,
        "## 1.0.0 - 2015-09-05
        #### Features
        - (**parser**) implement the changelog generator - (17f7e23) - *oknozor*
        - awesome feature - (17f7e23) - Paul Delafosse
        #### Bug Fixes
        - (**parser**) fix parser implementation - (17f7e23) - *oknozor*
        "
    );

    changelog_test!(
        should_render_full_hash_template_package,
        ReleaseFixture::default(),
        Template::from_arg("full_hash", default_remote_context())?,
        "#### Features
        - 17f7e23081db15e9318aeb37529b1d473cf41cbe - (**parser**) implement the changelog generator - @oknozor
        - 17f7e23081db15e9318aeb37529b1d473cf41cbe - awesome feature - Paul Delafosse

        #### Bug Fixes
        - 17f7e23081db15e9318aeb37529b1d473cf41cbe - (**parser**) fix parser implementation - @oknozor
        "
    );

    changelog_test!(
        should_render_remote_template_package,
        ReleaseFixture::default(),
        Template::from_arg("package_remote", default_remote_context())?,
        "## [1.0.0](https://github.com/cocogitto/cocogitto/compare/0.1.0..1.0.0) - 2015-09-05
        #### Features
        - (**parser**) implement the changelog generator - ([17f7e23](https://github.com/cocogitto/cocogitto/commit/17f7e23081db15e9318aeb37529b1d473cf41cbe)) - [@oknozor](https://github.com/oknozor)
        - awesome feature - ([17f7e23](https://github.com/cocogitto/cocogitto/commit/17f7e23081db15e9318aeb37529b1d473cf41cbe)) - Paul Delafosse
        #### Bug Fixes
        - (**parser**) fix parser implementation - ([17f7e23](https://github.com/cocogitto/cocogitto/commit/17f7e23081db15e9318aeb37529b1d473cf41cbe)) - [@oknozor](https://github.com/oknozor)
        "
    );

    changelog_test!(
        should_render_remote_template_monorepo_for_manual_bump,
        ReleaseFixture::default(),
        Template::from_arg("monorepo_remote", default_remote_context())?
            .with_context(default_pacakage_context()),
        "## [1.0.0](https://github.com/cocogitto/cocogitto/compare/0.1.0..1.0.0) - 2015-09-05
        ### Packages
        - [0.1.0](crates/one) locked to [0.1.0](https://github.com/cocogitto/cocogitto/tree/0.1.0)
        - [0.2.0](crates/two) locked to [0.2.0](https://github.com/cocogitto/cocogitto/tree/0.2.0)
        ### Global changes
        #### Features
        - (**parser**) implement the changelog generator - ([17f7e23](https://github.com/cocogitto/cocogitto/commit/17f7e23081db15e9318aeb37529b1d473cf41cbe)) - [@oknozor](https://github.com/oknozor)
        - awesome feature - ([17f7e23](https://github.com/cocogitto/cocogitto/commit/17f7e23081db15e9318aeb37529b1d473cf41cbe)) - Paul Delafosse
        #### Bug Fixes
        - (**parser**) fix parser implementation - ([17f7e23](https://github.com/cocogitto/cocogitto/commit/17f7e23081db15e9318aeb37529b1d473cf41cbe)) - [@oknozor](https://github.com/oknozor)
        "
    );

    #[test]
    fn should_render_github_footers() -> anyhow::Result<()> {
        let release = ReleaseFixture::builder()
            .with_commit(CommitFixture::default().with_footer(
                "Co-authored-by",
                "Toml Bombadil <tom.bombadil@lorien.com>",
                Separator::Colon,
            ))
            .build();

        let changelog = Template::from_arg("github", default_remote_context())?.render(release)?;
        assert_doc_eq!(changelog, "");
        Ok(())
    }

    pub struct ReleaseFixture<'a> {
        pub release: Release<'a>,
    }
    #[test]
    fn should_render_unified_default() -> Result<()> {
        // Arrange
        let release = Release::fixture();

        let renderer = Renderer::try_new(Template {
            remote_context: None,
            kind: TemplateKind::UnifiedDefault,
        })?;
        let mut renderer = monorepo_renderer(renderer)?;

        // Act
        let changelog = renderer.render(release)?;

        // Assert
        assert_doc_eq!(
            changelog,
            "## 1.0.0 - 2015-09-05
            ### Package updates
            - one bumped to 0.1.0
            - two bumped to 0.2.0
            ### All changes
            #### Features
            - (**parser**) implement the changelog generator - (17f7e23) - *oknozor*
            - awesome feature - (17f7e23) - Paul Delafosse
            #### Bug Fixes
            - (**parser**) fix parser implementation - (17f7e23) - *oknozor*
            "
        );

        Ok(())
    }

    #[test]
    fn should_render_unified_full_hash() -> Result<()> {
        // Arrange
        let release = Release::fixture();

        let renderer = Renderer::try_new(Template {
            remote_context: None,
            kind: TemplateKind::UnifiedFullHash,
        })?;
        let mut renderer = monorepo_renderer(renderer)?;

        // Act
        let changelog = renderer.render(release)?;

        // Assert
        assert_doc_eq!(changelog,
            "### Package updates
            - one bumped to 0.1.0
            - two bumped to 0.2.0
            ### All changes
            #### Features
            - 17f7e23081db15e9318aeb37529b1d473cf41cbe - (**parser**) implement the changelog generator - @oknozor
            - 17f7e23081db15e9318aeb37529b1d473cf41cbe - awesome feature - Paul Delafosse

            #### Bug Fixes
            - 17f7e23081db15e9318aeb37529b1d473cf41cbe - (**parser**) fix parser implementation - @oknozor
            "
        );

        Ok(())
    }

    #[test]
    fn should_render_unified_remote() -> Result<()> {
        // Arrange
        let release = Release::fixture();

        let renderer = Renderer::try_new(Template {
            remote_context: RemoteContext::try_new(
                Some("github.com".into()),
                Some("cocogitto".into()),
                Some("cocogitto".into()),
            ),
            kind: TemplateKind::UnifiedRemote,
        })?;
        let mut renderer = monorepo_renderer(renderer)?;

        // Act
        let changelog = renderer.render(release)?;

        // Assert
        assert_doc_eq!(changelog,
            "## [1.0.0](https://github.com/cocogitto/cocogitto/compare/0.1.0..1.0.0) - 2015-09-05
            ### Package updates
            - [0.1.0](crates/one) bumped to [0.1.0](https://github.com/cocogitto/cocogitto/compare/0.2.0..0.1.0)
            - [0.2.0](crates/two) bumped to [0.2.0](https://github.com/cocogitto/cocogitto/compare/0.3.0..0.2.0)
            ### All changes
            #### Features
            - (**parser**) implement the changelog generator - ([17f7e23](https://github.com/cocogitto/cocogitto/commit/17f7e23081db15e9318aeb37529b1d473cf41cbe)) - [@oknozor](https://github.com/oknozor)
            - awesome feature - ([17f7e23](https://github.com/cocogitto/cocogitto/commit/17f7e23081db15e9318aeb37529b1d473cf41cbe)) - Paul Delafosse
            #### Bug Fixes
            - (**parser**) fix parser implementation - ([17f7e23](https://github.com/cocogitto/cocogitto/commit/17f7e23081db15e9318aeb37529b1d473cf41cbe)) - [@oknozor](https://github.com/oknozor)
            "
        );

        Ok(())
    }

    impl Release<'_> {
        pub fn fixture() -> Release<'static> {
            let date =
                NaiveDateTime::parse_from_str("2015-09-05 23:56:04", "%Y-%m-%d %H:%M:%S").unwrap();

    impl<'a> ReleaseFixture<'a> {
        fn builder() -> ReleaseFixture<'a> {
            ReleaseFixture {
                release: Release {
                    version: OidOf::Tag(
                        Tag::from_str(
                            "1.0.0",
                            Some(
                                Oid::from_str("9bb5facac5724bc81385fdd740fedbb49056da00").unwrap(),
                            ),
                        )
                        .unwrap(),
                    ),
                    from: OidOf::Tag(
                        Tag::from_str(
                            "0.1.0",
                            Some(
                                Oid::from_str("fae3a288a1bc69b14f85a1d5fe57cee1964acd60").unwrap(),
                            ),
                        )
                        .unwrap(),
                    ),
                    date: NaiveDateTime::parse_from_str("2015-09-05 23:56:04", "%Y-%m-%d %H:%M:%S")
                        .unwrap(),
                    commits: vec![],
                    previous: None,
                },
            }
        }

        fn build(self) -> Release<'a> {
            self.release
        }

        fn with_commit(mut self, commit: CommitFixture<'a>) -> Self {
            self.release.commits.push(commit.build());
            self
        }
    }

    impl<'a> Default for ReleaseFixture<'a> {
        fn default() -> Self {
            return ReleaseFixture::builder()
                .with_commit(
                    CommitFixture::default()
                        .with_scope("parser")
                        .with_commit_type(CommitType::Feature)
                        .with_username("oknozor")
                        .with_message("implement the changelog generator"),
                )
                .with_commit(
                    CommitFixture::default()
                        .with_message("awesome feature")
                        .with_commit_type(CommitType::Feature),
                )
                .with_commit(
                    CommitFixture::default()
                        .with_scope("parser")
                        .with_username("oknozor")
                        .with_message("fix parser implementation"),
                );
        }
    }

    struct CommitFixture<'a> {
        changelog: ChangelogCommit<'a>,
    }

    impl<'a> CommitFixture<'a> {
        fn with_commit_type(mut self, commit_type: CommitType) -> Self {
            self.changelog.commit.conventional.commit_type = commit_type;
            self
        }

        fn with_username(mut self, author: &'a str) -> Self {
            self.changelog.author_username = Some(author);
            self
        }

        fn with_scope(mut self, scope: &str) -> Self {
            self.changelog.commit.conventional.scope = Some(scope.to_string());
            self
        }

        fn with_message(mut self, message: &str) -> Self {
            self.changelog.commit.conventional.summary = message.to_string();
            self
        }

        fn with_footer(mut self, token: &str, content: &str, token_separator: Separator) -> Self {
            self.changelog.commit.conventional.footers.push(Footer {
                token: token.to_string(),
                content: content.to_string(),
                token_separator,
            });
            self
        }

        fn with_breaking(mut self) -> Self {
            self.changelog.commit.conventional.is_breaking_change = true;
            self
        }

        fn build(self) -> ChangelogCommit<'a> {
            self.changelog
        }
    }

    impl Default for CommitFixture<'_> {
        fn default() -> Self {
            Self {
                changelog: ChangelogCommit {
                    author_username: None,
                    commit: Commit {
                        oid: "17f7e23081db15e9318aeb37529b1d473cf41cbe".to_string(),
                        conventional: ConventionalCommit {
                            commit_type: CommitType::BugFix,
                            scope: None,
                            summary: "fix parser implementation".to_string(),
                            body: None,
                            footers: vec![Footer {
                                token: "token".to_string(),
                                content: "content".to_string(),
                                ..Default::default()
                            }],
                            is_breaking_change: false,
                        },
                        author: "Paul Delafosse".to_string(),
                        date: NaiveDateTime::parse_from_str(
                            "2015-09-05 23:56:04",
                            "%Y-%m-%d %H:%M:%S",
                        )
                        .unwrap(),
                    },
                },
            }
        }
    }

    fn monorepo_context<'a>() -> MonoRepoContext<'a> {
        MonoRepoContext {
            package_lock: false,
            packages: vec![
                PackageBumpContext {
                    package_name: "one",
                    package_path: "crates/one",
                    version: OidOf::Tag(
                        Tag::from_str(
                            "0.1.0",
                            Some(
                                Oid::from_str("fae3a288a1bc69b14f85a1d5fe57cee1964acd60").unwrap(),
                            ),
                        )
                        .unwrap(),
                    ),
                    from: Some(OidOf::Tag(
                        Tag::from_str(
                            "0.2.0",
                            Some(
                                Oid::from_str("fae3a288a1bc69b14f85a1d5fe57cee1964acd60").unwrap(),
                            ),
                        )
                        .unwrap(),
                    )),
                },
                PackageBumpContext {
                    package_name: "two",
                    package_path: "crates/two",
                    version: OidOf::Tag(
                        Tag::from_str(
                            "0.2.0",
                            Some(
                                Oid::from_str("fae3a288a1bc69b14f85a1d5fe57cee1964acd60").unwrap(),
                            ),
                        )
                        .unwrap(),
                    ),
                    from: Some(OidOf::Tag(
                        Tag::from_str(
                            "0.3.0",
                            Some(
                                Oid::from_str("fae3a288a1bc69b14f85a1d5fe57cee1964acd60").unwrap(),
                            ),
                        )
                        .unwrap(),
                    )),
                },
            ],
        }
    }

    fn default_pacakage_context<'a>() -> MonoRepoContext<'a> {
        MonoRepoContext {
            package_lock: true,
            packages: vec![
                PackageBumpContext {
                    package_name: "one",
                    package_path: "crates/one",
                    version: OidOf::Tag(
                        Tag::from_str(
                            "0.1.0",
                            Some(
                                Oid::from_str("fae3a288a1bc69b14f85a1d5fe57cee1964acd60").unwrap(),
                            ),
                        )
                        .unwrap(),
                    ),
                    from: None,
                },
                PackageBumpContext {
                    package_name: "two",
                    package_path: "crates/two",
                    version: OidOf::Tag(
                        Tag::from_str(
                            "0.2.0",
                            Some(
                                Oid::from_str("fae3a288a1bc69b14f85a1d5fe57cee1964acd60").unwrap(),
                            ),
                        )
                        .unwrap(),
                    ),
                    from: None,
                },
            ],
        }
    }

    fn default_remote_context() -> Option<RemoteContext> {
        Some(
            RemoteContext::try_new(
                Some("github.com".into()),
                Some("cocogitto".into()),
                Some("cocogitto".into()),
            )
            .unwrap(),
        )
    }
}
