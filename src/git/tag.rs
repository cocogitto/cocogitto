use crate::conventional::version::Increment;
use crate::git::error::{Git2Error, TagError};
use crate::git::repository::Repository;
use crate::SETTINGS;
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

    pub(crate) fn create_annotated_tag(&self, tag: &Tag, msg: &str) -> Result<(), Git2Error> {
        if self.get_diff(true).is_some() {
            let statuses = self.get_statuses()?;
            return Err(Git2Error::ChangesNeedToBeCommitted(statuses));
        }

        let head = self.get_head_commit().unwrap();
        let sig = self.0.signature()?;
        self.0
            .tag(&tag.to_string(), &head.into_object(), &sig, msg, false)
            .map(|_| ())
            .map_err(Git2Error::from)
    }

    /// Get the latest tag, will ignore package tag if on a monorepo
    pub(crate) fn get_latest_tag(&self) -> Result<Tag, TagError> {
        let tags: Vec<Tag> = self.all_tags()?;
        tags.into_iter()
            .filter(|tag| tag.package.is_none())
            .filter(|tag| tag.version.pre.is_empty())
            .max()
            .ok_or(TagError::NoTag)
    }

    pub(crate) fn all_tags(&self) -> Result<Vec<Tag>, TagError> {
        Ok(self
            .tags()?
            .iter()
            .map(|tag| self.resolve_lightweight_tag(tag))
            .filter_map(Result::ok)
            .collect())
    }

    pub(crate) fn get_latest_tag_oid(&self) -> Result<Oid, TagError> {
        self.get_latest_tag()
            .map(|tag| tag.oid_unchecked().to_owned())
    }

    fn tags(&self) -> Result<Vec<String>, TagError> {
        let packages: Vec<&str> = SETTINGS
            .packages
            .keys()
            .map(|profile| -> &str { profile })
            .collect();

        let pattern = SETTINGS
            .tag_prefix
            .as_ref()
            .map(|prefix| format!("{prefix}*"));

        // Collect non packages tags
        let mut tags: Vec<String> = self
            .0
            .tag_names(pattern.as_deref())
            .map_err(|err| TagError::NoMatchFound { pattern, err })?
            .iter()
            .flatten()
            .map(str::to_string)
            .collect();

        // Extends with packages tags if we are in a mono-repository context
        if !packages.is_empty() {
            let package_tags = self.0.tag_names(None).map_err(|_| TagError::NoTag)?;
            let package_tags = package_tags
                .into_iter()
                .flatten()
                .filter(|tag| packages.iter().any(|package| tag.starts_with(package)))
                .map(str::to_string);

            tags.extend(package_tags);
        };

        Ok(tags)
    }
}

#[derive(Debug, Eq, Clone)]
pub struct Tag {
    pub package: Option<String>,
    pub prefix: Option<String>,
    pub version: Version,
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
    pub(crate) fn strip_metadata(&self) -> Self {
        let mut copy_without_prefix = self.clone();
        copy_without_prefix.package = None;
        copy_without_prefix.prefix = None;
        copy_without_prefix
    }

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

