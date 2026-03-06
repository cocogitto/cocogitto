use config::ConfigError;
use serde::de::StdError;
use std::fmt::{Debug, Display, Formatter};

#[derive(Debug)]
pub enum SettingError {
    Config(ConfigError),
    Git(git2::Error),
}

impl Display for SettingError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            SettingError::Config(err) => {
                writeln!(f, "failed to parse config\n")?;
                writeln!(f, "\tcause: {}", err)
            }
            SettingError::Git(err) => {
                writeln!(f, "failed to open repository\n")?;
                writeln!(f, "\tcause: {}", err)
            }
        }
    }
}

impl From<ConfigError> for SettingError {
    fn from(err: ConfigError) -> Self {
        SettingError::Config(err)
    }
}

impl From<git2::Error> for SettingError {
    fn from(err: git2::Error) -> Self {
        SettingError::Git(err)
    }
}

impl StdError for SettingError {}
