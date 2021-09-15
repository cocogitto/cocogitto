use anyhow::Result;
use assert_cmd::prelude::*;
use std::process::Command;

mod helper;
use helper::run_test_with_context;

#[test]
#[cfg(not(tarpaulin))]
fn commit_ok() -> Result<()> {
    run_test_with_context(|context| {
        // Arrange
        helper::git_init(".")?;
        std::fs::write(context.test_dir.join("test_file"), "content")?;
        helper::git_add()?;

        // Act
        Command::cargo_bin("coco")?
            .arg("feat")
            .arg("this is a commit message")
            .arg("scope")
            .arg("this is the body")
            .arg("this is the footer")
            // Assert
            .assert().success();
        Ok(())
    })
}

#[test]
#[cfg(not(tarpaulin))]
fn unstaged_changes_commit_err() -> Result<()> {
    run_test_with_context(|context| {
        // Arrange
        helper::git_init(".")?;
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
#[cfg(not(tarpaulin))]
fn untracked_changes_commit_ok() -> Result<()> {
    run_test_with_context(|context| {
        // Arrange
        helper::git_init(".")?;
        std::fs::write(context.test_dir.join("staged"), "content")?;
        helper::git_add()?;
        std::fs::write(context.test_dir.join("untracked"), "content")?;

        // Act
        Command::cargo_bin("coco")?
            .arg("feat")
            .arg("this is a commit message")
            .arg("scope")
            .arg("this is the body")
            .arg("this is the footer")
            // Assert
            .assert()
            .success();
        Ok(())
    })
}

#[test]
#[cfg(not(tarpaulin))]
fn empty_commit_err() -> Result<()> {
    run_test_with_context(|_context| {
        // Arrange
        helper::git_init(".")?;

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
