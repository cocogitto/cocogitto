use crate::git::error::{Git2Error, TagError};
use crate::git::repository::Repository;
use crate::SETTINGS;
use git2::string_array::StringArray;
use git2::Oid;
use git2::Tag as Git2Tag;
use semver::Version;
use std::cmp::Ordering;
use std::convert::TryFrom;
use std::fmt;
use std::fmt::Formatter;

impl Repository {
    /// Given a tag name return a [`Tag`], this will fail if the requested
    /// tag (without configured prefix) is not semver compliant or if the tag
    /// does not exist.
    pub fn resolve_tag(&self, tag: &str) -> Result<Tag, TagError> {
        let without_prefix = Tag::strip_default_prefix(tag)?;

        // Ensure the tag is SemVer compliant
        Version::parse(without_prefix).map_err(|err| TagError::semver(without_prefix, err))?;

        self.resolve_lightweight_tag(tag)
    }

    /// Resolve a tag from a given `&str`, return an error if the tag is not found.
    fn resolve_lightweight_tag(&self, tag: &str) -> Result<Tag, TagError> {
        self.0
            .resolve_reference_from_short_name(tag)
            .map_err(|err| TagError::not_found(tag, err))
            .map(|reference| reference.target().unwrap())
            .map(|oid| Tag::new(tag, Some(oid)))?
    }

    pub(crate) fn create_tag(&self, name: &str) -> Result<(), Git2Error> {
        if self.get_diff(true).is_some() {
            let statuses = self.get_statuses()?;
            return Err(Git2Error::ChangesNeedToBeCommitted(statuses));
        }

        let head = self.get_head_commit().unwrap();
        self.0
            .tag_lightweight(name, &head.into_object(), false)
            .map(|_| ())
            .map_err(Git2Error::from)
    }

    pub(crate) fn get_latest_tag(&self) -> Result<Tag, TagError> {
        let tags: Vec<Tag> = self.all_tags()?;

        tags.into_iter().max().ok_or(TagError::NoTag)
    }

    pub(crate) fn all_tags(&self) -> Result<Vec<Tag>, TagError> {
        Ok(self
            .tags()?
            .iter()
            .flatten()
            .map(|tag| self.resolve_lightweight_tag(tag))
            .filter_map(Result::ok)
            .collect())
    }

    pub(crate) fn get_latest_tag_oid(&self) -> Result<Oid, TagError> {
        self.get_latest_tag()
            .map(|tag| tag.oid_unchecked().to_owned())
    }

    fn tags(&self) -> Result<StringArray, TagError> {
        let pattern = SETTINGS
            .tag_prefix
            .as_ref()
            .map(|prefix| format!("{}*", prefix));

        self.0
            .tag_names(pattern.as_deref())
            .map_err(|err| TagError::NoMatchFound { pattern, err })
    }
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct Tag {
    tag: String,
    oid: Option<Oid>,
}

impl TryFrom<Git2Tag<'_>> for Tag {
    type Error = TagError;

    fn try_from(tag: Git2Tag) -> Result<Self, Self::Error> {
        let name = tag.name().expect("Unexpected unnamed tag");
        Self::new(name, Some(tag.id()))
    }
}

impl Tag {
    // Tag always contains an oid unless it was created before the tag exist.
    // The only case where we do that is while creating the changelog during `cog bump`.
    // In this situation we need a tag to generate the changelog but this tag does not exist in the
    // repo yet.
    pub(crate) fn oid_unchecked(&self) -> &Oid {
        self.oid.as_ref().unwrap()
    }

    pub(crate) fn oid(&self) -> Option<&Oid> {
        self.oid.as_ref()
    }

    pub(crate) fn new(name: &str, oid: Option<Oid>) -> Result<Tag, TagError> {
        let tag = Tag::strip_default_prefix(name)?.to_string();
        Ok(Tag { tag, oid })
    }

    pub(crate) fn to_string_with_prefix(&self) -> String {
        match SETTINGS.tag_prefix.as_ref() {
            None => self.tag.to_string(),
            Some(prefix) => format!("{}{}", prefix, self.tag),
        }
    }

    pub(crate) fn to_version(&self) -> Result<Version, TagError> {
        Version::parse(&self.tag).map_err(|err| TagError::semver(&self.tag, err))
    }

    pub(crate) fn to_package_version(&self, package_name: &str) -> Result<Version, TagError> {
        let prefix = format!("{package_name}-");
        let version = self.tag.strip_prefix(&prefix).ok_or(TagError::InvalidPrefixError {
            prefix,
            tag: self.tag.clone(),
        })?;

        Version::parse(version).map_err(|err| TagError::semver(&self.tag, err))
    }

    fn strip_default_prefix(tag: &str) -> Result<&str, TagError> {
        match SETTINGS.tag_prefix.as_ref() {
            None => Ok(tag),
            Some(prefix) => tag
                .strip_prefix(prefix)
                .ok_or(TagError::InvalidPrefixError {
                    prefix: prefix.to_string(),
                    tag: tag.to_string(),
                }),
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
    use anyhow::Result;
    use cmd_lib::run_cmd;
    use sealed_test::prelude::*;
    use speculoos::prelude::*;

    #[sealed_test]
    fn resolve_lightweight_tag_ok() -> Result<()> {
        // Arrange
        let repo = Repository::init(".")?;
        run_cmd!(
            git commit --allow-empty -m "first commit";
            git tag the_tag;
        )?;

        // Act
        let tag = repo.resolve_lightweight_tag("the_tag");

        // Assert
        assert_that!(tag).is_ok();
        Ok(())
    }

    #[sealed_test]
    fn resolve_lightweight_tag_err() -> Result<()> {
        // Arrange
        let repo = Repository::init(".")?;
        run_cmd!(
            git commit --allow-empty -m "first commit";
            git tag the_tag;
        )?;

        // Act
        let tag = repo.resolve_lightweight_tag("the_taaaag");

        // Assert
        assert_that!(tag).is_err();
        Ok(())
    }

    #[sealed_test]
    fn get_latest_tag_ok() -> Result<()> {
        // Arrange
        let repo = Repository::init(".")?;
        run_cmd!(
            git commit --allow-empty -m "first commit";
            git tag 0.1.0;
            git commit --allow-empty -m "second commit";
            git tag 0.2.0;
        )?;

        // Act
        let tag = repo.get_latest_tag()?;

        // Assert
        assert_that!(tag.to_string_with_prefix()).is_equal_to("0.2.0".to_string());
        Ok(())
    }

    #[sealed_test]
    fn get_latest_tag_err() -> Result<()> {
        // Arrange
        let repo = Repository::init(".")?;
        run_cmd!(
            git commit --allow-empty -m "first commit"
        )?;

        // Act
        let tag = repo.get_latest_tag();

        // Assert
        assert_that!(tag).is_err();
        Ok(())
    }

    #[sealed_test]
    fn get_latest_tag_oid_ok() -> Result<()> {
        // Arrange
        let repo = Repository::init(".")?;
        run_cmd!(
            git commit --allow-empty -m "first commit";
            git tag 0.1.0;
        )?;

        // Act
        let tag = repo.get_latest_tag_oid();

        // Assert
        assert_that!(tag).is_ok();
        Ok(())
    }

    #[sealed_test]
    fn get_latest_tag_oid_err() -> Result<()> {
        // Arrange
        let repo = Repository::init(".")?;
        run_cmd!(git commit --allow-empty -m "first commit")?;

        // Act
        let tag = repo.get_latest_tag_oid();

        // Assert
        assert_that!(tag).is_err();
        Ok(())
    }
}
