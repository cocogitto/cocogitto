use config::ConfigError;
use serde::de::StdError;
use std::fmt::{Debug, Display, Formatter};

#[derive(Debug)]
pub(crate) struct SettingError(config::ConfigError);

impl Display for SettingError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "Config format error: {}", self.0)
    }
}

impl From<ConfigError> for SettingError {
    fn from(err: ConfigError) -> Self {
        SettingError(err)
    }
}

impl StdError for SettingError {}
