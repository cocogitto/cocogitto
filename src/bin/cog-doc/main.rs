use crate::ser::Item;
use clap::Parser;
use cocogitto::settings::Settings;
use items::FieldOrVariant;
use query::STRUCT_QUERY;
use schemars::schema_for;
use std::collections::HashMap;
use std::fs;
use std::path::PathBuf;
use std::process::Command;
use trustfall_rustdoc::{VersionedIndexedCrate, VersionedRustdocAdapter};

mod visitor;
mod query;
mod items;
mod ser;

const _CONFIG_REFERENCE_HEADING: &str = r#"# Configuration reference

The config reference list all value that can be set in the `cog.toml` file at the root of a repository.

"#;

/// Generate json-schema and markdown documentation for cocogitto settings
#[derive(Parser)]
#[command(
    version,
    name = "cog-doc",
    author = "Paul D. <paul.delafosse@protonmail.com>"
)]
pub enum Cli {
    JsonSchema {
        /// Target output file
        #[arg(short, long)]
        out: Option<PathBuf>,
    },
    ConfigReference {
        /// Target output file
        #[arg(short, long)]
        out: Option<PathBuf>,
    },
}
fn main() -> anyhow::Result<()> {
    let cli = Cli::parse();

    match cli {
        Cli::JsonSchema { out } => {
            let schema = schema_for!(Settings);
            let schema = serde_json::to_string_pretty(&schema)?;
            match out {
                None => println!("{}", schema),
                Some(out) => fs::write(out, schema)?,
            }
        }
        Cli::ConfigReference { .. } => {
            trustfall_docgen();
        }
    }

    Ok(())
}

fn trustfall_docgen() {
    let manifest_dir = env!("CARGO_MANIFEST_DIR");
    Command::new("cargo")
        .env("RUSTC_BOOTSTRAP", "1")
        .env(
            "RUSTDOCFLAGS",
            "-Z unstable-options --output-format=json --cap-lints=allow",
        )
        .arg("+nightly")
        .arg("doc")
        .arg("--no-deps")
        .status().unwrap();

    let json_doc = PathBuf::from(manifest_dir).join("target/doc/cocogitto.json");
    let versioned_crate = trustfall_rustdoc::load_rustdoc(&json_doc).unwrap();
    println!("{:?}", versioned_crate.crate_version());
    let current = VersionedIndexedCrate::new(&versioned_crate);
    let adapter = VersionedRustdocAdapter::new(&current, None).unwrap();

    let result = query::trustfall_query(&adapter, "Settings", STRUCT_QUERY);
    let items = visitor::visit_struct(&adapter, result);

    let mut grouped_items: HashMap<String, Vec<FieldOrVariant>> = HashMap::new();

    for item in items {
        grouped_items.entry(match &item {
            FieldOrVariant::Struct(s) => s.parent_name.clone(),
            FieldOrVariant::Enum(e) => e.enum_name.clone()
        })
            .or_insert_with(Vec::new)
            .push(item);
    }

    let grouped_items: Vec<Item> = grouped_items.into_iter()
        .map(|(name, value)| Item {
            name,
            values: value,
        })
        .collect();

    for item in grouped_items.iter() {
        println!("{}", item.to_markdown());
    }

}


