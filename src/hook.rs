use std::{fmt, process::Command, str::FromStr};

use crate::Result;

static ENTRY_SYMBOL: char = '%';

pub struct Hook(Vec<String>);

impl FromStr for Hook {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s.is_empty() {
            bail!("hook must not be an empty string")
        }

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
    pub fn entries(&mut self) -> impl Iterator<Item = HookEntry> {
        self.0
            .iter_mut()
            .filter(|s| s.starts_with(ENTRY_SYMBOL))
            .map(HookEntry)
    }

    pub fn is_ready(&self) -> bool {
        !self.0.iter().any(|s| s.starts_with(ENTRY_SYMBOL))
    }

    pub fn run(&self) -> Result<()> {
        // Hook should have all entries filled before running
        assert!(self.is_ready());

        let (cmd, args) = self.0.split_first().expect("hook must not be empty");

        let status = Command::new(&cmd).args(args).status()?;

        if !status.success() {
            Err(anyhow!("hook failed with status {}", status))
        } else {
            Ok(())
        }
    }
}

pub struct HookEntry<'a>(&'a mut String);

impl HookEntry<'_> {
    pub fn fill<'b, F>(&mut self, f: F) -> Result<()>
    where
        F: FnOnce(&str) -> Option<&'b str> + 'b,
    {
        // trim ENTRY_SYMBOL in the beginning
        let key = &self.0[1..];

        let value = f(key).ok_or_else(|| anyhow!("unknown key {}", key))?;

        self.0.clear();
        self.0.push_str(value);

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
    fn fill_entries() -> Result<()> {
        let mut hook = Hook::from_str("cmd %one %two %three")?;

        assert!(!hook.is_ready());

        hook.entries().try_for_each(|mut entry| {
            entry.fill(|key| match key {
                "one" => Some("1"),
                "two" => Some("2"),
                "three" => Some("3"),
                _ => None,
            })
        })?;

        assert!(hook.is_ready());

        assert_eq!(
            &hook.0,
            &["cmd".to_string(), "1".into(), "2".into(), "3".into()]
        );

        Ok(())
    }

    #[test]
    fn fill_entries_unknown_key() -> Result<()> {
        let mut hook = Hook::from_str("%unknown")?;

        assert!(!hook.is_ready());

        let result = hook
            .entries()
            .try_for_each(|mut entry| entry.fill(|_| None));
        assert!(result.is_err());

        assert!(!hook.is_ready());

        Ok(())
    }
}
