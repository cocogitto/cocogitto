use anyhow::Result;
use assert_cmd::prelude::*;
use std::process::Command;

mod helper;

#[test]
#[cfg(not(tarpaulin))]
fn verify_ok() -> Result<()> {
    let message = "chore: a commit message";
    let username = helper::get_git_user_name()?;

    let mut command = Command::cargo_bin("cog")?;
    command.arg("verify").arg(message);
    command.assert().success().stdout(format!(
        r#"a commit message (not committed) - now
	Author: {}
	Type: chore
	Scope: none

"#,
        username
    ));

    Ok(())
}

#[test]
#[cfg(not(tarpaulin))]
fn verify_with_scope() -> Result<()> {
    let message = "feat(feature): a commit message";
    let username = helper::get_git_user_name()?;

    let mut command = Command::cargo_bin("cog")?;
    command.arg("verify").arg(message);

    command.assert().success().stdout(format!(
        r#"a commit message (not committed) - now
	Author: {}
	Type: feat
	Scope: feature

"#,
        username
    ));

    Ok(())
}

#[test]
#[cfg(not(tarpaulin))]
fn verify_fails() -> Result<()> {
    let message = "invalid message";

    let mut command = Command::cargo_bin("cog")?;
    command.arg("verify").arg(message);

    command.assert().failure();

    Ok(())
}
