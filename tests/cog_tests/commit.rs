use std::process::Command;

use crate::helpers::*;

use anyhow::Result;
use assert_cmd::prelude::*;
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
fn unstaged_changes_commit_err() -> Result<()> {
    // Arrange
    git_init()?;
    std::fs::write("test_file", "content")?;

    // Act
    Command::cargo_bin("cog")?
        .arg("commit")
        .arg("feat")
        .arg("this is a commit message")
        .arg("scope")
        .arg("this is the body")
        .arg("this is the footer")
        // Assert
        .assert()
        .failure();
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
    Command::cargo_bin("cog")?
        .arg("commit")
        .arg("feat")
        .arg("this is a commit message")
        .arg("scope")
        .arg("this is the body")
        .arg("this is the footer")
        // Assert
        .assert()
        .failure();
    Ok(())
}
