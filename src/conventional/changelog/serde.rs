use serde::ser::SerializeStruct;
use serde::{Serialize, Serializer};

use crate::conventional::changelog::release::{ChangelogCommit, ChangelogFooter};
use crate::git::oid::OidOf;
use crate::git::tag::Tag;
use crate::COMMITS_METADATA;

impl Serialize for Tag {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_str(&self.to_string())
    }
}

impl Serialize for ChangelogCommit<'_> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut commit = serializer.serialize_struct("Commit", 10)?;

        let footers = &self
            .commit
            .conventional
            .footers
            .iter()
            .map(ChangelogFooter::from)
            .collect::<Vec<ChangelogFooter>>();

        let commit_type = &COMMITS_METADATA
            .iter()
            .find(|(commit_type, _config)| *commit_type == &self.commit.conventional.commit_type)
            .map(|meta| meta.1.changelog_title.clone())
            .unwrap_or_else(|| self.commit.conventional.commit_type.to_string());

        commit.serialize_field("id", &self.commit.oid)?;
        commit.serialize_field("author", &self.author_username)?;
        commit.serialize_field("signature", &self.commit.author)?;
        commit.serialize_field("type", commit_type)?;
        commit.serialize_field("date", &self.commit.date)?;
        commit.serialize_field("scope", &self.commit.conventional.scope)?;
        commit.serialize_field("summary", &self.commit.conventional.summary)?;
        commit.serialize_field("body", &self.commit.conventional.body)?;
        commit.serialize_field(
            "breaking_change",
            &self.commit.conventional.is_breaking_change,
        )?;
        commit.serialize_field("footer", footers)?;
        commit.end()
    }
}

impl Serialize for OidOf {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut oidof = serializer.serialize_struct("OidOf", 1)?;
        match self {
            OidOf::Tag(tag) => {
                oidof.serialize_field("tag", &tag.to_string())?;
                if let Some(oid) = tag.oid() {
                    oidof.serialize_field("id", &oid.to_string())?;
                }
            }
            OidOf::Head(oid) | OidOf::Other(oid) => {
                oidof.serialize_field("id", &oid.to_string())?
            }
        };
        oidof.end()
    }
}

#[cfg(test)]
mod test {
    use chrono::Utc;
    use conventional_commit_parser::commit::{CommitType, ConventionalCommit, Footer};
    use git2::Oid;
    use speculoos::prelude::*;

    use crate::conventional::changelog::release::ChangelogCommit;
    use crate::conventional::commit::Commit;
    use crate::git::tag::Tag;

    #[test]
    fn should_serialize_tag() {
        let tag = Tag::from_str("1.0.0", Some(Oid::from_str("1234567890").unwrap())).unwrap();

        let result = toml::to_string(&tag);

        assert_that!(result)
            .is_ok()
            .is_equal_to("\"1.0.0\"".to_string())
    }

    #[test]
    fn should_serialize_commit() {
        let commit = ChangelogCommit {
            author_username: Some("Jm Doudou"),
            commit: Commit {
                oid: "1234567890".to_string(),
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
                author: "Jean Michel Doudou".to_string(),
                date: Utc::now().naive_utc(),
            },
        };

        let result = toml::to_string(&commit);

        assert_that!(result).is_ok();
    }
}
