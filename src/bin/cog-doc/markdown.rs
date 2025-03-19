use crate::schema::{
    reference_link, reference_stripped, AdditionalProperties, AnyOf, Property, Schema, Type,
};
use itertools::Itertools;
use serde_json::Value as JsonValue;
use std::io::Write;
use toml::map::Map;
use toml::Value as TomlValue;

pub trait ToMarkDown {
    fn to_markdown(&mut self, out: &mut impl Write) -> anyhow::Result<()>;
}

impl ToMarkDown for Schema {
    fn to_markdown(&mut self, out: &mut impl Write) -> anyhow::Result<()> {
        if let Some(title) = &self.title {
            writeln!(out, "## `{}`", title)?;
        };

        writeln!(out, "- **Description :** {}", self.description)?;

        if let Some(definitions) = &mut self.properties {
            for (name, mut definition) in definitions {
                definition.property_key = Some(name.clone());
                writeln!(out, "## `{}`", name)?;
                definition.to_markdown(out)?;
            }
        }

        if let Some(definitions) = &mut self.definitions {
            for (name, mut definition) in definitions {
                writeln!(out, "## {}", name)?;
                definition.to_markdown(out)?
            }
        }

        writeln!(out)?;
        Ok(())
    }
}

impl ToMarkDown for &mut Property {
    fn to_markdown(&mut self, out: &mut impl Write) -> anyhow::Result<()> {
        writeln!(out, "- **Description :** {}", &self.description)?;
        match &self.reference {
            None => match &mut self.r#type {
                Some(Type::Unique(t)) if t == "object" => {
                    let key = self
                        .property_names
                        .as_ref()
                        .map(|a| a.clone().into())
                        .unwrap_or(Type::Unique("String".to_string()));

                    match &mut self.additional_properties {
                        Some(AdditionalProperties::AnyOf { any_of }) => {
                            let types: Vec<Type> =
                                any_of.clone().into_iter().map(Into::into).collect();
                            let types = types.iter().map(|t| format!("{t}")).join(" | ");
                            Type::Unique(format!("Map<{key}, {types}>")).to_markdown(out)?
                        }
                        Some(AdditionalProperties::Ref { reference }) => {
                            Type::Unique(format!("Map<{key}, {}>", reference_stripped(reference)))
                                .to_markdown(out)?
                        }
                        _ => {}
                    }
                }
                Some(t) => t.to_markdown(out)?,
                None => {}
            },
            Some(reference) => {
                let link = reference_link(reference);
                writeln!(out, "- **Type :** {}", link)?;
            }
        };

        let required = self.required.clone().unwrap_or_default();

        if let Some(definitions) = &mut self.properties {
            for (name, mut definition) in definitions {
                definition.property_key = Some(name.clone());
                if required.contains(name) {
                    writeln!(
                        out,
                        r####"### `{}` <Badge type="danger" text="required" />"####,
                        name
                    )?;
                } else {
                    writeln!(out, "### `{}`", name)?;
                }
                definition.to_markdown(out)?;
            }
        }

        if let Some(default) = &self.default.as_ref().and_then(json_to_toml) {
            let mut table = Map::new();
            let key = self.property_key.as_ref().unwrap();
            table.insert(key.clone(), default.clone());
            let default = toml::Value::Table(table);
            let default = toml::to_string_pretty(&default)?;
            writeln!(out, "- **Default :**")?;
            writeln!(out, "```toml\n{default}```")?;
        }

        if let Some(enum_values) = &self.r#enum {
            let values = enum_values
                .iter()
                .map(|value| format!("`{value}`"))
                .join(", ");

            writeln!(out, "- **Possible values :** {}", values)?;
        }

        if let Some(property_name) = &mut self.property_names {
            property_name.to_markdown(out)?;
        }

        if let Some(items) = &mut self.items {
            items.to_markdown(out)?;
        }

        writeln!(out)?;
        Ok(())
    }
}
impl ToMarkDown for Type {
    fn to_markdown(&mut self, out: &mut impl Write) -> anyhow::Result<()> {
        write!(out, "- **Type :** ")?;
        write!(out, "`{self}`")?;
        writeln!(out)?;
        Ok(())
    }
}

impl ToMarkDown for AnyOf {
    fn to_markdown(&mut self, out: &mut impl Write) -> anyhow::Result<()> {
        match self {
            AnyOf::Type { r#type } => Type::Unique(r#type.clone()).to_markdown(out)?,
            AnyOf::Ref { reference } => writeln!(out, "ref {}", reference)?,
        };

        Ok(())
    }
}

fn json_to_toml(json: &JsonValue) -> Option<TomlValue> {
    match json {
        JsonValue::Null => Some(TomlValue::String("null".to_string())), // TOML has no null
        JsonValue::Bool(b) => Some(TomlValue::Boolean(*b)),
        JsonValue::Number(n) => n
            .as_i64()
            .map(TomlValue::Integer)
            .or(n.as_f64().map(TomlValue::Float)),
        JsonValue::String(s) => Some(TomlValue::String(s.clone())),
        JsonValue::Array(arr) => {
            let toml_arr: Vec<TomlValue> = arr.iter().filter_map(json_to_toml).collect();
            Some(TomlValue::Array(toml_arr))
        }
        JsonValue::Object(obj) => {
            let mut toml_map = Map::new();
            for (key, value) in obj {
                if let Some(toml_value) = json_to_toml(value) {
                    toml_map.insert(key.clone(), toml_value);
                }
            }
            Some(TomlValue::Table(toml_map))
        }
    }
}

#[cfg(test)]
mod test {
    use crate::markdown::ToMarkDown;
    use crate::schema::root_schema;
    use std::fs;
    use std::io::BufWriter;

    #[test]
    fn test() -> anyhow::Result<()> {
        let mut root = root_schema()?;
        let out = vec![];
        let mut writer = BufWriter::new(out);
        root.to_markdown(&mut writer)?;
        fs::write("website/reference/config.md", writer.into_inner()?)?;
        Ok(())
    }
}
