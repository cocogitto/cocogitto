use anyhow::Result;
use assert_cmd::prelude::*;
use std::process::Command;

#[test]
#[cfg(not(tarpaulin))]
fn display_help() -> Result<()> {
    let mut command = Command::cargo_bin("cog")?;

    command.arg("--help");
    command.assert().success();

    Ok(())
}
