mod error;
mod parser;

use std::collections::{HashMap, VecDeque};
use std::ops::Range;
use std::process::Command;
use std::str::FromStr;
use std::{fmt, path};

use crate::hook::parser::VersionAccessToken;
use crate::Tag;
use parser::Token;

use crate::settings::{BumpProfile, HookType};
use anyhow::{anyhow, ensure, Result};

pub trait Hooks {
    fn bump_profiles(&self) -> &HashMap<String, BumpProfile>;
    fn pre_bump_hooks(&self) -> &Vec<String>;
    fn post_bump_hooks(&self) -> &Vec<String>;

    fn get_hooks(&self, hook_type: HookType) -> &Vec<String> {
        match hook_type {
            HookType::PreBump => self.pre_bump_hooks(),
            HookType::PostBump => self.post_bump_hooks(),
        }
    }

    fn get_profile_hooks(&self, profile: &str, hook_type: HookType) -> &Vec<String> {
        let profile = self
            .bump_profiles()
            .get(profile)
            .expect("Bump profile not found");
        match hook_type {
            HookType::PreBump => &profile.pre_bump_hooks,
            HookType::PostBump => &profile.post_bump_hooks,
        }
    }
}

#[derive(Debug, Eq, PartialEq)]
pub struct VersionSpan {
    range: Range<usize>,
    tokens: VecDeque<Token>,
}

pub(crate) struct HookVersion {
    pub prefixed_tag: Tag,
}

impl HookVersion {
    pub(crate) fn new(tag: Tag) -> Self {
        HookVersion { prefixed_tag: tag }
    }
}

impl VersionSpan {
    pub(crate) fn build_version_str(
        &mut self,
        version: Option<&HookVersion>,
        latest: Option<&HookVersion>,
    ) -> Result<String> {
        // According to the pest grammar, a `version` or `latest_version` token is expected first
        let mut tag = match self.tokens.pop_front() {
            Some(Token::Version) => version
                .map(|version| version.prefixed_tag.strip_metadata())
                .ok_or_else(|| anyhow!("No previous tag found to replace {{{{version}}}} version")),
            Some(Token::LatestVersion) => latest
                .map(|version| version.prefixed_tag.strip_metadata())
                .ok_or_else(|| anyhow!("No previous tag found to replace {{{{latest}}}} version")),
            Some(Token::LatestVersionTag) => latest
                .map(|version| version.prefixed_tag.clone())
                .ok_or_else(|| {
                    anyhow!("No previous tag found to replace {{{{latest_tag}}}} version")
                }),
            Some(Token::VersionTag) => version
                .map(|version| version.prefixed_tag.clone())
                .ok_or_else(|| {
                    anyhow!("No previous tag found to replace {{{{version_tag}}}} version")
                }),
            Some(Token::Package) => {
                return version
                    .and_then(|version| version.prefixed_tag.package.clone())
                    .ok_or_else(|| anyhow!("Current tag as no {{{{package}}}} info"))
            }

            _ => unreachable!("Unexpected parsing error"),
        }?;

        let mut amount = 1;
        let mut version_access_token: Option<VersionAccessToken> = None;

        while let Some(token) = self.tokens.pop_front() {
            match token {
                // reset the increment amount to default whenever we encounter a `+` token
                Token::Add => amount = 1,
                // set the desired amount
                Token::Amount(amt) => amount = amt,
                // increments ...
                Token::Major => {
                    tag.version.major += amount;
                    tag.version.minor = 0;
                    tag.version.patch = 0;
                }
                Token::Minor => {
                    tag.version.minor += amount;
                    tag.version.patch = 0;
                }
                Token::Patch => tag.version.patch += amount,
                // set  build metadata and prerelease
                Token::PreRelease(pre_release) => tag.version.pre = pre_release,
                Token::BuildMetadata(build) => tag.version.build = build,
                Token::VersionAccess(version_access) => {
                    version_access_token = Some(version_access);
                }
                _ => unreachable!("Unexpected parsing error"),
            }
        }

        if let Some(version_access) = version_access_token {
            Ok(match version_access {
                VersionAccessToken::Major => tag.version.major.to_string(),
                VersionAccessToken::Minor => tag.version.minor.to_string(),
                VersionAccessToken::Patch => tag.version.patch.to_string(),
            })
        } else {
            Ok(tag.to_string())
        }
    }
}

