use std::path::PathBuf;

use serde::{Deserialize, Serialize};

#[cfg_attr(feature = "json_schema", derive(schemars::JsonSchema))]
#[derive(Debug, Deserialize, Serialize, Clone, Eq, PartialEq)]
#[serde(deny_unknown_fields, default)]
pub struct Changelog {
    pub template: Option<String>,
    pub package_template: Option<String>,
    pub remote: Option<String>,
    pub path: PathBuf,
    pub owner: Option<String>,
    pub repository: Option<String>,
    pub authors: Vec<AuthorSetting>,
}

impl Default for Changelog {
    fn default() -> Self {
        Changelog {
            template: None,
            package_template: None,
            remote: None,
            path: PathBuf::from("CHANGELOG.md"),
            owner: None,
            repository: None,
            authors: vec![],
        }
    }
}

#[cfg_attr(feature = "json_schema", derive(schemars::JsonSchema))]
#[derive(Debug, Deserialize, Serialize, Clone, Eq, PartialEq)]
#[serde(deny_unknown_fields)]
pub struct AuthorSetting {
    pub signature: String,
    pub username: String,
}
