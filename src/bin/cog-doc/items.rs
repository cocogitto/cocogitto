#[derive(Debug)]
pub enum FieldOrVariant {
    Struct(StructField),
    Enum(EnumVariant),
}

#[derive(Debug)]
pub struct StructField {
    pub parent_name: String,
    pub parent_docs: Option<String>,
    pub field: Field,
}

#[derive(Debug)]
pub struct EnumVariant {
    pub parent_doc: Option<String>,
    pub docs: Option<String>,
    pub variant_name: String,
    pub fields: Vec<Field>,
    pub enum_name: String,
}

#[derive(Debug)]
pub struct Field {
    pub name: String,
    pub type_name: String,
    pub docs: Option<String>,
}

impl StructField {
    pub(crate) fn maybe_type_parameter(&self) -> Vec<String> {
        let mut additional_types = vec![];

        if let Some(types) = self.field.type_name.strip_prefix("Map<") {
            let types = types.strip_suffix(">").unwrap();
            let types: Vec<String> = types.split(',').map(|v| v.trim().to_string()).collect();
            additional_types.extend(types);
        } else if let Some(types) = self.field.type_name.strip_prefix("Option<") {
            let t = types.strip_suffix(">").unwrap();
            additional_types.push(t.to_string());
        } else if let Some(types) = self.field.type_name.strip_prefix("Array<") {
            let t = types.strip_suffix(">").unwrap();
            additional_types.push(t.to_string());
        }

        additional_types
    }
}

pub fn type_to_documentation(t: &str) -> String {
    t.replace("Vec<", "Array<")
        .replace("std::collections::", "")
        .replace("std::path::", "")
        .replace("crate::", "")
        .replace("HashMap", "Map")
}