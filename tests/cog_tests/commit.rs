use std::ffi::OsStr;
use std::process::Command;

use crate::helpers::*;

use anyhow::Result;
use assert_cmd::prelude::*;
use indoc::{formatdoc, indoc};
use pretty_assertions::assert_eq;
use sealed_test::prelude::*;

#[sealed_test]
fn commit_ok() -> Result<()> {
    // Arrange
    git_init()?;
    git_add("content", "test_file")?;

    // Act
    Command::cargo_bin("cog")?
        .arg("commit")
        .arg("feat")
        .arg("this is a commit message")
        .arg("scope")
        // Assert
        .assert()
        .success();
    Ok(())
}

#[sealed_test]
fn commit_fail_if_not_a_repository() -> Result<()> {
    // Act
    let output = Command::cargo_bin("cog")?
        .arg("commit")
        .arg("feat")
        .arg("this is a commit message")
        .arg("scope")
        .output()?;

    let stderr = String::from_utf8(output.stderr)?;

    // Assert
    let current_dir = std::env::current_dir()?;
    let current_dir: &OsStr = current_dir.as_os_str();
    let current_dir = current_dir.to_str().expect("utf8 error");

    assert_eq!(
        stderr,
        formatdoc!(
            "Error: failed to open repository

        cause: could not find repository from '{}'; class=Repository (6); code=NotFound (-3)

        ",
            current_dir
        )
    );
    Ok(())
}

#[sealed_test]
fn unstaged_changes_commit_err() -> Result<()> {
    // Arrange
    git_init()?;
    std::fs::write("test_file", "content")?;

    // Act
    let output = Command::cargo_bin("cog")?
        .arg("commit")
        .arg("feat")
        .arg("this is a commit message")
        .arg("scope")
        .output()?;

    let stderr = String::from_utf8(output.stderr)?;

    // Assert
    assert_eq!(
        stderr,
        indoc!(
            "Error: Untracked files :
                \tnew: test_file

                nothing added to commit but untracked files present (use \"git add\" to track)\n\n"
        )
    );

    Ok(())
}

#[sealed_test]
fn untracked_changes_commit_ok() -> Result<()> {
    // Arrange
    git_init()?;
    git_add("content", "staged")?;
    std::fs::write("untracked", "content")?;

    // Act
    Command::cargo_bin("cog")?
        .arg("commit")
        .arg("feat")
        .arg("this is a commit message")
        .arg("scope")
        // Assert
        .assert()
        .success();
    Ok(())
}

#[sealed_test]
fn empty_commit_err() -> Result<()> {
    // Arrange
    git_init()?;

    // Act
    let output = Command::cargo_bin("cog")?
        .arg("commit")
        .arg("feat")
        .arg("this is a commit message")
        .arg("scope")
        .output()?;

    let stderr = String::from_utf8(output.stderr)?;

    // Assert
    assert_eq!(
        stderr,
        "Error: nothing to commit (create/copy files and use \"git add\" to track)\n\n"
    );

    Ok(())
}
