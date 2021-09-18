use anyhow::Result;
use assert_cmd::prelude::*;
use std::process::Command;

#[test]
fn cog_display_help() -> Result<()> {
    Command::cargo_bin("cog")?.arg("--help").assert().success();

    Ok(())
}

#[test]
fn coco_display_help() -> Result<()> {
    Command::cargo_bin("coco")?.arg("--help").assert().success();

    Ok(())
}
