use std::collections::VecDeque;

use anyhow::Result;
use semver::Version;

use crate::hook::Token;
use std::ops::Range;

pub const DELIMITER_START: &str = "{{";
pub const DELIMITER_END: &str = "}}";

#[derive(Debug, Eq, PartialEq)]
pub struct HookExpr {
    src: String,
    tokens: VecDeque<Token>,
}

impl Token {
    fn parse(src: &str) -> Result<(Token, &str)> {
        if let Some(remains) = src.strip_prefix("version") {
            Ok((Token::Version, remains))
        } else if let Some(remains) = src.strip_prefix("latest") {
            Ok((Token::LatestVersion, remains))
        } else if let Some(remains) = src.strip_prefix('+') {
            Ok((Token::Add, remains))
        } else if let Some(remains) = src.strip_prefix("major") {
            Ok((Token::Major, remains))
        } else if let Some(remains) = src.strip_prefix("minor") {
            Ok((Token::Minor, remains))
        } else if let Some(remains) = src.strip_prefix("patch") {
            Ok((Token::Patch, remains))
        } else if src[0..1].parse::<u32>().is_ok() {
            let mut position = 1;
            while src[position..position + 1].parse::<u32>().is_ok() {
                position += 1;
            }
            match src[0..position].parse::<u32>() {
                Ok(amount) => Ok((Token::Amount(amount), &src[position..])),
                Err(e) => Err(anyhow!("{}", e)),
            }
        } else {
            Ok((Token::AlphaNumeric(src.to_string()), ""))
        }
    }
}

impl HookExpr {
    pub fn parse_version(
        src: &str,
        current_version: Option<Version>,
        next_version: Version,
    ) -> Option<(Range<usize>, String)> {
        if let Some((range, mut expression)) = HookExpr::scan_hook_entry(src) {
            expression.tokenize();
            expression
                .calculate_version(current_version, next_version)
                .ok()
                .map(|exp| (range, exp))
        } else {
            None
        }
    }

    fn from_str(src: &str) -> Self {
        HookExpr {
            src: src.to_string(),
            tokens: VecDeque::new(),
        }
    }

    fn scan_hook_entry(hook_entry: &str) -> Option<(Range<usize>, HookExpr)> {
        match hook_entry.find(DELIMITER_START) {
            Some(start) => hook_entry.find(DELIMITER_END).map(|end| {
                let range = start..end + DELIMITER_END.len();
                let expression =
                    HookExpr::from_str(&hook_entry[start + DELIMITER_START.len()..end]);
                (range, expression)
            }),
            None => None,
        }
    }

    fn tokenize(&mut self) {
        let mut src = self.src.as_str();
        while !src.is_empty() {
            if let Ok((token, remains)) = Token::parse(src) {
                self.tokens.push_back(token);
                src = remains
            }
        }
    }

    fn increment_major(version: Version, amt: u32) -> Version {
        let mut version = version;
        for _ in 0..amt {
            version.major += 1
        }

        version
    }

    fn increment_patch(version: Version, amt: u32) -> Version {
        let mut version = version;
        for _ in 0..amt {
            version.patch += 1
        }

        version
    }

    fn increment_minor(version: Version, amt: u32) -> Version {
        let mut version = version;
        for _ in 0..amt {
            version.minor += 1
        }

        version
    }

    fn calculate_version(
        &mut self,
        current_version: Option<Version>,
        next_version: Version,
    ) -> Result<String> {
        ensure!(!self.tokens.is_empty(), "Hook expression must not be empty");

        let mut version = match self.tokens.pop_front() {
            Some(Token::Version) => Ok(next_version),
            Some(Token::LatestVersion) => current_version
                .ok_or_else(|| anyhow!("Not previous tag found to replace {{latest}} version")),
            _ => Err(anyhow!(
                "Hook expression must start with \"version\" or \"latest\""
            )),
        }?;

        while let Some(token) = self.tokens.pop_front() {
            match token {
                Token::Add => version = self.calculate_increment(version)?,
                Token::AlphaNumeric(string) => {
                    let output = [version.to_string(), string].join("");
                    return Ok(output);
                }
                _ => return Err(anyhow!("Unexpected token in hook expression:{:?}", token)),
            };
        }

        Ok(version.to_string())
    }

