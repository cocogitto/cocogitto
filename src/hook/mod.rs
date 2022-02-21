mod error;
mod parser;

use std::collections::VecDeque;
use std::fmt;
use std::ops::Range;
use std::process::Command;
use std::str::FromStr;

use semver::Version;

use crate::SETTINGS;
use parser::Token;

use anyhow::{anyhow, ensure, Result};
use itertools::Itertools;

#[derive(Debug, Eq, PartialEq)]
pub struct VersionSpan {
    range: Range<usize>,
    tokens: VecDeque<Token>,
}

pub(crate) struct HookVersion {
    pub prefixed_tag: String,
}

impl HookVersion {
    pub(crate) fn new(tag: &str) -> Self {
        HookVersion {
            prefixed_tag: tag.to_string(),
        }
    }
    pub(crate) fn to_version(&self) -> Result<Version> {
        match SETTINGS.tag_prefix.as_ref() {
            Some(prefix) => {
                if self.prefixed_tag.starts_with(prefix) {
                    let version = self.prefixed_tag.strip_prefix(prefix);
                    Version::parse(version.unwrap())
                } else {
                    Version::parse(self.prefixed_tag.as_ref())
                }
            }
            None => Version::parse(self.prefixed_tag.as_ref()),
        }
        .map_err(|err| anyhow!("{}", err))
    }
}

impl VersionSpan {
    pub(crate) fn build_version_str(
        &mut self,
        version: &HookVersion,
        latest: Option<&HookVersion>,
    ) -> Result<String> {
        let version = version.to_version()?;
        let latest = latest
            .map(|version| version.to_version())
            .and_then(Result::ok);

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
pub struct Hook(Vec<String>);

impl FromStr for Hook {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        ensure!(!s.is_empty(), "hook must not be an empty string");
        Ok(Hook(shell_words::split(s)?))
    }
}

impl fmt::Display for Hook {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let command = shell_words::join(&self.0);
        f.write_str(&command)
    }
}

impl Hook {
    pub(crate) fn insert_versions(
        &mut self,
        current_version: Option<&HookVersion>,
        next_version: &HookVersion,
    ) -> Result<()> {
        let parts = self
            .0
            .iter()
            .map(|part| parser::parse(part))
            .map(Result::unwrap)
            .map(|mut span| span.replace_versions(next_version, current_version))
            .map(Result::unwrap)
            .collect();

        self.0 = parts;

        Ok(())
    }

    pub fn run(&self) -> Result<()> {
        let (cmd, args) = self.0.split_first().expect("hook must not be empty");

        #[cfg(target_family = "unix")]
        {
            let args = args.iter().join(" ");
            let cmd = format!("{cmd} {args}");

            let status = Command::new("sh").arg("-c").arg(cmd).status()?;

            ensure!(status.success(), "hook failed with status {}", status);
        }

        #[cfg(target_family = "windows")]
        {
            let status = Command::new(cmd).args(args).status()?;

            ensure!(status.success(), "hook failed with status {}", status);
        }

        Ok(())
    }
}

#[cfg(test)]
mod test {
    use std::str::FromStr;

    use crate::{Hook, HookVersion, Result};

    use speculoos::prelude::*;

    #[test]
    fn parse_empty_string() {
        let empty_hook = Hook::from_str("");
        assert_that!(empty_hook).is_err();
    }

    #[test]
    fn parse_valid_string() -> Result<()> {
        let hook = Hook::from_str("cargo bump {{version}}")?;
        assert_eq!(&hook.0, &["cargo", "bump", "{{version}}"]);
        Ok(())
    }

    #[test]
    fn replace_version_cargo() -> Result<()> {
        let mut hook = Hook::from_str("cargo bump {{version}}")?;
        hook.insert_versions(None, &HookVersion::new("1.0.0"))
            .unwrap();

        assert_that!(hook.0).contains_all_of(&vec![
            &"cargo".to_string(),
            &"bump".to_string(),
            &"1.0.0".to_string(),
        ]);
        Ok(())
    }