#[derive(Debug, Eq, PartialEq)]
pub struct HookSpan {
    version_spans: Vec<VersionSpan>,
    content: String,
}

impl HookSpan {
    fn replace_versions(
        &mut self,
        version: Option<&HookVersion>,
        latest: Option<&HookVersion>,
    ) -> Result<String> {
        let mut output = self.content.clone();
        if let Some(mut span) = self.version_spans.pop() {
            let version_str = span.build_version_str(version, latest)?;
            let version_str = version_str.as_str();
            output.replace_range(span.range.clone(), version_str);
            output = parser::parse(&output)?.replace_versions(version, latest)?;
        }

        Ok(output)
    }
}

#[derive(Debug)]
pub struct Hook(String);

impl FromStr for Hook {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        ensure!(!s.is_empty(), "hook must not be an empty string");
        Ok(Hook(s.to_string()))
    }
}

impl fmt::Display for Hook {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str(&self.0)
    }
}

impl Hook {
    pub(crate) fn insert_versions(
        &mut self,
        current_version: Option<&HookVersion>,
        next_version: Option<&HookVersion>,
    ) -> Result<()> {
        let mut parts = parser::parse(&self.0)?;
        self.0 = parts.replace_versions(next_version, current_version)?;

        Ok(())
    }

    pub fn run(&self, package_path: Option<&path::Path>) -> Result<()> {
        let mut cmd = Command::new("sh");
        let cmd = cmd.arg("-c").arg(&self.0);
        if let Some(current_dir) = package_path {
            cmd.current_dir(current_dir);
        }
        let status = cmd.status()?;
        ensure!(status.success(), "hook failed with status {}", status);
        Ok(())
    }
}

#[cfg(test)]
mod test {
    use cmd_lib::run_cmd;
    use git2::Repository;
    use std::collections::HashMap;
    use std::str::FromStr;

    use crate::{Result, Tag};

    use crate::hook::{Hook, HookVersion};
    use crate::settings::{MonoRepoPackage, Settings};
    use sealed_test::prelude::*;
    use semver::Version;
    use speculoos::prelude::*;

    #[test]
    fn parse_empty_string() {
        let empty_hook = Hook::from_str("");
        assert_that!(empty_hook).is_err();
    }

    #[test]
    fn parse_valid_string() -> Result<()> {
        let hook = Hook::from_str("cargo bump {{version}}")?;
        assert_that!(hook.0.as_str()).is_equal_to("cargo bump {{version}}");
        Ok(())
    }

    #[test]
    fn parse_current_version() -> Result<()> {
        let hook = Hook::from_str("cargo bump {{version_tag}}")?;
        assert_that!(hook.0.as_str()).is_equal_to("cargo bump {{version_tag}}");
        Ok(())
    }

    #[test]
    fn parse_latest_tag() -> Result<()> {
        let hook = Hook::from_str("cargo bump {{latest_tag}}")?;
        assert_that!(hook.0.as_str()).is_equal_to("cargo bump {{latest_tag}}");
        Ok(())
    }

    #[test]
    fn replace_version_cargo() -> Result<()> {
        let mut hook = Hook::from_str("cargo bump {{version}}")?;
        hook.insert_versions(None, Some(&HookVersion::new(Tag::from_str("1.0.0", None)?)))
            .unwrap();

        assert_that!(hook.0.as_str()).is_equal_to("cargo bump 1.0.0");
        Ok(())
    }

