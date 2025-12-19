use std::process::Command;

use anyhow::Result;
use assert_cmd::assert::OutputAssertExt;

#[test]
fn cog_display_help() -> Result<()> {
    Command::new(assert_cmd::cargo_bin!("cog"))
        .arg("--help")
        .assert()
        .success();

    Ok(())
}