    #[test]
    fn replace_maven_version() -> Result<()> {
        let mut hook = Hook::from_str("mvn versions:set -DnewVersion={{version}}")?;
        hook.insert_versions(None, &HookVersion::new("1.0.0"))
            .unwrap();

        assert_eq!(&hook.0, &["mvn", "versions:set", "-DnewVersion=1.0.0"]);
        Ok(())
    }

    #[test]
    fn replace_maven_version_with_expression() -> Result<()> {
        let mut hook = Hook::from_str("mvn versions:set -DnewVersion={{version+1minor-SNAPSHOT}}")?;
        hook.insert_versions(None, &HookVersion::new("1.0.0"))
            .unwrap();

        assert_eq!(
            &hook.0,
            &["mvn", "versions:set", "-DnewVersion=1.1.0-SNAPSHOT"]
        );
        Ok(())
    }

    #[test]
    fn leave_hook_untouched_when_no_version() -> Result<()> {
        let mut hook = Hook::from_str("echo \"Hello World\"")?;
        hook.insert_versions(None, &HookVersion::new("1.0.0"))
            .unwrap();

        assert_eq!(&hook.0, &["echo", "Hello World"]);
        Ok(())
    }

    #[test]
    fn replace_quoted_version() -> Result<()> {
        let mut hook = Hook::from_str("echo \"{{version}}\"")?;
        hook.insert_versions(None, &HookVersion::new("1.0.0"))
            .unwrap();

        assert_eq!(&hook.0, &["echo", "1.0.0"]);
        Ok(())
    }

    #[test]
    fn replace_version_with_nested_simple_quoted_arg() -> Result<()> {
        let mut hook = Hook::from_str("coco chore 'bump snapshot to {{version+1minor-pre}}'")?;
        hook.insert_versions(None, &HookVersion::new("1.0.0"))
            .unwrap();

        assert_eq!(&hook.0, &["coco", "chore", "bump snapshot to 1.1.0-pre"]);
        Ok(())
    }

    #[test]
    fn replace_version_with_nested_double_quoted_arg() -> Result<()> {
        let mut hook = Hook::from_str("coco chore \"bump snapshot to {{version+1minor-pre}}\"")?;
        hook.insert_versions(None, &HookVersion::new("1.0.0"))
            .unwrap();

        assert_eq!(&hook.0, &["coco", "chore", "bump snapshot to 1.1.0-pre"]);
        Ok(())
    }

    #[test]
    fn replace_version_with_multiple_placeholders() -> Result<()> {
        let mut hook = Hook::from_str("echo \"the latest {{latest}}, the greatest {{version}}\"")?;
        hook.insert_versions(Some(&HookVersion::new("0.5.9")), &HookVersion::new("1.0.0"))
            .unwrap();

        assert_eq!(&hook.0, &["echo", "the latest 0.5.9, the greatest 1.0.0"]);
        Ok(())
    }

    #[test]
    fn replace_version_with_multiple_placeholders_and_increments() -> Result<()> {
        let mut hook = Hook::from_str(
            "echo \"the latest {{latest+3major+1minor}}, the greatest {{version+2patch}}\"",
        )?;
        hook.insert_versions(Some(&HookVersion::new("0.5.9")), &HookVersion::new("1.0.0"))
            .unwrap();

        assert_eq!(&hook.0, &["echo", "the latest 3.1.0, the greatest 1.0.2"]);
        Ok(())
    }

    #[test]
    fn replace_version_with_pre_and_build_metadata() -> Result<()> {
        let mut hook =
            Hook::from_str("echo \"the latest {{version+1major-pre.alpha-bravo+build.42}}\"")?;
        hook.insert_versions(None, &HookVersion::new("1.0.0"))
            .unwrap();

        assert_eq!(
            &hook.0,
            &["echo", "the latest 2.0.0-pre.alpha-bravo+build.42"]
        );
        Ok(())
    }
}
