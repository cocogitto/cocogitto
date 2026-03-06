use std::cmp::Ordering;
use std::fmt;
use std::fmt::Formatter;

use crate::error::TagError;
use crate::increment::Increment;
use cocogitto_settings::SETTINGS;
use git2::Oid;
use semver::{BuildMetadata, Prerelease, Version};

#[derive(Debug, Eq, Clone)]
pub struct Tag {
    pub package: Option<String>,
    pub prefix: Option<String>,
    pub version: Version,
    /// Oid of the commit pointed to by the tag
    pub oid: Option<Oid>,
}

impl Ord for Tag {
    fn cmp(&self, other: &Self) -> Ordering {
        self.version.cmp(&other.version)
    }
}

impl PartialEq for Tag {
    fn eq(&self, other: &Self) -> bool {
        self.package == other.package
            && self.version == other.version
            && self.prefix == other.prefix
    }
}

impl PartialOrd<Tag> for Tag {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Default for Tag {
    fn default() -> Self {
        Tag::create(Version::new(0, 0, 0), None)
    }
}

impl Tag {
    pub fn strip_metadata(&self) -> Self {
        let mut copy_without_prefix = self.clone();
        copy_without_prefix.package = None;
        copy_without_prefix.prefix = None;
        copy_without_prefix
    }

    pub fn reset_metadata(mut self) -> Self {
        self.version.build = BuildMetadata::EMPTY;
        self.version.pre = Prerelease::EMPTY;
        self.oid = None;
        self
    }

    // Tag always contains an oid unless it was created before the tag exist.
    // The only case where we do that is while creating the changelog during `cog bump`.
    // In this situation we need a tag to generate the changelog but this tag does not exist in the
    // repo yet.
    pub fn oid_unchecked(&self) -> &Oid {
        self.oid.as_ref().unwrap()
    }

    pub fn create(version: Version, package: Option<String>) -> Self {
        Tag {
            package,
            prefix: SETTINGS.tag_prefix.clone(),
            version,
            oid: None,
        }
    }

    pub fn oid(&self) -> Option<&Oid> {
        self.oid.as_ref()
    }

    pub fn from_str(raw: &str, oid: Option<Oid>) -> Result<Tag, TagError> {
        let prefix = SETTINGS.tag_prefix.as_ref();

        let package_tag: Option<Tag> = SETTINGS
            .monorepo
            .as_ref()
            .map(|m| m.packages.keys())
            .unwrap_or_default()
            .filter_map(|package_name| {
                raw.strip_prefix(package_name)
                    .zip(SETTINGS.monorepo_separator())
                    .and_then(|(remains, prefix)| remains.strip_prefix(prefix))
                    .map(|remains| {
                        SETTINGS
                            .tag_prefix
                            .as_ref()
                            .and_then(|prefix| remains.strip_prefix(prefix))
                            .unwrap_or(remains)
                    })
                    .and_then(|version| Version::parse(version).ok())
                    .map(|version| Tag {
                        package: Some(package_name.to_string()),
                        prefix: SETTINGS.tag_prefix.clone(),
                        version,
                        oid,
                    })
            })
            .next();

        if let Some(tag) = package_tag {
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

    pub fn is_zero(&self) -> bool {
        self.version == Version::new(0, 0, 0)
    }

    pub fn get_increment_from(&self, other: &Tag) -> Option<Increment> {
        if self.version.major > other.version.major {
            Some(Increment::Major)
        } else if self.version.minor > other.version.minor {
            Some(Increment::Minor)
        } else if self.version.patch > other.version.patch {
            Some(Increment::Patch)
        } else {
            None
        }
    }
}

impl fmt::Display for Tag {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        let version = self.version.to_string();
        if let Some((package, prefix)) = self.package.as_ref().zip(self.prefix.as_ref()) {
            let separator = SETTINGS.monorepo_separator().unwrap_or_else(||
                panic!("Found a tag with monorepo package prefix but there are no packages in cog.toml")
            );
            write!(f, "{package}{separator}{prefix}{version}")
        } else if let Some(package) = self.package.as_ref() {
            let separator = SETTINGS.monorepo_separator().unwrap_or_else(||
                panic!("Found a tag with monorepo package prefix but there are no packages in cog.toml")
            );

            write!(f, "{package}{separator}{version}")
        } else if let Some(prefix) = self.prefix.as_ref() {
            write!(f, "{prefix}{version}")
        } else {
            write!(f, "{version}")
        }
    }
}

#[cfg(test)]
mod test {
    use crate::tag::Tag;
    use anyhow::Result;
    use semver::Version;
    use speculoos::prelude::*;

    #[test]
    fn should_compare_tags() -> Result<()> {
        let v1_0_0 = Tag::from_str("1.0.0", None)?;
        let v1_1_0 = Tag::from_str("1.1.0", None)?;
        let v2_1_0 = Tag::from_str("2.1.0", None)?;
        let v0_1_0 = Tag::from_str("0.1.0", None)?;
        let v0_2_0 = Tag::from_str("0.2.0", None)?;
        let v0_0_1 = Tag::from_str("0.0.1", None)?;
        assert_that!([v1_0_0, v1_1_0, v2_1_0, v0_1_0, v0_2_0, v0_0_1,]
            .iter()
            .max())
        .is_some()
        .is_equal_to(&Tag::from_str("2.1.0", None)?);

        Ok(())
    }

    #[test]
    fn should_get_tag_from_str() -> Result<()> {
        let tag = Tag::from_str("1.0.0", None);
        assert_that!(tag).is_ok().is_equal_to(Tag {
            package: None,
            prefix: None,
            version: Version::new(1, 0, 0),
            oid: None,
        });

        Ok(())
    }
}
