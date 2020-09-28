use anyhow::Result;
use assert_cmd::prelude::*;
use std::process::Command;

#[test]
#[cfg(not(tarpaulin))]
fn verify_ok() -> Result<()> {
    let message = "chore: a commit message";
    let username = Command::new("git")
        .arg("config")
        .arg("user.name")
        .output()?
        .stdout;

    let username = String::from_utf8(username)?;
    let user = username.trim_end();

    let mut command = Command::cargo_bin("cog")?;
    command.arg("verify").arg(message);

    command.assert().success().stdout(format!(
        r#"a commitw message (not committed) - now
	Author: {username}
	Type: chore
	Scope: none

"#,
        username = user
    ));

    Ok(())
}

#[test]
#[cfg(not(tarpaulin))]
fn verify_with_scope() -> Result<()> {
    let message = "feat(feature): a commit message";
    let username = Command::new("git")
        .arg("config")
        .arg("user.name")
        .output()?
        .stdout;

    let username = String::from_utf8(username)?;
    let user = username.trim_end();

    let mut command = Command::cargo_bin("cog")?;
    command.arg("verify").arg(message);

    command.assert().success().stdout(format!(
        r#"a commit message (not committed) - now
	Author: {username}
	Type: feat
	Scope: feature

"#,
        username = user
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
