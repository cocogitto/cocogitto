use crate::git::error::{Git2Error, TagError};
use crate::git::repository::Repository;
use crate::SETTINGS;
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
    pub fn resolve_tag(&self, tag: &str) -> Result<Tag, TagError> {
        self.resolve_lightweight_tag(tag)
    }

    /// Resolve a tag from a given `&str`, return an error if the tag is not found.
    fn resolve_lightweight_tag(&self, tag: &str) -> Result<Tag, TagError> {
        self.0
            .resolve_reference_from_short_name(tag)
            .map_err(|err| TagError::not_found(tag, err))
            .map(|reference| reference.target().unwrap())
            .map(|oid| Tag::from_str(tag, Some(oid)))?
    }

    pub(crate) fn create_tag(&self, tag: &Tag) -> Result<(), Git2Error> {
        if self.get_diff(true).is_some() {
            let statuses = self.get_statuses()?;
            return Err(Git2Error::ChangesNeedToBeCommitted(statuses));
        }

        let head = self.get_head_commit().unwrap();
        self.0
            .tag_lightweight(&tag.to_string(), &head.into_object(), false)
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
    pub package: Option<String>,
    pub prefix: Option<String>,
    pub version: Version,
    pub oid: Option<Oid>,
}

impl Tag {
    // Tag always contains an oid unless it was created before the tag exist.
    // The only case where we do that is while creating the changelog during `cog bump`.
    // In this situation we need a tag to generate the changelog but this tag does not exist in the
    // repo yet.
    pub(crate) fn oid_unchecked(&self) -> &Oid {
        self.oid.as_ref().unwrap()
    }

    pub(crate) fn create(version: Version, package: Option<String>) -> Self {
        Tag {
            package,
            prefix: SETTINGS.tag_prefix.clone(),
            version,
            oid: None,
        }
    }
    pub(crate) fn oid(&self) -> Option<&Oid> {
        self.oid.as_ref()
    }

    pub(crate) fn from_str(raw: &str, oid: Option<Oid>) -> Result<Tag, TagError> {
        let prefix = SETTINGS.tag_prefix.as_ref();

        let tag = SETTINGS
            .monorepo_version_separator
            .as_ref()
            .and_then(|separator| raw.split_once(separator))
            .map(|(package, remains)| {
                let version = prefix
                    .and_then(|prefix| remains.strip_prefix(prefix))
                    .unwrap_or(remains);

                if SETTINGS.packages.keys().any(|name| name == package) {
                    Version::parse(version)
                        .map(|version| Tag {
                            package: Some(package.to_string()),
                            prefix: prefix.cloned(),
                            version,
                            oid,
                        })
                        .map_err(|err| TagError::semver(raw, err))
                } else {
                    Err(TagError::InvalidPrefixError {
                        prefix: package.to_string(),
                        tag: raw.to_string(),
                    })
                }
            });

        if let Some(Ok(tag)) = tag {
            Ok(tag)
        } else {
            let version = prefix
                .and_then(|prefix| raw.strip_prefix(prefix))
                .unwrap_or(raw);

            let version = Version::parse(version).map_err(|err| TagError::semver(raw, err))?;

            Ok(Tag {
                package: None,
                prefix: prefix.cloned(),
                version,
                oid,
            })
        }
    }
}

impl fmt::Display for Tag {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        let version = self.version.to_string();
        if let Some((prefix, package)) = self.package.as_ref().zip(self.prefix.as_ref()) {
            let separator = &SETTINGS.monorepo_version_separator.as_ref().unwrap_or_else(||
                panic!("Found a tag with monorepo package prefix but 'monorepo_version_separator' is not defined")
            );

            write!(f, "{package}{separator}{prefix}{version}")
        } else if let Some(prefix) = self.prefix.as_ref() {
            write!(f, "{prefix}{version}")
        } else {
            write!(f, "{version}")
        }
    }
}

impl Ord for Tag {
    fn cmp(&self, other: &Self) -> Ordering {
        self.partial_cmp(other).unwrap()
    }
}

impl PartialOrd<Tag> for Tag {
    fn partial_cmp(&self, other: &Tag) -> Option<Ordering> {
        Some(self.version.cmp(&other.version))
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
            git tag 1.0.0;
        )?;

        // Act
        let tag = repo.resolve_lightweight_tag("1.0.0");

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
        assert_that!(tag.to_string()).is_equal_to("0.2.0".to_string());
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
