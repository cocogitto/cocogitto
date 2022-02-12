use std::process::Command;

use crate::helpers::*;

use anyhow::Result;
use assert_cmd::prelude::*;
use indoc::indoc;
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
    Command::cargo_bin("cog")?
        .arg("commit")
        .arg("feat")
        .arg("this is a commit message")
        .arg("scope")
        // Assert
        .assert()
        .failure()
        .stderr(predicates::str::contains(
            "Error: Failed to open repository",
        ));
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
