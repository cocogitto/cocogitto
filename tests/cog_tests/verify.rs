use crate::helpers::*;

use anyhow::Result;
use assert_cmd::prelude::*;
use indoc::indoc;
use std::process::Command;

#[test]
#[cfg(not(tarpaulin))]
fn verify_ok() -> Result<()> {
    run_test_with_context(|_| {
        // Arrange
        git_init()?;
        let message = "chore: a commit message";
        let expected = indoc!(
            "a commit message (not committed) - now
            \tAuthor: Tom
            \tType: chore
            \tScope: none

            ",
        );

        // Act
        Command::cargo_bin("cog")?
            .arg("verify")
            .arg(message)
            // Assert
            .assert()
            .success()
            .stdout(expected);

        Ok(())
    })
}

#[test]
#[cfg(not(tarpaulin))]
fn verify_with_scope() -> Result<()> {
    run_test_with_context(|context| {
        // Arrange
        git_init()?;
        println!(
            "{:?}",
            std::fs::read_to_string(context.test_dir.join(".git/config"))
        );
        let message = "feat(feature): a commit message";
        let expected = indoc!(
            "a commit message (not committed) - now
            \tAuthor: Tom
            \tType: feat
            \tScope: feature

            ",
        );

        // Act
        Command::cargo_bin("cog")?
            .arg("verify")
            .arg(message)
            // Assert
            .assert()
            .success()
            .stdout(expected);
        Ok(())
    })
}

#[test]
#[cfg(not(tarpaulin))]
fn verify_fails() -> Result<()> {
    // Arrange
    let message = "invalid message";

    // Act
    Command::cargo_bin("cog")?
        .arg("verify")
        .arg(message)
        // Assert
        .assert()
        .failure();

    Ok(())
}