    fn parse_amount(&mut self) -> u32 {
        let amt = if let Some(Token::Amount(amt)) = self.tokens.get(0) {
            *amt
        } else {
            1
        };

        self.tokens.pop_front();
        amt
    }

    fn calculate_increment(&mut self, version: Version) -> Result<Version> {
        let amt = self.parse_amount();
        let token = self.tokens.pop_front();
        match token {
            None => Err(anyhow!("Missing token after operator")),
            Some(token) => match token {
                Token::Major => Ok(HookExpr::increment_major(version, amt)),
                Token::Minor => Ok(HookExpr::increment_minor(version, amt)),
                Token::Patch => Ok(HookExpr::increment_patch(version, amt)),
                _ => Err(anyhow!("Unexpected token in hook expression:{:?}", token)),
            },
        }
    }
}

#[cfg(test)]
mod tests {
    use std::collections::VecDeque;

    use semver::Version;

    use crate::hook::parser::HookExpr;
    use crate::hook::Token;
    use anyhow::Result;
    use speculoos::prelude::*;

    #[test]
    fn scan_exp() {
        let entry = "echo {{version+1major}}";

        let (range, expr) = HookExpr::scan_hook_entry(entry).unwrap();
        assert_that!(range).is_equal_to(5..23);

        assert_that!(expr).is_equal_to(HookExpr {
            src: "version+1major".to_string(),
            tokens: VecDeque::new(),
        })
    }

    #[test]
    fn tokenize_exp() {
        let entry = "echo {{version+minor}}";

        let (range, mut expr) = HookExpr::scan_hook_entry(entry).unwrap();
        expr.tokenize();
        assert_that!(range).is_equal_to(5..22);
        assert_that!(expr.tokens)
            .is_equal_to(&vec![Token::Version, Token::Add, Token::Minor].into())
    }

    #[test]
    fn tokenize_latest_version() {
        let entry = "echo {{latest}}";

        let (range, mut expr) = HookExpr::scan_hook_entry(entry).unwrap();
        expr.tokenize();
        assert_that!(range).is_equal_to(5..15);
        assert_that!(expr.tokens).is_equal_to(&vec![Token::LatestVersion].into())
    }

    #[test]
    fn tokenize_exp_with_amount() {
        let entry = "echo {{version+2major}}";

        let (range, mut expr) = HookExpr::scan_hook_entry(entry).unwrap();
        expr.tokenize();

        assert_that!(range).is_equal_to(5..23);
        assert_that!(expr.tokens)
            .is_equal_to(&vec![Token::Version, Token::Add, Token::Amount(2), Token::Major].into())
    }

    #[test]
    fn tokenize_exp_with_alpha() {
        let entry = "echo {{version+33patch-rc}}";

        let (range, mut expr) = HookExpr::scan_hook_entry(entry).unwrap();
        expr.tokenize();

        assert_that!(range).is_equal_to(5..27);
        assert_that!(expr.tokens).is_equal_to(
            &vec![
                Token::Version,
                Token::Add,
                Token::Amount(33),
                Token::Patch,
                Token::AlphaNumeric("-rc".to_string()),
            ]
            .into(),
        )
    }

    #[test]
    fn calculate_version() {
        let mut hookexpr = HookExpr {
            src: "echo {{version+33patch-rc}}".to_string(),
            tokens: VecDeque::from(vec![
                Token::Version,
                Token::Add,
                Token::Amount(33),
                Token::Patch,
                Token::AlphaNumeric("-rc".to_string()),
            ]),
        };

        let result = hookexpr.calculate_version(None, Version::new(1, 0, 0));
        assert_that!(result)
            .is_ok()
            .is_equal_to("1.0.33-rc".to_string());
    }

    #[test]
    fn increment_version() -> Result<()> {
        let version = Version::parse("0.0.0")?;
        let version = HookExpr::increment_major(version, 1);
        assert_that!(version.major).is_equal_to(1);

        let version = HookExpr::increment_minor(version, 2);
        assert_that!(version.minor).is_equal_to(2);

        let version = HookExpr::increment_patch(version, 5);
        assert_that!(version.patch).is_equal_to(5);

        Ok(())
    }
}
