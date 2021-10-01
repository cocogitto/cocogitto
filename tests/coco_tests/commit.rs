use crate::helpers::*;

use anyhow::Result;
use assert_cmd::prelude::*;
use std::process::Command;

#[test]
fn commit_ok() -> Result<()> {
    run_test_with_context(|context| {
        // Arrange
        git_init()?;
        std::fs::write(context.test_dir.join("test_file"), "content")?;
        git_add()?;

        // Act
        Command::cargo_bin("coco")?
            .arg("feat")
            .arg("this is a commit message")
            .arg("scope")
            // Assert
            .assert()
            .success();
        Ok(())
    })
}

#[test]
fn unstaged_changes_commit_err() -> Result<()> {
    run_test_with_context(|context| {
        // Arrange
        git_init()?;
        std::fs::write(context.test_dir.join("test_file"), "content")?;

        // Act
        Command::cargo_bin("coco")?
            .arg("feat")
            .arg("this is a commit message")
            .arg("scope")
            .arg("this is the body")
            .arg("this is the footer")
            // Assert
            .assert()
            .failure();
        Ok(())
    })
}

#[test]
fn untracked_changes_commit_ok() -> Result<()> {
    run_test_with_context(|context| {
        // Arrange
        git_init()?;
        std::fs::write(context.test_dir.join("staged"), "content")?;
        git_add()?;
        std::fs::write(context.test_dir.join("untracked"), "content")?;

        // Act
        Command::cargo_bin("coco")?
            .arg("feat")
            .arg("this is a commit message")
            .arg("scope")
            // Assert
            .assert()
            .success();
        Ok(())
    })
}

#[test]
fn empty_commit_err() -> Result<()> {
    run_test_with_context(|_context| {
        // Arrange
        git_init()?;

        // Act
        Command::cargo_bin("coco")?
            .arg("feat")
            .arg("this is a commit message")
            .arg("scope")
            .arg("this is the body")
            .arg("this is the footer")
            // Assert
            .assert()
            .failure();
        Ok(())
    })
}
