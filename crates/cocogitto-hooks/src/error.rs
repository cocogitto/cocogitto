use std::error::Error;
use std::fmt::{Display, Formatter};

#[derive(Debug)]
pub struct HookParseError {
    error: Box<dyn Error + Sync + Send>,
}

impl From<semver::Error> for HookParseError {
    fn from(err: semver::Error) -> Self {
        Self {
            error: Box::new(err),
        }
    }
}

impl From<pest::error::Error<crate::parser::Rule>> for HookParseError {
    fn from(err: pest::error::Error<crate::parser::Rule>) -> Self {
        Self {
            error: Box::new(err),
        }
    }
}

impl Display for HookParseError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "failed to parse bump hook\n")?;
        writeln!(f, "\tcause: {}", self.error)
    }
}

impl Error for HookParseError {}
