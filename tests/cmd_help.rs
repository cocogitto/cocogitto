use std::process::Command;

use anyhow::Result;
use assert_cmd::prelude::*;

#[test]
fn cog_display_help() -> Result<()> {
    Command::cargo_bin("cog")?.arg("--help").assert().success();

    Ok(())
}
