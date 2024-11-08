use clap::Parser;
use cocogitto::settings::Settings;
use itertools::Itertools;
use schemars::schema_for;
use std::collections::BTreeMap;
use std::fmt::Write;
use std::fs;
use std::path::PathBuf;
use std::process::Command;
use trustfall_rustdoc::{VersionedIndexedCrate, VersionedRustdocAdapter};

mod props;

const CONFIG_REFERENCE_HEADING: &str = r#"# Configuration reference

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
            unimplemented!("Config reference generation not implemented yet (https://github.com/obi1kenobi/trustfall-rustdoc-adapter/issues/566)");
            trustfall_docgen();
        }
    }

    Ok(())
}

fn trustfall_docgen() {
    let manifest_dir = env!("CARGO_MANIFEST_DIR");
    let doc_build = Command::new("cargo")
        .env("RUSTC_BOOTSTRAP", "1")
        .env(
            "RUSTDOCFLAGS",
            "-Z unstable-options --output-format=json --cap-lints=allow",
        )
        .arg("+nightly")
        .arg("doc")
        .arg("--no-deps")
        .status();

    let json_doc = PathBuf::from(manifest_dir).join("target/doc/cocogitto.json");
    let versioned_crate = trustfall_rustdoc::load_rustdoc(&json_doc).unwrap();
    println!("{:?}", versioned_crate.crate_version());
    let current = VersionedIndexedCrate::new(&versioned_crate);
    let adapter = VersionedRustdocAdapter::new(&current, None).unwrap();

    let query = r#"query {
  Crate {
    item {
      ... on Struct {
        name @filter(op: "=", value: ["$struct_name"])

        field {
          field_name: name @output
          raw_type {
            type_name: name @output
          }
        }
      }
    }
  }
}"#;

    let mut vars = BTreeMap::new();
    vars.insert("struct_name", "Settings");
    let result = adapter.run_query(query, vars);
    let result = result.unwrap();

    for res in result {
        println!("{:#?}", res);
    }
}
