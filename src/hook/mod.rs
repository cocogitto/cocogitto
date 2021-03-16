use std::fmt;
use std::process::Command;
use std::str::FromStr;

use anyhow::Result;
use semver::Version;

use crate::hook::parser::{HookExpr, DELIMITER_END, DELIMITER_START};

mod parser;

#[derive(Debug, Eq, PartialEq)]
enum Token {
    Version,
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

        let words = shell_words::split(s)?;
        let mut output = vec![];
        for word in words {
            let delimiters = (word.find(DELIMITER_START), word.find(DELIMITER_END));

            if let (Some(start), Some(end)) = delimiters {
                let before_exp = &word[0..start];
                let after_exp = &word[end + DELIMITER_END.len()..word.len()];

                if !before_exp.is_empty() {
                    output.push(before_exp.to_string());
                }

                output.push(word[start..end + DELIMITER_END.len()].to_string());

                if !after_exp.is_empty() {
                    output.push(after_exp.to_string());
                }
            } else {
                output.push(word);
            }
        }

        Ok(Hook(output))
    }
}

impl fmt::Display for Hook {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let command = shell_words::join(self.0.iter());
        f.write_str(&command)
    }
}

impl Hook {
    pub fn insert_version(&mut self, value: &str) -> Result<()> {
        for i in 0..self.0.len() {
            if let Some(version) = HookExpr::parse(&self.0[i], Version::from_str(value)?) {
                self.0[i] = version
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
        assert_eq!(
            &hook.0,
            &["cargo".to_string(), "bump".into(), "{{version}}".into()]
        );
        Ok(())
    }

    #[test]
    fn replace_version_cargo() -> Result<()> {
        let mut hook = Hook::from_str("cargo bump {{version}}")?;
        hook.insert_version("1.0.0").unwrap();

        assert_eq!(
            &hook.0,
            &["cargo".to_string(), "bump".into(), "1.0.0".into()]
        );
        Ok(())
    }

    #[test]
    fn replace_maven_version() -> Result<()> {
        let mut hook = Hook::from_str("mvn versions:set -DnewVersion={{version}}")?;
        hook.insert_version("1.0.0").unwrap();

        assert_eq!(
            &hook.0,
            &[
                "mvn".to_string(),
                "versions:set".into(),
                "-DnewVersion=".into(),
                "1.0.0".into()
            ]
        );
        Ok(())
    }

    #[test]
    fn replace_maven_version_with_expression() -> Result<()> {
        let mut hook = Hook::from_str("mvn versions:set -DnewVersion={{version+1minor-SNAPSHOT}}")?;
        hook.insert_version("1.0.0").unwrap();

        assert_eq!(
            &hook.0,
            &[
                "mvn".to_string(),
                "versions:set".into(),
                "-DnewVersion=".into(),
                "1.1.0-SNAPSHOT".into()
            ]
        );
        Ok(())
    }

    #[test]
    fn leave_hook_untouched_when_no_version() -> Result<()> {
        let mut hook = Hook::from_str("echo \"Hello World\"")?;
        hook.insert_version("1.0.0").unwrap();

        assert_eq!(&hook.0, &["echo".to_string(), "Hello World".into()]);
        Ok(())
    }

    #[test]
    fn replace_quoted_version() -> Result<()> {
        let mut hook = Hook::from_str("echo \"{{version}}\"")?;
        hook.insert_version("1.0.0").unwrap();

        assert_eq!(&hook.0, &["echo".to_string(), "1.0.0".into()]);
        Ok(())
    }
}
