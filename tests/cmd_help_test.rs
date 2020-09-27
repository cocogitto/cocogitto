use anyhow::Result;
use assert_cmd::prelude::*;
use std::process::Command;

#[test]
#[cfg(not(tarpaulin))]
fn cog_display_help() -> Result<()> {
    let mut command = Command::cargo_bin("cog")?;

    command.arg("--help");
    command.assert().success();

    Ok(())
}

#[test]
#[cfg(not(tarpaulin))]
fn coco_display_help() -> Result<()> {
    let mut command = Command::cargo_bin("coco")?;

    command.arg("--help");
    command.assert().success();

    Ok(())
}