    #[test]
    fn replace_version_tag_cargo() -> Result<()> {
        let mut hook = Hook::from_str("cargo bump {{version_tag}}")?;
        let tag = Tag {
            package: None,
            prefix: Some("v".to_string()),
            version: Version::new(1, 0, 0),
            oid: None,
        };

        hook.insert_versions(None, Some(&HookVersion::new(tag)))
            .unwrap();

        assert_that!(hook.0.as_str()).is_equal_to("cargo bump v1.0.0");
        Ok(())
    }

    #[sealed_test]
    fn replace_version_tag_with_package() -> Result<()> {
        let mut packages = HashMap::new();
        packages.insert("cog".to_string(), MonoRepoPackage::default());
        let settings = Settings {
            packages,
            ..Default::default()
        };

        let settings = toml::to_string(&settings)?;

        run_cmd!(
            git init;
            echo $settings > cog.toml;
            git add .;
            git commit -m "first commit";
        )?;

        let mut hook = Hook::from_str("echo {{version_tag}}")?;

        let tag = Tag {
            package: Some("cog".to_string()),
            prefix: Some("v".to_string()),
            version: Version::new(1, 0, 0),
            oid: None,
        };

        hook.insert_versions(None, Some(&HookVersion::new(tag)))
            .unwrap();

        assert_that!(hook.0.as_str()).is_equal_to("echo cog-v1.0.0");
        Ok(())
    }

    #[test]
    fn replace_latest_tag() -> Result<()> {
        let mut hook = Hook::from_str("echo {{latest_tag}}")?;
        let tag = Tag {
            package: None,
            prefix: Some("v".to_string()),
            version: Version::new(1, 0, 0),
            oid: None,
        };

        hook.insert_versions(Some(&HookVersion::new(tag)), None)
            .unwrap();

        assert_that!(hook.0.as_str()).is_equal_to("echo v1.0.0");
        Ok(())
    }

    #[test]
    fn replace_maven_version() -> Result<()> {
        let mut hook = Hook::from_str("mvn versions:set -DnewVersion={{version}}")?;
        hook.insert_versions(None, Some(&HookVersion::new(Tag::from_str("1.0.0", None)?)))
            .unwrap();

        assert_that!(hook.0.as_str()).is_equal_to("mvn versions:set -DnewVersion=1.0.0");
        Ok(())
    }

    #[test]
    fn replace_maven_version_with_expression() -> Result<()> {
        let mut hook = Hook::from_str("mvn versions:set -DnewVersion={{version+1minor-SNAPSHOT}}")?;
        hook.insert_versions(None, Some(&HookVersion::new(Tag::from_str("1.0.0", None)?)))
            .unwrap();

        assert_that!(hook.0.as_str()).is_equal_to("mvn versions:set -DnewVersion=1.1.0-SNAPSHOT");
        Ok(())
    }

    #[test]
    fn replace_version_tag_with_expression() -> Result<()> {
        let mut hook =
            Hook::from_str("mvn versions:set -DnewVersion={{version_tag+1minor-SNAPSHOT}}")?;
        let tag = Tag {
            package: None,
            prefix: Some("v".to_string()),
            version: Version::new(1, 0, 0),
            oid: None,
        };

        hook.insert_versions(None, Some(&HookVersion::new(tag)))
            .unwrap();

        assert_that!(hook.0.as_str()).is_equal_to("mvn versions:set -DnewVersion=v1.1.0-SNAPSHOT");
        Ok(())
    }

