use std::process::Command;

use crate::helpers::*;

use anyhow::Result;
use assert_cmd::prelude::*;
use indoc::indoc;
use sealed_test::prelude::*;

#[sealed_test]
fn verify_ok() -> Result<()> {
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
}

#[sealed_test]
fn verify_with_scope() -> Result<()> {
    // Arrange
    git_init()?;
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
}

#[test]
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

#[test]
fn verify_fails_on_merge_commit() -> Result<()> {
    // Arrange
    let message = "Merge branch 'main' of github.com:thalo-rs/thalo into dev";

    // Act
    Command::cargo_bin("cog")?
        .arg("verify")
        .arg(message)
        // Assert
        .assert()
        .failure();

    Ok(())
}

#[test]
fn verify_ignores_merge_commit() -> Result<()> {
    // Arrange
    let message = "Merge branch 'main' of github.com:thalo-rs/thalo into dev";

    // Act
    Command::cargo_bin("cog")?
        .arg("verify")
        .arg("--ignore-merge-commit")
        .arg(message)
        // Assert
        .assert()
        .success();

    Ok(())
}
