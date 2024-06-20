use git2::Oid;

use crate::error::Git2Error;
use crate::rev::cache::get_cache;
use crate::Repository;
use cocogitto_config::SETTINGS;
use cocogitto_oid::OidOf;
use cocogitto_tag::error::TagError;
use cocogitto_tag::Tag;

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
    pub fn create_tag(&self, tag: &Tag, disable_bump_commit: bool) -> Result<(), Git2Error> {
        if !disable_bump_commit && self.get_diff(true).is_some() {
            let statuses = self.get_statuses()?;
            return Err(Git2Error::ChangesNeedToBeCommitted(statuses));
        }

        let head = self.get_head_commit().unwrap();
        self.0
            .tag_lightweight(&tag.to_string(), &head.into_object(), false)
            .map(|_| ())
            .map_err(Git2Error::from)
    }

    pub fn create_annotated_tag(
        &self,
        tag: &Tag,
        msg: &str,
        disable_bump_commit: bool,
    ) -> Result<(), Git2Error> {
        if !disable_bump_commit && self.get_diff(true).is_some() {
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
    pub fn get_latest_tag(&self, options: TagLookUpOptions) -> Result<Tag, TagError> {
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

    pub fn get_latest_tag_oid(&self, options: TagLookUpOptions) -> Result<Oid, TagError> {
        self.get_latest_tag(options)
            .map(|tag| tag.oid_unchecked().to_owned())
    }

    pub fn tag_lookup(&self, option: TagLookUpOptions) -> Result<Vec<Tag>, TagError> {
        let prefix = SETTINGS.tag_prefix.as_ref();
        let repo_cache = get_cache(self);
        let include_pre_release = option.include_pre_release;

        let tag_filter = |tag: &Tag| {
            tag.prefix == prefix.map(|p| p.as_ref())
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

    use crate::test::commit;
    use crate::test::git_tag;
    use cocogitto_config::monorepo::MonoRepoPackage;
    use cocogitto_config::{Settings, SETTINGS};

    use crate::tag::{Tag, TagLookUpOptions};
    use crate::test::git_init_no_gpg;

    #[test]
    fn should_compare_tags() -> Result<()> {
        let v1_0_0 = Tag::from_str(
            "1.0.0",
            None,
            None,
            SETTINGS.tag_prefix(),
            SETTINGS.monorepo_separator(),
            SETTINGS.package_names(),
        )?;
        let v1_1_0 = Tag::from_str(
            "1.1.0",
            None,
            None,
            SETTINGS.tag_prefix(),
            SETTINGS.monorepo_separator(),
            SETTINGS.package_names(),
        )?;
        let v2_1_0 = Tag::from_str(
            "2.1.0",
            None,
            None,
            SETTINGS.tag_prefix(),
            SETTINGS.monorepo_separator(),
            SETTINGS.package_names(),
        )?;
        let v0_1_0 = Tag::from_str(
            "0.1.0",
            None,
            None,
            SETTINGS.tag_prefix(),
            SETTINGS.monorepo_separator(),
            SETTINGS.package_names(),
        )?;
        let v0_2_0 = Tag::from_str(
            "0.2.0",
            None,
            None,
            SETTINGS.tag_prefix(),
            SETTINGS.monorepo_separator(),
            SETTINGS.package_names(),
        )?;
        let v0_0_1 = Tag::from_str(
            "0.0.1",
            None,
            None,
            SETTINGS.tag_prefix(),
            SETTINGS.monorepo_separator(),
            SETTINGS.package_names(),
        )?;
        assert_that!([v1_0_0, v1_1_0, v2_1_0, v0_1_0, v0_2_0, v0_0_1,]
            .iter()
            .max())
        .is_some()
        .is_equal_to(&Tag::from_str(
            "2.1.0",
            None,
            None,
            SETTINGS.tag_prefix(),
            SETTINGS.monorepo_separator(),
            SETTINGS.package_names(),
        )?);

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

        let v1_0_0 = Tag::from_str(
            "v1.0.0",
            None,
            None,
            SETTINGS.tag_prefix(),
            SETTINGS.monorepo_separator(),
            SETTINGS.package_names(),
        )?;
        let v1_1_0 = Tag::from_str(
            "v1.1.0",
            None,
            None,
            SETTINGS.tag_prefix(),
            SETTINGS.monorepo_separator(),
            SETTINGS.package_names(),
        )?;
        let v2_1_0 = Tag::from_str(
            "v2.1.0",
            None,
            None,
            SETTINGS.tag_prefix(),
            SETTINGS.monorepo_separator(),
            SETTINGS.package_names(),
        )?;
        let v0_1_0 = Tag::from_str(
            "v0.1.0",
            None,
            None,
            SETTINGS.tag_prefix(),
            SETTINGS.monorepo_separator(),
            SETTINGS.package_names(),
        )?;
        let v0_2_0 = Tag::from_str(
            "v0.2.0",
            None,
            None,
            SETTINGS.tag_prefix(),
            SETTINGS.monorepo_separator(),
            SETTINGS.package_names(),
        )?;
        let v0_0_1 = Tag::from_str(
            "v0.0.1",
            None,
            None,
            SETTINGS.tag_prefix(),
            SETTINGS.monorepo_separator(),
            SETTINGS.package_names(),
        )?;
        assert_that!([v1_0_0, v1_1_0, v2_1_0, v0_1_0, v0_2_0, v0_0_1,]
            .iter()
            .max())
        .is_some()
        .is_equal_to(&Tag::from_str(
            "2.1.0",
            None,
            None,
            SETTINGS.tag_prefix(),
            SETTINGS.monorepo_separator(),
            SETTINGS.package_names(),
        )?);

        Ok(())
    }

    #[test]
    fn should_get_tag_from_str() -> Result<()> {
        let tag = Tag::from_str(
            "1.0.0",
            None,
            None,
            SETTINGS.tag_prefix(),
            SETTINGS.monorepo_separator(),
            SETTINGS.package_names(),
        );
        assert_that!(tag).is_ok().is_equal_to(Tag {
            package: None,
            prefix: None,
            monorepo_separator: None,
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

        let tag = Tag::from_str(
            "v1.0.0",
            None,
            None,
            SETTINGS.tag_prefix(),
            SETTINGS.monorepo_separator(),
            SETTINGS.package_names(),
        );

        assert_that!(tag).is_ok().is_equal_to(Tag {
            package: None,
            prefix: Some("v"),
            monorepo_separator: None,
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

        let tag = Tag::from_str(
            "one-1.0.0",
            None,
            None,
            SETTINGS.tag_prefix(),
            SETTINGS.monorepo_separator(),
            SETTINGS.package_names(),
        );

        assert_that!(tag).is_ok().is_equal_to(Tag {
            package: Some("one".to_string()),
            prefix: None,
            monorepo_separator: None,
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

        let tag = Tag::from_str(
            "one-v1.0.0",
            None,
            None,
            SETTINGS.tag_prefix(),
            SETTINGS.monorepo_separator(),
            SETTINGS.package_names(),
        );

        assert_that!(tag).is_ok().is_equal_to(Tag {
            package: Some("one".to_string()),
            prefix: Some("v"),
            monorepo_separator: None,
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

        let tag = Tag::from_str(
            "one#v1.0.0",
            None,
            None,
            SETTINGS.tag_prefix(),
            SETTINGS.monorepo_separator(),
            SETTINGS.package_names(),
        );

        assert_that!(tag).is_ok().is_equal_to(Tag {
            package: Some("one".to_string()),
            prefix: Some("v"),
            monorepo_separator: None,
            version: Version::new(1, 0, 0),
            oid: None,
            target: None,
        });

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
