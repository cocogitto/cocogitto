use std::cmp::Ordering;
use std::fmt;
use std::fmt::Formatter;

use git2::Oid;
use semver::Version;

use crate::conventional::version::Increment;
use crate::git::error::{Git2Error, TagError};
use crate::git::oid::OidOf;
use crate::git::repository::Repository;
use crate::SETTINGS;

#[derive(Debug, Default)]
pub struct TagLookUpOptions<'a> {
    include_pre_release: bool,
    package_name: Option<&'a str>,
    packages_only: bool,
    include_packages: bool,
}

impl<'a> TagLookUpOptions<'a> {
    /// Perform a tag lookup with pre-release tag included
    pub fn include_pre_release(mut self) -> Self {
        self.include_pre_release = true;
        self
    }

    /// Perform a tag lookup, keeping only tags for the given package
    pub fn package(package: &'a str) -> Self {
        TagLookUpOptions {
            include_pre_release: false,
            package_name: Some(package),
            packages_only: true,
            include_packages: true,
        }
    }

    /// Perform a tag lookup, keeping only packages tags.
    pub fn packages_only(mut self) -> Self {
        self.packages_only = true;
        self
    }

    /// Perform a tag lookup, mixing non package and package tags.
    pub fn include_packages(mut self) -> Self {
        self.include_packages = true;
        self
    }
}