    #[sealed_test]
    fn replace_package_version_tag_with_expression() -> Result<()> {
        let mut packages = HashMap::new();
        packages.insert("cog".to_string(), MonoRepoPackage::default());
        let settings = Settings {
            packages,
            ..Default::default()
        };

        let settings = toml::to_string(&settings)?;

        run_cmd!(
            git init;
            echo $settings > cog.toml;
            git add .;
            git commit -m "first commit";
        )?;

        let mut hook =
            Hook::from_str("mvn versions:set -DnewVersion={{version_tag+1minor-SNAPSHOT}}")?;

        let tag = Tag {
            package: Some("cog".to_string()),
            prefix: Some("v".to_string()),
            version: Version::new(1, 0, 0),
            oid: None,
        };

        hook.insert_versions(None, Some(&HookVersion::new(tag)))
            .unwrap();

        assert_that!(hook.0.as_str())
            .is_equal_to("mvn versions:set -DnewVersion=cog-v1.1.0-SNAPSHOT");
        Ok(())
    }

    #[test]
    fn leave_hook_untouched_when_no_version() -> Result<()> {
        let mut hook = Hook::from_str("echo \"Hello World\"")?;
        hook.insert_versions(None, Some(&HookVersion::new(Tag::from_str("1.0.0", None)?)))
            .unwrap();

        assert_that!(hook.0.as_str()).is_equal_to("echo \"Hello World\"");
        Ok(())
    }

    #[test]
    fn replace_quoted_version() -> Result<()> {
        let mut hook = Hook::from_str("echo \"{{version}}\"")?;
        hook.insert_versions(None, Some(&HookVersion::new(Tag::from_str("1.0.0", None)?)))
            .unwrap();

        assert_that!(hook.0.as_str()).is_equal_to("echo \"1.0.0\"");
        Ok(())
    }

    #[test]
    fn replace_version_with_nested_simple_quoted_arg() -> Result<()> {
        let mut hook =
            Hook::from_str("cog commit chore 'bump snapshot to {{version+1minor-pre}}'")?;
        hook.insert_versions(None, Some(&HookVersion::new(Tag::from_str("1.0.0", None)?)))
            .unwrap();

        assert_that!(hook.0.as_str()).is_equal_to("cog commit chore 'bump snapshot to 1.1.0-pre'");
        Ok(())
    }

    #[test]
    fn replace_version_with_nested_double_quoted_arg() -> Result<()> {
        let mut hook =
            Hook::from_str("cog commit chore \"bump snapshot to {{version+1minor-pre}}\"")?;
        hook.insert_versions(None, Some(&HookVersion::new(Tag::from_str("1.0.0", None)?)))
            .unwrap();

        assert_that!(hook.0.as_str())
            .is_equal_to("cog commit chore \"bump snapshot to 1.1.0-pre\"");
        Ok(())
    }

    #[test]
    fn replace_version_with_multiple_placeholders() -> Result<()> {
        let mut hook = Hook::from_str("echo \"the latest {{latest}}, the greatest {{version}}\"")?;
        hook.insert_versions(
            Some(&HookVersion::new(Tag::from_str("0.5.9", None)?)),
            Some(&HookVersion::new(Tag::from_str("1.0.0", None)?)),
        )
        .unwrap();

        assert_that!(hook.0.as_str()).is_equal_to("echo \"the latest 0.5.9, the greatest 1.0.0\"");
        Ok(())
    }

    #[test]
    fn replace_version_with_multiple_placeholders_and_increments() -> Result<()> {
        let mut hook = Hook::from_str(
            "echo \"the latest {{latest+3major+1minor}}, the greatest {{version+2patch}}\"",
        )?;
        hook.insert_versions(
            Some(&HookVersion::new(Tag::from_str("0.5.9", None)?)),
            Some(&HookVersion::new(Tag::from_str("1.0.0", None)?)),
        )
        .unwrap();

        assert_that!(hook.0.as_str()).is_equal_to("echo \"the latest 3.1.0, the greatest 1.0.2\"");
        Ok(())
    }

    #[test]
    fn replace_version_with_pre_and_build_metadata() -> Result<()> {
        let mut hook =
            Hook::from_str("echo \"the latest {{version+1major-pre.alpha-bravo+build.42}}\"")?;
        hook.insert_versions(None, Some(&HookVersion::new(Tag::from_str("1.0.0", None)?)))
            .unwrap();

        assert_that!(hook.0.as_str())
            .is_equal_to("echo \"the latest 2.0.0-pre.alpha-bravo+build.42\"");
        Ok(())
    }

