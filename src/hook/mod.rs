use std::fmt;
use std::process::Command;
use std::str::FromStr;

use anyhow::Result;
use semver::Version;

use crate::hook::parser::HookExpr;

mod parser;

#[derive(Debug, Eq, PartialEq)]
enum Token {
    Version,
    LatestVersion,
    Amount(u32),
    Add,
    Major,
    Minor,
    Patch,
    AlphaNumeric(String),
}

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
    pub fn insert_versions(
        &mut self,
        current_version: Option<String>,
        next_version: &str,
    ) -> Result<()> {
        let next_version = Version::from_str(next_version)?;
        let current_version = current_version
            .map(|version| Version::from_str(&version))
            .map(Result::ok)
            .flatten();

        for i in 0..self.0.len() {
            if let Some((range, version)) =
                HookExpr::parse_version(&self.0[i], current_version.clone(), next_version.clone())
            {
                self.0[i].replace_range(range, &version);
            }
        }

        Ok(())
    }

    pub fn run(&self) -> Result<()> {
        let (cmd, args) = self.0.split_first().expect("hook must not be empty");

        let status = Command::new(&cmd).args(args).status()?;

        ensure!(status.success(), "hook failed with status {}", status);

        Ok(())
    }
}

#[cfg(test)]
mod test {
    use std::str::FromStr;

    use crate::Hook;
    use crate::Result;

    #[test]
    fn parse_empty_string() {
        assert!(Hook::from_str("").is_err())
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
        hook.insert_versions(None, "1.0.0").unwrap();

        assert_eq!(&hook.0, &["cargo", "bump", "1.0.0"]);
        Ok(())
    }

    #[test]
    fn replace_maven_version() -> Result<()> {
        let mut hook = Hook::from_str("mvn versions:set -DnewVersion={{version}}")?;
        hook.insert_versions(None, "1.0.0").unwrap();

        assert_eq!(&hook.0, &["mvn", "versions:set", "-DnewVersion=1.0.0"]);
        Ok(())
    }

    #[test]
    fn replace_maven_version_with_expression() -> Result<()> {
        let mut hook = Hook::from_str("mvn versions:set -DnewVersion={{version+1minor-SNAPSHOT}}")?;
        hook.insert_versions(None, "1.0.0").unwrap();

        assert_eq!(
            &hook.0,
            &["mvn", "versions:set", "-DnewVersion=1.1.0-SNAPSHOT"]
        );
        Ok(())
    }

    #[test]
    fn leave_hook_untouched_when_no_version() -> Result<()> {
        let mut hook = Hook::from_str("echo \"Hello World\"")?;
        hook.insert_versions(None, "1.0.0").unwrap();

        assert_eq!(&hook.0, &["echo", "Hello World"]);
        Ok(())
    }

    #[test]
    fn replace_quoted_version() -> Result<()> {
        let mut hook = Hook::from_str("echo \"{{version}}\"")?;
        hook.insert_versions(None, "1.0.0").unwrap();

        assert_eq!(&hook.0, &["echo", "1.0.0"]);
        Ok(())
    }

    #[test]
    fn replace_version_with_nested_simple_quoted_arg() -> Result<()> {
        let mut hook = Hook::from_str("coco chore 'bump snapshot to {{version+1minor-pre}}'")?;
        hook.insert_versions(None, "1.0.0").unwrap();

        assert_eq!(&hook.0, &["coco", "chore", "bump snapshot to 1.1.0-pre"]);
        Ok(())
    }

    #[test]
    fn replace_version_with_nested_double_quoted_arg() -> Result<()> {
        let mut hook = Hook::from_str("coco chore \"bump snapshot to {{version+1minor-pre}}\"")?;
        hook.insert_versions(None, "1.0.0").unwrap();

        assert_eq!(&hook.0, &["coco", "chore", "bump snapshot to 1.1.0-pre"]);
        Ok(())
    }
}
