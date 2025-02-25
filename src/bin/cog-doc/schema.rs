use cocogitto::settings::Settings;
use itertools::Itertools;
use schemars::schema_for;
use serde::Deserialize;
use serde_json::Value;
use std::collections::BTreeMap;
use std::fmt;
use std::fmt::Formatter;

pub fn root_schema() -> anyhow::Result<Schema> {
    let schema = schema_for!(Settings);
    let json = serde_json::to_string(&schema.to_value())?;
    serde_json::from_str(&json).map_err(anyhow::Error::from)
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
#[allow(unused)] // We want exhaustive deserialization rather that exhaustive usage
pub struct Schema {
    #[serde(rename = "$schema")]
    pub _schema: String,
    pub title: Option<String>,
    pub description: String,
    pub properties: Option<BTreeMap<String, Property>>,
    #[serde(rename = "type")]
    pub _type: Option<Type>,
    pub additional_properties: Option<AdditionalProperties>,
    #[serde(rename = "$defs")]
    pub definitions: Option<BTreeMap<String, Property>>,
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
#[allow(unused)]
pub struct Property {
    #[serde(skip)]
    pub property_key: Option<String>,
    pub title: Option<String>,
    pub description: String,
    pub required: Option<Vec<String>>,
    pub properties: Option<BTreeMap<String, Property>>,
    pub r#type: Option<Type>,
    pub additional_properties: Option<AdditionalProperties>,
    #[serde(rename = "$ref")]
    pub reference: Option<String>,
    pub default: Option<Value>,
    pub items: Option<AnyOf>,
    pub any_of: Option<Vec<AnyOf>>,
    pub r#enum: Option<Vec<String>>,
    #[serde(rename = "format")]
    pub _format: Option<String>,
    #[serde(rename = "minimum")]
    pub _minimum: Option<u64>,
    pub property_names: Option<AnyOf>,
}

#[derive(Deserialize, Debug)]
#[serde(untagged)]
pub enum Type {
    Unique(String),
    Multiple(Vec<String>),
}

impl fmt::Display for Type {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            Type::Unique(t) => write!(f, "{}", capitalize_first_letter(t))?,
            Type::Multiple(types) => write!(
                f,
                "{}",
                types
                    .iter()
                    .map(|t| capitalize_first_letter(&t))
                    .join(" | ")
            )?,
        };
        Ok(())
    }
}

#[derive(Deserialize, Debug)]
#[serde(untagged)]
#[allow(unused)]
pub enum AdditionalProperties {
    False(bool),
    Ref {
        #[serde(rename = "$ref")]
        reference: String,
    },
    AnyOf {
        #[serde(rename = "anyOf")]
        any_of: Vec<AnyOf>,
    },
}

#[derive(Deserialize, Debug, Clone)]
#[serde(untagged)]
pub enum AnyOf {
    Type {
        r#type: String,
    },
    Ref {
        #[serde(rename = "$ref")]
        reference: String,
    },
}

impl Into<Type> for AnyOf {
    fn into(self) -> Type {
        match self {
            AnyOf::Type { r#type } => Type::Unique(r#type),
            AnyOf::Ref { reference } => Type::Unique(reference_stripped(&reference).to_string()),
        }
    }
}

pub(crate) fn reference_link(reference: &str) -> String {
    let reference = reference.strip_prefix("#/$defs/");
    let reference = reference.unwrap();
    format!("[{reference}](#{reference})")
}

pub(crate) fn reference_stripped(reference: &str) -> String {
    let reference = reference.strip_prefix("#/$defs/");
    let reference = reference.unwrap();
    format!("{reference}")
}

fn capitalize_first_letter(s: &str) -> String {
    let mut chars = s.chars();
    match chars.next() {
        Some(first) => first.to_uppercase().collect::<String>() + chars.as_str(),
        None => String::new(),
    }
}

#[cfg(test)]
mod tests {
    use crate::schema::root_schema;
    use speculoos::prelude::*;

    #[test]
    fn should_parse_root_settings() {
        let root = root_schema();
        assert_that!(root).is_ok();
    }
}
