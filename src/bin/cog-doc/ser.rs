use crate::items::FieldOrVariant;

pub struct Item {
    pub(crate) name: String,
    pub(crate) values: Vec<FieldOrVariant>,
}

impl Item {
    pub fn to_markdown(&self) -> String {
        let mut out = String::new();
        out.push_str(&format!("## {} \n", self.name));
        for i in &self.values {
            out.push_str(&i.to_markdown());
        }

        out
    }
}

impl FieldOrVariant {
    pub fn to_markdown(&self) -> String {
        match self {
            FieldOrVariant::Struct(field) => {
                let mut md = format!(
                    r#"
### `{name}`
- type: {typ}
"#,
                    name = field.field.name,
                    typ = field.field.type_name);

                if let Some(doc)= &field.field.docs {
                    md.push_str(&format!("- description: {}\n", doc));
                }

                md
            }
            FieldOrVariant::Enum(_variant) => "".to_string()
        }
    }
}