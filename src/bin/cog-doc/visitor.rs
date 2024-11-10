use crate::items::{EnumVariant, Field, FieldOrVariant, StructField};
use crate::query;
use crate::query::{ENUM_QUERY, STRUCT_QUERY};
use std::collections::BTreeMap;
use std::sync::Arc;
use trustfall::FieldValue;
use trustfall_rustdoc::VersionedRustdocAdapter;

pub fn visit_enum(result: Vec<BTreeMap<Arc<str>, FieldValue>>) -> Vec<FieldOrVariant> {
    let mut items = vec![];
    for res in result {
        println!("{:?}", res);

        let enum_name = res.get("enum_name")
            .and_then(|v| v.as_str())
            .map(ToString::to_string)
            .unwrap();

        let variant_name = res.get("variant_name")
            .and_then(|v| v.as_str())
            .map(ToString::to_string)
            .unwrap();

        let parent_doc = res.get("enum_doc")
            .and_then(|v| v.as_str())
            .map(ToString::to_string);

        let docs = res.get("variant_docs")
            .and_then(|v| v.as_str())
            .map(ToString::to_string);

        let item = EnumVariant {
            parent_doc,
            docs,
            variant_name,
            fields: vec![],
            enum_name,
        };

        items.push(FieldOrVariant::Enum(item));
    }

    items
}

pub fn visit_struct(adapter: &VersionedRustdocAdapter, result: Vec<BTreeMap<Arc<str>, FieldValue>>) -> Vec<FieldOrVariant> {
    let mut items = vec![];
    for res in result {
        let docs = res.get("docs")
            .and_then(|v| v.as_str())
            .map(ToString::to_string);

        let field_name = res.get("field_name")
            .and_then(|v| v.as_str())
            .map(ToString::to_string)
            .unwrap();

        let type_name = res.get("type_name")
            .and_then(|v| v.as_str())
            .map(crate::items::type_to_documentation)
            .unwrap();

        let parent_name = res.get("struct_name")
            .and_then(|v| v.as_str())
            .map(ToString::to_string)
            .unwrap();

        let parent_docs = res.get("struct_doc")
            .and_then(|v| v.as_str())
            .map(ToString::to_string);

        let item = StructField {
            field: Field {
                name: field_name,
                type_name,
                docs
            },
            parent_name,
            parent_docs,
        };

        if item.field.type_name != "String" && item.field.type_name != "bool" {
            let result = query::trustfall_query(&adapter, &item.field.type_name, STRUCT_QUERY);
            let additional_items = visit_struct(&adapter, result);
            items.extend(additional_items);

            let result = query::trustfall_query(&adapter, &item.field.type_name, ENUM_QUERY);
            let additional_items = visit_enum(result);
            items.extend(additional_items);
        }

        let mut additional_types = item.maybe_type_parameter();
        additional_types.retain(|t| t != "String" && t != "bool");
        for additional in additional_types {
            let result = query::trustfall_query(&adapter, &additional, STRUCT_QUERY);
            let additional_items = visit_struct(&adapter, result);
            items.extend(additional_items);

            let result = query::trustfall_query(&adapter, &additional, ENUM_QUERY);
            let additional_items = visit_enum(result);
            items.extend(additional_items);
        }

        items.push(FieldOrVariant::Struct(item));
    }

    items
}