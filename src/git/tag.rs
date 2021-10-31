use crate::git::repository::Repository;
use crate::{error::CocogittoError, SETTINGS};
use anyhow::{anyhow, ensure, Result};
use colored::Colorize;
use git2::string_array::StringArray;
use git2::Oid;
use semver::Version;
use std::cmp::Ordering;
use std::fmt;
use std::fmt::Formatter;

impl Repository {
    /// Given a tag name return a [`Tag`], this will fail if the requested
    /// tag (without configured prefix) is not semver compliant or if the tag
    /// does not exist.
    pub fn resolve_tag(&self, tag: &str) -> Result<Tag> {
        let without_prefix = match SETTINGS.tag_prefix.as_ref() {
            None => Ok(tag),
            Some(prefix) => tag
                .strip_prefix(prefix)
                .ok_or_else(|| anyhow!("Expected a tag with prefix {}, got {}", prefix, tag)),
        }?;

        // Ensure the tag is SemVer compliant
        Version::parse(without_prefix)?;

        self.resolve_lightweight_tag(tag)
    }

    /// Resolve a tag from a given `&str`, return an error if the tag is not found.
    pub fn resolve_lightweight_tag(&self, tag: &str) -> Result<Tag> {
        self.0
            .resolve_reference_from_short_name(tag)
            .map(|reference| reference.target().unwrap())
            .map(|oid| Tag::new(tag, oid))?
    }

    pub(crate) fn create_tag(&self, name: &str) -> Result<()> {
        ensure!(
            self.get_diff(true).is_none(),
            "{}{}",
            self.get_statuses()?,
            "Cannot create tag: changes need to be committed".red()
        );

        let head = self.get_head_commit().unwrap();
        self.0
            .tag_lightweight(name, &head.into_object(), false)
            .map(|_| ())
            .map_err(|err| {
                let cause_key = "cause:".red();
                anyhow!(CocogittoError::Git {
                    level: "Git error".to_string(),
                    cause: format!("{} {}", cause_key, err.message())
                })
            })
    }

    pub(crate) fn get_latest_tag(&self) -> Result<Tag> {
        let latest_tag: Option<Tag> = self
            .tags()?
            .iter()
            .flatten()
            .map(|tag| self.resolve_lightweight_tag(tag))
            .filter_map(Result::ok)
            .max();

        match latest_tag {
            Some(tag) => Ok(tag),
            None => Err(anyhow!("Unable to get any tag")),
        }
    }

    pub(crate) fn get_latest_tag_oid(&self) -> Result<Oid> {
        self.get_latest_tag()
            .map(|tag| tag.oid().to_owned())
            .map_err(|err| anyhow!("Could not resolve latest tag:{}", err))
    }

    fn tags(&self) -> Result<StringArray> {
        let pattern = SETTINGS
            .tag_prefix
            .as_ref()
            .map(|prefix| format!("{}*", prefix));

        self.0
            .tag_names(pattern.as_deref())
            .map_err(|err| anyhow!("{}", err))
    }
}

#[derive(Debug, PartialEq, Eq)]
pub struct Tag {
    tag: String,
    oid: Oid,
}

impl Tag {
    pub(crate) fn oid(&self) -> &Oid {
        &self.oid
    }

    pub(crate) fn new(name: &str, oid: Oid) -> Result<Tag> {
        let tag = match SETTINGS.tag_prefix.as_ref() {
            None => Ok(name),
            Some(prefix) => name
                .strip_prefix(prefix)
                .ok_or_else(|| anyhow!("Expected a tag with prefix {}, got {}", prefix, name)),
        }?
        .to_string();

        Ok(Tag { tag, oid })
    }

    pub(crate) fn to_version(&self) -> Result<Version> {
        Version::parse(&self.tag).map_err(|err| anyhow!("{}", err))
    }

    pub(crate) fn to_string_with_prefix(&self) -> String {
        match SETTINGS.tag_prefix.as_ref() {
            None => self.tag.to_string(),
            Some(prefix) => format!("{}{}", prefix, self.tag),
        }
    }
}

impl fmt::Display for Tag {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.to_string_with_prefix())
    }
}

impl Ord for Tag {
    fn cmp(&self, other: &Self) -> Ordering {
        self.partial_cmp(other).unwrap()
    }
}

impl PartialOrd<Tag> for Tag {
    fn partial_cmp(&self, other: &Tag) -> Option<Ordering> {
        Some(self.to_version().ok().cmp(&other.to_version().ok()))
    }
}

#[cfg(test)]
mod test {
    use crate::git::repository::Repository;
    use crate::test_helpers::run_test_with_context;
    use anyhow::Result;
    use speculoos::prelude::*;

    #[test]
    fn resolve_lightweight_tag_ok() -> Result<()> {
        run_test_with_context(|context| {
            let repo = Repository::init(&context.test_dir)?;
            std::fs::write(context.test_dir.join("file"), "changes")?;
            repo.add_all()?;
            repo.commit("first commit")?;
            repo.create_tag("the_tag")?;

            let tag = repo.resolve_lightweight_tag("the_tag");

            assert_that!(tag).is_ok();
            Ok(())
        })
    }

    #[test]
    fn resolve_lightweight_tag_err() -> Result<()> {
        run_test_with_context(|context| {
            let repo = Repository::init(&context.test_dir)?;
            std::fs::write(context.test_dir.join("file"), "changes")?;
            repo.add_all()?;
            repo.commit("first commit")?;
            repo.create_tag("the_tag")?;

            let tag = repo.resolve_lightweight_tag("the_taaaag");

            assert_that!(tag).is_err();
            Ok(())
        })
    }

    #[test]
    fn get_latest_tag_ok() -> Result<()> {
        run_test_with_context(|context| {
            let repo = Repository::init(&context.test_dir)?;
            std::fs::write(&context.test_dir.join("file"), "changes")?;
            repo.add_all()?;
            repo.commit("first commit")?;
            repo.create_tag("0.1.0")?;

            std::fs::write(&context.test_dir.join("file"), "changes2")?;
            repo.add_all()?;
            repo.commit("second commit")?;
            repo.create_tag("0.2.0")?;

            let tag = repo.get_latest_tag()?;

            assert_that!(tag.to_string_with_prefix()).is_equal_to("0.2.0".to_string());
            Ok(())
        })
    }

    #[test]
    fn get_latest_tag_err() -> Result<()> {
        run_test_with_context(|context| {
            let repo = Repository::init(&context.test_dir)?;
            std::fs::write(&context.test_dir.join("file"), "changes")?;
            repo.add_all()?;
            repo.commit("first commit")?;

            let tag = repo.get_latest_tag();

            assert_that!(tag).is_err();
            Ok(())
        })
    }

    #[test]
    fn get_latest_tag_oid_ok() -> Result<()> {
        run_test_with_context(|context| {
            let repo = Repository::init(&context.test_dir)?;
            std::fs::write(&context.test_dir.join("file"), "changes")?;
            repo.add_all()?;
            repo.commit("first commit")?;
            repo.create_tag("1.0.0")?;

            let tag = repo.get_latest_tag_oid();

            assert_that!(tag).is_ok();
            Ok(())
        })
    }

    #[test]
    fn get_latest_tag_oid_err() -> Result<()> {
        run_test_with_context(|context| {
            let repo = Repository::init(&context.test_dir)?;
            std::fs::write(&context.test_dir.join("file"), "changes")?;
            repo.add_all()?;
            repo.commit("first commit")?;

            let tag = repo.get_latest_tag_oid();

            assert_that!(tag).is_err();
            Ok(())
        })
    }
}
