use anyhow::Result;
use assert_cmd::prelude::*;
use std::process::Command;

#[test]
#[cfg(not(tarpaulin))]
fn cog_display_help() -> Result<()> {
    Command::cargo_bin("cog")?
        .arg("--help")
        .assert()
        .success();

    Ok(())
}

#[test]
#[cfg(not(tarpaulin))]
fn coco_display_help() -> Result<()> {
    Command::cargo_bin("coco")?
        .arg("--help")
        .assert()
        .success();

    Ok(())
}