impl Repository {
    /// Given a tag name return a [`Tag`], this will fail if the requested
    /// tag (without configured prefix) is not semver compliant or if the tag
    /// does not exist.
    pub fn resolve_tag(&self, tag: &str) -> Result<Tag, TagError> {
        self.0
            .resolve_reference_from_short_name(tag)
            .map_err(|err| TagError::not_found(tag, err))
            .map(|reference| reference.target().unwrap())
            .map(|commit_oid| {
                if let Ok(annotated_tag) = self.0.find_tag(commit_oid) {
                    Tag::from_str(tag, Some(commit_oid), Some(annotated_tag.target_id()))
                } else {
                    Tag::from_str(tag, Some(commit_oid), None)
                }
            })?
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
    pub(crate) fn get_latest_tag(&self, options: TagLookUpOptions) -> Result<Tag, TagError> {
        let tags: Vec<Tag> = self.tag_lookup(options)?;
        tags.into_iter().max().ok_or(TagError::NoTag)
    }

    pub(crate) fn get_previous_tag(&self, current: &Tag) -> Result<Option<Tag>, TagError> {
        let mut options = match &current.package {
            None => TagLookUpOptions::default(),
            Some(package) => TagLookUpOptions::package(package),
        };

        if !current.version.pre.is_empty() {
            options.include_pre_release = true
        }

        let mut tags: Vec<Tag> = self
            .tag_lookup(options)?
            .into_iter()
            .filter(|tag| tag.package == current.package)
            .collect();

        tags.sort();

        let Some(current_idx) = tags
            .iter()
            .enumerate()
            .find(|(_, tag)| tag == &current)
            .map(|(idx, _)| idx)
        else {
            return Ok(None);
        };

        if current_idx == 0 {
            return Ok(None);
        }

        Ok(tags.get(current_idx - 1).cloned())
    }

    pub(crate) fn get_latest_tag_oid(&self, options: TagLookUpOptions) -> Result<Oid, TagError> {
        self.get_latest_tag(options)
            .map(|tag| tag.oid_unchecked().to_owned())
    }

    pub fn tag_lookup(&self, option: TagLookUpOptions) -> Result<Vec<Tag>, TagError> {
        let prefix = SETTINGS.tag_prefix.as_ref();
        let repo_cache = crate::git::rev::refresh(self);
        let include_pre_release = option.include_pre_release;

        let tag_filter = |tag: &Tag| {
            tag.prefix.as_ref() == prefix
                && tag.package.as_deref() == option.package_name
                && option.include_packages != tag.package.is_none()
                && if include_pre_release {
                    true
                } else {
                    tag.version.pre.is_empty()
                }
        };

        Ok(repo_cache
            .values()
            .filter_map(|oid| match oid {
                OidOf::Tag(tag) if tag_filter(tag) => Some(tag),
                _ => None,
            })
            .cloned()
            .collect())
    }
}

#[derive(Debug, Eq, Clone)]
pub struct Tag {
    pub package: Option<String>,
    pub prefix: Option<String>,
    pub version: Version,
    pub oid: Option<Oid>,
    pub target: Option<Oid>,
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
            target: None,
        }
    }

    pub(crate) fn oid(&self) -> Option<&Oid> {
        self.oid.as_ref()
    }

    pub fn from_str(raw: &str, oid: Option<Oid>, target: Option<Oid>) -> Result<Tag, TagError> {
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
                prefix: prefix.cloned(),
                version,
                oid,
                target,
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
    use std::collections::HashMap;
    use std::fs;
    use std::path::PathBuf;

    use anyhow::Result;
    use cmd_lib::run_cmd;
    use sealed_test::prelude::*;
    use semver::Version;
    use speculoos::prelude::*;

    use crate::git::tag::{Tag, TagLookUpOptions};
    use crate::settings::{MonoRepoPackage, Settings};
    use crate::test_helpers::{commit, git_init_no_gpg, git_tag};

    #[test]
    fn should_compare_tags() -> Result<()> {
        let v1_0_0 = Tag::from_str("1.0.0", None, None)?;
        let v1_1_0 = Tag::from_str("1.1.0", None, None)?;
        let v2_1_0 = Tag::from_str("2.1.0", None, None)?;
        let v0_1_0 = Tag::from_str("0.1.0", None, None)?;
        let v0_2_0 = Tag::from_str("0.2.0", None, None)?;
        let v0_0_1 = Tag::from_str("0.0.1", None, None)?;
        assert_that!([v1_0_0, v1_1_0, v2_1_0, v0_1_0, v0_2_0, v0_0_1,]
            .iter()
            .max())
        .is_some()
        .is_equal_to(&Tag::from_str("2.1.0", None, None)?);

        Ok(())
    }

    #[sealed_test]
    fn tag_lookup() -> Result<()> {
        let repository = git_init_no_gpg()?;
        let settings = Settings {
            tag_prefix: Some("v".to_string()),
            ..Default::default()
        };
        let settings = toml::to_string(&settings)?;
        fs::write("cog.toml", settings)?;

        commit("first commit")?;
        git_tag("v1.0.0")?;
        git_tag("v1.1.0")?;
        git_tag("v2.1.0")?;
        git_tag("v0.1.0")?;
        git_tag("v0.2.0")?;
        git_tag("v0.0.1")?;

        let result = repository.tag_lookup(TagLookUpOptions::default())?;

        let tags: Vec<_> = result.into_iter().map(|tag| tag.to_string()).collect();

        assert_that!(tags).contains_all_of(&[
            &"v1.0.0".to_string(),
            &"v1.1.0".to_string(),
            &"v2.1.0".to_string(),
            &"v0.1.0".to_string(),
            &"v0.2.0".to_string(),
            &"v0.0.1".to_string(),
        ]);
        Ok(())
    }

    #[sealed_test]
    fn should_compare_tags_with_prefix() -> Result<()> {
        git_init_no_gpg()?;
        let settings = Settings {
            tag_prefix: Some("v".to_string()),
            ..Default::default()
        };
        let settings = toml::to_string(&settings)?;
        fs::write("cog.toml", settings)?;

        let v1_0_0 = Tag::from_str("v1.0.0", None, None)?;
        let v1_1_0 = Tag::from_str("v1.1.0", None, None)?;
        let v2_1_0 = Tag::from_str("v2.1.0", None, None)?;
        let v0_1_0 = Tag::from_str("v0.1.0", None, None)?;
        let v0_2_0 = Tag::from_str("v0.2.0", None, None)?;
        let v0_0_1 = Tag::from_str("v0.0.1", None, None)?;
        assert_that!([v1_0_0, v1_1_0, v2_1_0, v0_1_0, v0_2_0, v0_0_1,]
            .iter()
            .max())
        .is_some()
        .is_equal_to(&Tag::from_str("2.1.0", None, None)?);

        Ok(())
    }

    #[test]
    fn should_get_tag_from_str() -> Result<()> {
        let tag = Tag::from_str("1.0.0", None, None);
        assert_that!(tag).is_ok().is_equal_to(Tag {
            package: None,
            prefix: None,
            version: Version::new(1, 0, 0),
            oid: None,
            target: None,
        });

        Ok(())
    }

    #[sealed_test]
    fn should_get_tag_from_str_with_prefix() -> Result<()> {
        git_init_no_gpg()?;

        let settings = Settings {
            tag_prefix: Some("v".to_string()),
            ..Default::default()
        };

        let settings = toml::to_string(&settings)?;
        fs::write("cog.toml", settings)?;

        let tag = Tag::from_str("v1.0.0", None, None);

        assert_that!(tag).is_ok().is_equal_to(Tag {
            package: None,
            prefix: Some("v".to_string()),
            version: Version::new(1, 0, 0),
            oid: None,
            target: None,
        });

        Ok(())
    }

    #[sealed_test]
    fn should_get_tag_from_str_with_separator() -> Result<()> {
        git_init_no_gpg()?;

        let mut packages = HashMap::new();
        packages.insert("one".to_string(), Default::default());
        let settings = Settings {
            packages,
            ..Default::default()
        };

        let settings = toml::to_string(&settings)?;
        fs::write("cog.toml", settings)?;

        let tag = Tag::from_str("one-1.0.0", None, None);

        assert_that!(tag).is_ok().is_equal_to(Tag {
            package: Some("one".to_string()),
            prefix: None,
            version: Version::new(1, 0, 0),
            oid: None,
            target: None,
        });

        Ok(())
    }

    #[sealed_test]
    fn should_get_tag_from_str_with_prefix_and_separator() -> Result<()> {
        git_init_no_gpg()?;

        let mut packages = HashMap::new();
        packages.insert("one".to_string(), Default::default());
        let settings = Settings {
            tag_prefix: Some("v".to_string()),
            packages,
            ..Default::default()
        };

        let settings = toml::to_string(&settings)?;
        fs::write("cog.toml", settings)?;

        let tag = Tag::from_str("one-v1.0.0", None, None);

        assert_that!(tag).is_ok().is_equal_to(Tag {
            package: Some("one".to_string()),
            prefix: Some("v".to_string()),
            version: Version::new(1, 0, 0),
            oid: None,
            target: None,
        });

        Ok(())
    }

    #[sealed_test]
    fn should_get_tag_from_str_with_prefix_and_custom_separator() -> Result<()> {
        git_init_no_gpg()?;

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

        let tag = Tag::from_str("one#v1.0.0", None, None);

        assert_that!(tag).is_ok().is_equal_to(Tag {
            package: Some("one".to_string()),
            prefix: Some("v".to_string()),
            version: Version::new(1, 0, 0),
            oid: None,
            target: None,
        });

        Ok(())
    }

    #[sealed_test]
    fn resolve_annotated_tag_ok() -> Result<()> {
        // Arrange
        let repo = git_init_no_gpg()?;
        run_cmd!(
            git commit --allow-empty -m "first commit";
            git tag -a 1.0.0 -m "annotated tag";
        )?;

        // Act
        let tag = repo.resolve_tag("1.0.0");

        // Assert
        assert_that!(tag).is_ok();
        Ok(())
    }

    #[sealed_test]
    fn resolve_lightweight_tag_ok() -> Result<()> {
        // Arrange
        let repo = git_init_no_gpg()?;
        run_cmd!(
            git commit --allow-empty -m "first commit";
            git tag 1.0.0;
        )?;

        // Act
        let tag = repo.resolve_tag("1.0.0");

        // Assert
        assert_that!(tag).is_ok();
        Ok(())
    }

    #[sealed_test]
    fn resolve_lightweight_tag_err() -> Result<()> {
        // Arrange
        let repo = git_init_no_gpg()?;
        run_cmd!(
            git commit --allow-empty -m "first commit";
            git tag the_tag;
        )?;

        // Act
        let tag = repo.resolve_tag("the_taaaag");

        // Assert
        assert_that!(tag).is_err();
        Ok(())
    }

    #[sealed_test]
    fn get_latest_tag_ok() -> Result<()> {
        // Arrange
        let repo = git_init_no_gpg()?;
        run_cmd!(
            git commit --allow-empty -m "first commit";
            git tag 0.1.0;
            git commit --allow-empty -m "second commit";
            git tag 0.2.0;
        )?;

        // Act
        let tag = repo.get_latest_tag(TagLookUpOptions::default())?;

        // Assert
        assert_that!(tag.to_string()).is_equal_to("0.2.0".to_string());
        Ok(())
    }

    #[sealed_test]
    fn get_previous_tag_ok() -> Result<()> {
        // Arrange
        let repo = git_init_no_gpg()?;
        run_cmd!(
            git commit --allow-empty -m "first commit";
            git tag 0.1.0;
            git commit --allow-empty -m "second commit";
            git tag 0.2.0;
        )?;

        let tag = repo.get_latest_tag(TagLookUpOptions::default())?;

        // Act
        let previous = repo.get_previous_tag(&tag)?.map(|t| t.to_string());

        // Assert
        assert_that!(tag.to_string()).is_equal_to("0.2.0".to_string());

        assert_that!(previous)
            .is_some()
            .is_equal_to("0.1.0".to_string());
        Ok(())
    }

    #[sealed_test]
    fn get_previous_tag_pre_release_ok() -> Result<()> {
        // Arrange
        let repo = git_init_no_gpg()?;
        run_cmd!(
            git commit --allow-empty -m "first commit";
            git tag 0.1.0;
            git commit --allow-empty -m "second commit";
            git tag 0.2.0-pre;
        )?;

        let tag = repo.get_latest_tag(TagLookUpOptions::default().include_pre_release())?;

        // Act
        let previous = repo.get_previous_tag(&tag)?.map(|t| t.to_string());

        // Assert
        assert_that!(tag.to_string()).is_equal_to("0.2.0-pre".to_string());

        assert_that!(previous)
            .is_some()
            .is_equal_to("0.1.0".to_string());
        Ok(())
    }

    #[sealed_test]
    fn get_latest_tag_err() -> Result<()> {
        // Arrange
        let repo = git_init_no_gpg()?;
        run_cmd!(
            git commit --allow-empty -m "first commit"
        )?;

        // Act
        let tag = repo.get_latest_tag(TagLookUpOptions::default());

        // Assert
        assert_that!(tag).is_err();
        Ok(())
    }

    #[sealed_test]
    fn get_latest_tag_oid_ok() -> Result<()> {
        // Arrange
        let repo = git_init_no_gpg()?;
        run_cmd!(
            git commit --allow-empty -m "first commit";
            git tag 0.1.0;
        )?;

        // Act
        let tag = repo.get_latest_tag_oid(TagLookUpOptions::default());

        // Assert
        assert_that!(tag).is_ok();
        Ok(())
    }

    #[sealed_test]
    fn get_latest_tag_oid_err() -> Result<()> {
        // Arrange
        let repo = git_init_no_gpg()?;
        run_cmd!(git commit --allow-empty -m "first commit")?;

        // Act
        let tag = repo.get_latest_tag_oid(TagLookUpOptions::default());

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

        let repo = git_init_no_gpg()?;
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
