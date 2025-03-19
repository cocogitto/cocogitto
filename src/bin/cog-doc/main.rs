use crate::markdown::ToMarkDown;
use crate::schema::root_schema;
use clap::Parser;
use cocogitto::settings::Settings;
use schemars::schema_for;
use std::fs;
use std::io::{BufWriter, Write};
use std::path::PathBuf;

mod markdown;
mod schema;

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
    Reference {
        #[arg(short, long)]
        out: PathBuf,
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
        Cli::Reference { out } => {
            let mut root = root_schema()?;
            let buffer = vec![];
            let mut writer = BufWriter::new(buffer);
            writer.write_all(CONFIG_REFERENCE_HEADING.as_bytes())?;
            root.to_markdown(&mut writer)?;
            let content = writer.into_inner()?;
            fs::write(out, content)?;
        }
    }

    Ok(())
}
