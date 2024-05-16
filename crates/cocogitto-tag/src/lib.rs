use crate::error::TagError;
use crate::increment::Increment;
use git2::Oid;
use semver::{BuildMetadata, Prerelease, Version};
use serde::{Serialize, Serializer};
use std::cmp::Ordering;
use std::fmt;
use std::fmt::Formatter;

pub mod error;
pub mod increment;

#[derive(Debug, Eq, Clone)]
pub struct Tag {
    pub package: Option<String>,
    pub prefix: Option<&'static str>,
    pub monorepo_separator: Option<&'static str>,
    pub version: Version,
    pub oid: Option<Oid>,
    pub target: Option<Oid>,
}

impl Tag {
    pub fn manual_bump(&self, version: &str) -> Result<Self, semver::Error> {
        let mut next = self.clone();
        next.version = Version::parse(version)?;
        Ok(next)
    }

    pub fn major_bump(&self) -> Self {
        let mut next = self.clone();
        next.version.major += 1;
        next.version.minor = 0;
        next.version.patch = 0;
        next.reset_metadata()
    }

    pub fn minor_bump(&self) -> Self {
        let mut next = self.clone();
        next.version.minor += 1;
        next.version.patch = 0;
        next.reset_metadata()
    }

    pub fn patch_bump(&self) -> Self {
        let mut next = self.clone();
        next.version.patch += 1;
        next.reset_metadata()
    }

    pub fn no_bump(&self) -> Self {
        let next = self.clone();
        next.reset_metadata()
    }

    fn reset_metadata(mut self) -> Self {
        self.version.build = BuildMetadata::EMPTY;
        self.version.pre = Prerelease::EMPTY;
        self.oid = None;
        self
    }

    pub fn strip_metadata(&self) -> Self {
        let mut copy_without_prefix = self.clone();
        copy_without_prefix.package = None;
        copy_without_prefix.prefix = None;
        copy_without_prefix
    }

    // Tag always contains an oid unless it was created before the tag exist.
    // The only case where we do that is while creating the changelog during `cog bump`.
    // In this situation we need a tag to generate the changelog but this tag does not exist in the
    // repo yet.
    pub fn oid_unchecked(&self) -> &Oid {
        self.oid.as_ref().unwrap()
    }

    pub fn create_default(
        prefix: Option<&'static str>,
        monorepo_separator: Option<&'static str>,
    ) -> Self {
        Tag::create(Version::new(0, 0, 0), None, prefix, monorepo_separator)
    }

    pub fn create(
        version: Version,
        package: Option<String>,
        prefix: Option<&'static str>,
        monorepo_separator: Option<&'static str>,
    ) -> Self {
        Tag {
            package,
            prefix,
            monorepo_separator,
            version,
            oid: None,
            target: None,
        }
    }

    pub fn oid(&self) -> Option<&Oid> {
        self.oid.as_ref()
    }

    pub fn from_str(
        raw: &str,
        oid: Option<Oid>,
        target: Option<Oid>,
        prefix: Option<&'static str>,
        monorepo_separator: Option<&'static str>,
        packages: impl Iterator<Item = &'static str>,
    ) -> Result<Tag, TagError> {
        let package_tag: Option<Tag> = packages
            .filter_map(|package_name| {
                raw.strip_prefix(package_name)
                    .zip(monorepo_separator)
                    .and_then(|(remains, prefix)| remains.strip_prefix(prefix))
                    .map(|remains| {
                        prefix
                            .and_then(|prefix| remains.strip_prefix(prefix))
                            .unwrap_or(remains)
                    })
                    .and_then(|version| Version::parse(version).ok())
                    .map(|version| Tag {
                        package: Some(package_name.to_string()),
                        prefix,
                        monorepo_separator,
                        version,
                        oid,
                        target,
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
                prefix,
                monorepo_separator: None,
                version,
                oid,
                target,
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

impl fmt::Display for Tag {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        let version = self.version.to_string();
        if let Some((package, prefix)) = self.package.as_ref().zip(self.prefix.as_ref()) {
            let separator = self.monorepo_separator.unwrap_or_else(||
                panic!("Found a tag with monorepo package prefix but there are no packages in cog.toml")
            );
            write!(f, "{package}{separator}{prefix}{version}")
        } else if let Some(package) = self.package.as_ref() {
            let separator = self.monorepo_separator.unwrap_or_else(||
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

impl Serialize for Tag {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_str(&self.to_string())
    }
}
