mod error;
mod parser;

use std::collections::VecDeque;
use std::fmt;
use std::ops::Range;
use std::process::Command;
use std::str::FromStr;

use crate::Tag;
use parser::Token;

use anyhow::{anyhow, ensure, Result};

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
        version: &HookVersion,
        latest: Option<&HookVersion>,
    ) -> Result<String> {
        let version = version.prefixed_tag.version.clone();
        let latest = latest.map(|version| version.prefixed_tag.version.clone());

        // According to the pest grammar, a `version` or `latest_version` token is expected first
        let mut version = match self.tokens.pop_front() {
            Some(Token::Version) => Ok(version),
            Some(Token::LatestVersion) => {
                latest.ok_or_else(|| anyhow!("No previous tag found to replace {{latest}} version"))
            }
            _ => unreachable!("Unexpected parsing error"),
        }?;

        let mut amount = 1;

        while let Some(token) = self.tokens.pop_front() {
            match token {
                // reset the increment amount to default whenever we encounter a `+` token
                Token::Add => amount = 1,
                // set the desired amount
                Token::Amount(amt) => amount = amt,
                // increments ...
                Token::Major => {
                    version.major += amount;
                    version.minor = 0;
                    version.patch = 0;
                }
                Token::Minor => {
                    version.minor += amount;
                    version.patch = 0;
                }
                Token::Patch => version.patch += amount,
                // set  build metadata and prerelease
                Token::PreRelease(pre_release) => version.pre = pre_release,
                Token::BuildMetadata(build) => version.build = build,
                _ => unreachable!("Unexpected parsing error"),
            }
        }

        Ok(version.to_string())
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
        version: &HookVersion,
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
        next_version: &HookVersion,
    ) -> Result<()> {
        let mut parts = parser::parse(&self.0)?;
        self.0 = parts.replace_versions(next_version, current_version)?;

        Ok(())
    }

    pub fn run(&self) -> Result<()> {
        let status = Command::new("sh").arg("-c").arg(&self.0).status()?;
        ensure!(status.success(), "hook failed with status {}", status);
        Ok(())
    }
}

#[cfg(test)]
mod test {
    use git2::Repository;
    use std::str::FromStr;

    use crate::{Hook, HookVersion, Result, Tag};

    use sealed_test::prelude::*;
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
    fn replace_version_cargo() -> Result<()> {
        let mut hook = Hook::from_str("cargo bump {{version}}")?;
        hook.insert_versions(None, &HookVersion::new(Tag::from_str("1.0.0", None)?))
            .unwrap();

        assert_that!(hook.0.as_str()).is_equal_to("cargo bump 1.0.0");
        Ok(())
    }

    #[test]
    fn replace_maven_version() -> Result<()> {
        let mut hook = Hook::from_str("mvn versions:set -DnewVersion={{version}}")?;
        hook.insert_versions(None, &HookVersion::new(Tag::from_str("1.0.0", None)?))
            .unwrap();

        assert_that!(hook.0.as_str()).is_equal_to("mvn versions:set -DnewVersion=1.0.0");
        Ok(())
    }

    #[test]
    fn replace_maven_version_with_expression() -> Result<()> {
        let mut hook = Hook::from_str("mvn versions:set -DnewVersion={{version+1minor-SNAPSHOT}}")?;
        hook.insert_versions(None, &HookVersion::new(Tag::from_str("1.0.0", None)?))
            .unwrap();

        assert_that!(hook.0.as_str()).is_equal_to("mvn versions:set -DnewVersion=1.1.0-SNAPSHOT");
        Ok(())
    }

    #[test]
    fn leave_hook_untouched_when_no_version() -> Result<()> {
        let mut hook = Hook::from_str("echo \"Hello World\"")?;
        hook.insert_versions(None, &HookVersion::new(Tag::from_str("1.0.0", None)?))
            .unwrap();

        assert_that!(hook.0.as_str()).is_equal_to("echo \"Hello World\"");
        Ok(())
    }

    #[test]
    fn replace_quoted_version() -> Result<()> {
        let mut hook = Hook::from_str("echo \"{{version}}\"")?;
        hook.insert_versions(None, &HookVersion::new(Tag::from_str("1.0.0", None)?))
            .unwrap();

        assert_that!(hook.0.as_str()).is_equal_to("echo \"1.0.0\"");
        Ok(())
    }

    #[test]
    fn replace_version_with_nested_simple_quoted_arg() -> Result<()> {
        let mut hook =
            Hook::from_str("cog commit chore 'bump snapshot to {{version+1minor-pre}}'")?;
        hook.insert_versions(None, &HookVersion::new(Tag::from_str("1.0.0", None)?))
            .unwrap();

        assert_that!(hook.0.as_str()).is_equal_to("cog commit chore 'bump snapshot to 1.1.0-pre'");
        Ok(())
    }

    #[test]
    fn replace_version_with_nested_double_quoted_arg() -> Result<()> {
        let mut hook =
            Hook::from_str("cog commit chore \"bump snapshot to {{version+1minor-pre}}\"")?;
        hook.insert_versions(None, &HookVersion::new(Tag::from_str("1.0.0", None)?))
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
            &HookVersion::new(Tag::from_str("1.0.0", None)?),
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
            &HookVersion::new(Tag::from_str("1.0.0", None)?),
        )
        .unwrap();

        assert_that!(hook.0.as_str()).is_equal_to("echo \"the latest 3.1.0, the greatest 1.0.2\"");
        Ok(())
    }

    #[test]
    fn replace_version_with_pre_and_build_metadata() -> Result<()> {
        let mut hook =
            Hook::from_str("echo \"the latest {{version+1major-pre.alpha-bravo+build.42}}\"")?;
        hook.insert_versions(None, &HookVersion::new(Tag::from_str("1.0.0", None)?))
            .unwrap();

        assert_that!(hook.0.as_str())
            .is_equal_to("echo \"the latest 2.0.0-pre.alpha-bravo+build.42\"");
        Ok(())
    }

    #[sealed_test]
    fn parenthesis_in_hook_works() -> Result<()> {
        Repository::init(".")?;

        let mut hook = Hook::from_str("git commit --allow-empty -m 'chore(snapshot): bump snapshot to {{version+1patch-SNAPSHOT}}'")?;

        hook.insert_versions(None, &HookVersion::new(Tag::from_str("1.0.0", None)?))
            .unwrap();

        let outcome = hook.run();

        assert_that!(outcome).is_ok();

        Ok(())
    }
}
