use crate::conventional::changelog::context::{MonoRepoContext, PackageBumpContext, RemoteContext};
use crate::conventional::changelog::release::{ChangelogCommit, Release};
use crate::conventional::commit::Commit;
use crate::git::oid::OidOf;
use crate::git::tag::Tag;
use chrono::NaiveDateTime;
use conventional_commit_parser::commit::{CommitType, ConventionalCommit, Footer, Separator};
use git2::Oid;

pub struct ReleaseFixture {
    pub release: Release,
}

impl<'a> ReleaseFixture {
    pub fn builder() -> ReleaseFixture {
        ReleaseFixture {
            release: Release {
                version: OidOf::Tag(
                    Tag::from_str(
                        "1.0.0",
                        Some(Oid::from_str("9bb5facac5724bc81385fdd740fedbb49056da00").unwrap()),
                    )
                    .unwrap(),
                ),
                from: OidOf::Tag(
                    Tag::from_str(
                        "0.1.0",
                        Some(Oid::from_str("fae3a288a1bc69b14f85a1d5fe57cee1964acd60").unwrap()),
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

    pub fn build(self) -> Release {
        self.release
    }

    pub fn with_commit(mut self, commit: CommitFixture) -> Self {
        self.release.commits.push(commit.build());
        self
    }
}

impl Default for ReleaseFixture {
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

impl ReleaseFixture {
    pub fn cocogitto() -> Self {
        return ReleaseFixture::builder()
            .with_commit(
                CommitFixture::default()
                    .with_scope("parser")
                    .with_author("Paul Delafosse")
                    .with_commit_type(CommitType::Feature)
                    .with_sha("9d14c0b967598780d2acd9e281bcf2ee4d0e9fd7")
                    .with_footer(
                        "Co-authored-by",
                        "dependabot[bot] <49699333+dependabot[bot]@users.noreply.github.com>",
                        Separator::Colon,
                    )
                    .with_message("implement the changelog generator"),
            )
            .with_commit(
                CommitFixture::default()
                    .with_sha("cc0e64d2c1e075ac9b782258783212b4d7917892")
                    .with_author("Lindner, Bernhard")
                    .with_message("awesome feature")
                    .with_footer(
                        "Co-authored-by",
                        "Paul Delafosse <paul.delafosse@protonmail.com>",
                        Separator::Colon,
                    )
                    .with_commit_type(CommitType::Feature),
            );
    }
}

pub struct CommitFixture {
    changelog: ChangelogCommit,
}

impl CommitFixture {
    pub fn with_sha(mut self, sha: &str) -> Self {
        self.changelog.commit.oid = sha.to_string();
        self
    }

    pub fn with_commit_type(mut self, commit_type: CommitType) -> Self {
        self.changelog.commit.conventional.commit_type = commit_type;
        self
    }

    pub fn with_author(mut self, author: &str) -> Self {
        self.changelog.commit.author = author.to_string();
        self
    }

    pub fn with_username(mut self, author: &str) -> Self {
        self.changelog.author_username = Some(author.to_string());
        self
    }

    pub fn with_scope(mut self, scope: &str) -> Self {
        self.changelog.commit.conventional.scope = Some(scope.to_string());
        self
    }

    pub fn with_message(mut self, message: &str) -> Self {
        self.changelog.commit.conventional.summary = message.to_string();
        self
    }

    pub fn with_footer(mut self, token: &str, content: &str, token_separator: Separator) -> Self {
        self.changelog.commit.conventional.footers.push(Footer {
            token: token.to_string(),
            content: content.to_string(),
            token_separator,
        });
        self
    }

    pub fn with_breaking(mut self) -> Self {
        self.changelog.commit.conventional.is_breaking_change = true;
        self
    }

    pub fn build(self) -> ChangelogCommit {
        self.changelog
    }
}

impl Default for CommitFixture {
    fn default() -> Self {
        Self {
            changelog: ChangelogCommit {
                committer_username: None,
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
                    committer: "Paul Delafosse".to_string(),
                    date: NaiveDateTime::parse_from_str("2015-09-05 23:56:04", "%Y-%m-%d %H:%M:%S")
                        .unwrap(),
                },
                co_authors: vec![],
                github_closes_numbers: vec![],
            },
        }
    }
}

pub fn monorepo_context<'a>() -> MonoRepoContext<'a> {
    MonoRepoContext {
        package_lock: false,
        packages: vec![
            PackageBumpContext {
                package_name: "one",
                package_path: "crates/one",
                version: OidOf::Tag(
                    Tag::from_str(
                        "0.1.0",
                        Some(Oid::from_str("fae3a288a1bc69b14f85a1d5fe57cee1964acd60").unwrap()),
                    )
                    .unwrap(),
                ),
                from: Some(OidOf::Tag(
                    Tag::from_str(
                        "0.2.0",
                        Some(Oid::from_str("fae3a288a1bc69b14f85a1d5fe57cee1964acd60").unwrap()),
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
                        Some(Oid::from_str("fae3a288a1bc69b14f85a1d5fe57cee1964acd60").unwrap()),
                    )
                    .unwrap(),
                ),
                from: Some(OidOf::Tag(
                    Tag::from_str(
                        "0.3.0",
                        Some(Oid::from_str("fae3a288a1bc69b14f85a1d5fe57cee1964acd60").unwrap()),
                    )
                    .unwrap(),
                )),
            },
        ],
    }
}

pub fn default_package_context<'a>() -> MonoRepoContext<'a> {
    MonoRepoContext {
        package_lock: true,
        packages: vec![
            PackageBumpContext {
                package_name: "one",
                package_path: "crates/one",
                version: OidOf::Tag(
                    Tag::from_str(
                        "0.1.0",
                        Some(Oid::from_str("fae3a288a1bc69b14f85a1d5fe57cee1964acd60").unwrap()),
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
                        Some(Oid::from_str("fae3a288a1bc69b14f85a1d5fe57cee1964acd60").unwrap()),
                    )
                    .unwrap(),
                ),
                from: None,
            },
        ],
    }
}

pub fn default_remote_context() -> Option<RemoteContext> {
    Some(
        RemoteContext::try_new(
            Some("github.com".into()),
            Some("cocogitto".into()),
            Some("cocogitto".into()),
            None,
        )
        .unwrap(),
    )
}

pub fn remote_context_with_github_provider() -> Option<RemoteContext> {
    Some(
        RemoteContext::try_new(
            Some("github.com".into()),
            Some("cocogitto".into()),
            Some("cocogitto".into()),
            None,
        )
        .unwrap(),
    )
}