    pub fn from_str(raw: &str, oid: Option<Oid>) -> Result<Tag, TagError> {
        let prefix = SETTINGS.tag_prefix.as_ref();

        let package_tag: Option<Tag> = SETTINGS
            .packages
            .keys()
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

    pub(crate) fn is_zero(&self) -> bool {
        self.version == Version::new(0, 0, 0)
    }

    pub(crate) fn get_increment_from(&self, other: &Tag) -> Option<Increment> {
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
    use crate::git::repository::Repository;
    use crate::git::tag::Tag;
    use crate::settings::{MonoRepoPackage, Settings};
    use anyhow::Result;
    use cmd_lib::run_cmd;
    use sealed_test::prelude::*;
    use semver::Version;
    use speculoos::prelude::*;
    use std::collections::HashMap;
    use std::fs;
    use std::path::PathBuf;

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

    #[sealed_test]
    fn should_compare_tags_with_prefix() -> Result<()> {
        Repository::init(".")?;
        let settings = Settings {
            tag_prefix: Some("v".to_string()),
            ..Default::default()
        };
        let settings = toml::to_string(&settings)?;
        fs::write("cog.toml", settings)?;

        let v1_0_0 = Tag::from_str("v1.0.0", None)?;
        let v1_1_0 = Tag::from_str("v1.1.0", None)?;
        let v2_1_0 = Tag::from_str("v2.1.0", None)?;
        let v0_1_0 = Tag::from_str("v0.1.0", None)?;
        let v0_2_0 = Tag::from_str("v0.2.0", None)?;
        let v0_0_1 = Tag::from_str("v0.0.1", None)?;
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

    #[sealed_test]
    fn should_get_tag_from_str_with_prefix() -> Result<()> {
        Repository::init(".")?;

        let settings = Settings {
            tag_prefix: Some("v".to_string()),
            ..Default::default()
        };

        let settings = toml::to_string(&settings)?;
        fs::write("cog.toml", settings)?;

        let tag = Tag::from_str("v1.0.0", None);

        assert_that!(tag).is_ok().is_equal_to(Tag {
            package: None,
            prefix: Some("v".to_string()),
            version: Version::new(1, 0, 0),
            oid: None,
        });

        Ok(())
    }

    #[sealed_test]
    fn should_get_tag_from_str_with_separator() -> Result<()> {
        Repository::init(".")?;

        let mut packages = HashMap::new();
        packages.insert("one".to_string(), Default::default());
        let settings = Settings {
            packages,
            ..Default::default()
        };

        let settings = toml::to_string(&settings)?;
        fs::write("cog.toml", settings)?;

        let tag = Tag::from_str("one-1.0.0", None);

        assert_that!(tag).is_ok().is_equal_to(Tag {
            package: Some("one".to_string()),
            prefix: None,
            version: Version::new(1, 0, 0),
            oid: None,
        });

        Ok(())
    }

    #[sealed_test]
    fn should_get_tag_from_str_with_prefix_and_separator() -> Result<()> {
        Repository::init(".")?;

        let mut packages = HashMap::new();
        packages.insert("one".to_string(), Default::default());
        let settings = Settings {
            tag_prefix: Some("v".to_string()),
            packages,
            ..Default::default()
        };

        let settings = toml::to_string(&settings)?;
        fs::write("cog.toml", settings)?;

        let tag = Tag::from_str("one-v1.0.0", None);

        assert_that!(tag).is_ok().is_equal_to(Tag {
            package: Some("one".to_string()),
            prefix: Some("v".to_string()),
            version: Version::new(1, 0, 0),
            oid: None,
        });

        Ok(())
    }

    #[sealed_test]
    fn should_get_tag_from_str_with_prefix_and_custom_separator() -> Result<()> {
        Repository::init(".")?;

        let mut packages = HashMap::new();
        packages.insert("one".to_string(), Default::default());
        let settings = Settings {
            tag_prefix: Some("v".to_string()),
            monorepo_version_separator: Some("#".to_string()),
            packages,
            ..Default::default()
        };

        let settings = toml::to_string(&settings)?;
        fs::write("cog.toml", settings)?;

        let tag = Tag::from_str("one#v1.0.0", None);

        assert_that!(tag).is_ok().is_equal_to(Tag {
            package: Some("one".to_string()),
            prefix: Some("v".to_string()),
            version: Version::new(1, 0, 0),
            oid: None,
        });

        Ok(())
    }

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

    #[sealed_test]
    fn get_latest_package_tag() -> Result<()> {
        // Arrange
        let mut packages = HashMap::new();
        packages.insert(
            "lunatic-timer-api".to_string(),
            MonoRepoPackage {
                path: PathBuf::from("lunatic-timer-api"),
                ..Default::default()
            },
        );

        let settings = Settings {
            from_latest_tag: true,
            ignore_merge_commits: true,
            tag_prefix: Some("v".to_string()),
            packages,
            ..Default::default()
        };

        let repo = Repository::init(".")?;
        let settings = toml::to_string(&settings)?;

        run_cmd!(
            echo $settings > cog.toml;
            git add .;
            git commit -m "first commit";
            git commit --allow-empty -m "feature one";
            git tag lunatic-timer-api-v0.12.0;
        )?;

        run_cmd!(git tag)?;

        // Act
        let tag = repo.get_latest_package_tag("lunatic-timer-api")?;

        // Assert
        assert_that!(tag.to_string()).is_equal_to(&"lunatic-timer-api-v0.12.0".to_string());
        Ok(())
    }
}
