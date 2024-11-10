use trustfall_rustdoc::VersionedRustdocAdapter;
use std::collections::BTreeMap;
use std::sync::Arc;
use trustfall::FieldValue;

pub const STRUCT_QUERY: &'static str = r#"query {
  Crate {
    item {
      ... on Struct {
       struct_name: name @filter(op: "=", value: ["$struct_name"]) @output
       struct_doc: docs @output

        field {
          field_name: name @output
          docs @output
          raw_type {
            type_name: name @output
          }
        }
      }
    }
  }
}"#;
pub const ENUM_QUERY: &str = r#"
{
    Crate {
        item {
            ... on Enum {
                enum_name: name @filter(op: "=", value: ["$struct_name"]) @output
                enum_doc: docs @output
                variant {
                    variant_docs: docs @output
                    variant_name: name @output
                    field @optional {
                        field_name: name @output
                        docs @output
                        raw_type {
                            type_name: name @output
                        }
                    }
                }
            }
        }
    }
}
"#;

pub fn trustfall_query(adapter: &VersionedRustdocAdapter, struct_name: &str, query: &str) -> Vec<BTreeMap<Arc<str>, FieldValue>> {
    let mut vars = BTreeMap::new();
    vars.insert("struct_name", struct_name);
    let result = adapter.run_query(query, vars);
    let result = result.unwrap();
    result.collect()
}