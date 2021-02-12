use std::{fmt, process::Command, str::FromStr};

use crate::Result;

pub const VERSION_KEY: &str = "%version";

pub struct Hook(Vec<String>);

impl FromStr for Hook {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        ensure!(!s.is_empty(), "hook must not be an empty string");

        let words = shell_words::split(s)?;

        Ok(Hook(words))
    }
}

impl fmt::Display for Hook {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let command = shell_words::join(self.0.iter());
        f.write_str(&command)
    }
}

impl Hook {
    pub fn insert_version(&mut self, value: &str) {
        for entry in &mut self.0 {
            *entry = entry.replace(VERSION_KEY, value);
        }
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
    use super::Hook;
    use crate::Result;
    use std::str::FromStr;

    #[test]
    fn parse_empty_string() {
        assert!(Hook::from_str("").is_err())
    }

    #[test]
    fn parse_valid_string() -> Result<()> {
        let hook = Hook::from_str("cargo bump %version")?;
        assert_eq!(
            &hook.0,
            &["cargo".to_string(), "bump".into(), "%version".into()]
        );
        Ok(())
    }

    #[test]
    fn replace_version_cargo() -> Result<()> {
        let mut hook = Hook::from_str("cargo bump %version")?;
        hook.insert_version("1.0.0");

        assert_eq!(
            &hook.0,
            &["cargo".to_string(), "bump".into(), "1.0.0".into()]
        );
        Ok(())
    }

    #[test]
    fn replace_version_maven() -> Result<()> {
        let mut hook = Hook::from_str("mvn versions:set -DnewVersion=%version")?;
        hook.insert_version("1.0.0");

        assert_eq!(
            &hook.0,
            &[
                "mvn".to_string(),
                "versions:set".into(),
                "-DnewVersion=1.0.0".into()
            ]
        );
        Ok(())
    }

    #[test]
    fn leave_hook_untouched_when_no_version() -> Result<()> {
        let mut hook = Hook::from_str("echo \"Hello World\"")?;
        hook.insert_version("1.0.0");

        assert_eq!(&hook.0, &["echo".to_string(), "Hello World".into()]);
        Ok(())
    }

    #[test]
    fn replace_quoted_version() -> Result<()> {
        let mut hook = Hook::from_str("echo \"%version\"")?;
        hook.insert_version("1.0.0");

        assert_eq!(&hook.0, &["echo".to_string(), "1.0.0".into()]);
        Ok(())
    }
}