    #[test]
    fn replace_version_tag_with_pre_and_build_metadata() -> Result<()> {
        let mut hook =
            Hook::from_str("echo \"the latest {{version_tag+1major-pre.alpha-bravo+build.42}}\"")?;

        let tag = Tag {
            package: None,
            prefix: Some("v".to_string()),
            version: Version::new(1, 0, 0),
            oid: None,
        };

        hook.insert_versions(None, Some(&HookVersion::new(tag)))
            .unwrap();

        assert_that!(hook.0.as_str())
            .is_equal_to("echo \"the latest v2.0.0-pre.alpha-bravo+build.42\"");

        Ok(())
    }

    #[sealed_test]
    fn parenthesis_in_hook_works() -> Result<()> {
        Repository::init(".")?;

        let mut hook = Hook::from_str("git commit --allow-empty -m 'chore(snapshot): bump snapshot to {{version+1patch-SNAPSHOT}}'")?;

        hook.insert_versions(None, Some(&HookVersion::new(Tag::from_str("1.0.0", None)?)))
            .unwrap();

        let outcome = hook.run(None);

        assert_that!(outcome).is_ok();

        Ok(())
    }

    #[sealed_test]
    fn replace_package_name_and_version_tag_with_expression() -> Result<()> {
        let mut packages = HashMap::new();
        packages.insert("cog".to_string(), MonoRepoPackage::default());
        let settings = Settings {
            packages,
            ..Default::default()
        };

        let settings = toml::to_string(&settings)?;

        run_cmd!(
            git init;
            echo $settings > cog.toml;
            git add .;
            git commit -m "first commit";
        )?;

        let mut hook = Hook::from_str(
            r#"echo "{{package}}, version: {{version}}, tag: {{version_tag}}, current: {{latest}}, current_tag: {{latest_tag}}""#,
        )?;

        let current = Tag {
            package: Some("cog".to_string()),
            prefix: Some("v".to_string()),
            version: Version::new(1, 0, 0),
            oid: None,
        };

        let tag = Tag {
            package: Some("cog".to_string()),
            prefix: Some("v".to_string()),
            version: Version::new(1, 1, 0),
            oid: None,
        };

        hook.insert_versions(
            Some(&HookVersion::new(current)),
            Some(&HookVersion::new(tag)),
        )
        .unwrap();

        assert_that!(hook.0.as_str())
            .is_equal_to(r#"echo "cog, version: 1.1.0, tag: cog-v1.1.0, current: 1.0.0, current_tag: cog-v1.0.0""#);
        Ok(())
    }

    #[sealed_test]
    fn replace_major_minor_patch() -> Result<()> {
        let mut packages = HashMap::new();
        packages.insert("cog".to_string(), MonoRepoPackage::default());
        let settings = Settings {
            packages,
            ..Default::default()
        };

        let settings = toml::to_string(&settings)?;

        run_cmd!(
            git init;
            echo $settings > cog.toml;
            git add .;
            git commit -m "first commit";
        )?;

        let mut hook = Hook::from_str(
            r#"major={{version.major}} minor={{version.minor}} patch={{version.patch}}"#,
        )?;

        let current = Tag {
            package: Some("cog".to_string()),
            prefix: Some("v".to_string()),
            version: Version::new(1, 0, 0),
            oid: None,
        };

        let tag = Tag {
            package: Some("cog".to_string()),
            prefix: Some("v".to_string()),
            version: Version::new(1, 2, 3),
            oid: None,
        };

        hook.insert_versions(
            Some(&HookVersion::new(current)),
            Some(&HookVersion::new(tag)),
        )
        .unwrap();

        assert_that!(hook.0.as_str()).is_equal_to(r#"major=1 minor=2 patch=3"#);
        Ok(())
    }
}
