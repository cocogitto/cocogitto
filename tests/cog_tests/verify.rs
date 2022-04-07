use std::process::Command;

use crate::helpers::*;

use anyhow::Result;
use assert_cmd::prelude::*;
use cmd_lib::run_cmd;
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
        .stderr(expected);

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
        .stderr(expected);
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
fn verify_with_unknown_commit_type_fails() -> Result<()> {
    // Arrange
    let message = "toto: la totomobile";

    // Act
    Command::cargo_bin("cog")?
        .arg("verify")
        .arg(message)
        // Assert
        .assert()
        .failure();

    Ok(())
}

#[sealed_test]
fn should_not_ignore_merge_commit_by_default() -> Result<()> {
    // Arrange
    let message = "Merge toto into titi";

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
fn should_ignore_merge_commit_with_ignore_flag() -> Result<()> {
    // Arrange
    let message = "Merge toto into titi";

    // Act
    Command::cargo_bin("cog")?
        .arg("verify")
        .arg("--ignore-merge-commits")
        .arg(message)
        // Assert
        .assert()
        .success();

    Ok(())
}

#[sealed_test]
fn should_ignore_merge_commit_via_config() -> Result<()> {
    // Arrange
    git_init()?;
    let settings = r#"ignore_merge_commits = true"#;

    run_cmd!(
        echo $settings > cog.toml;
        git add .;
        git commit -m "feat: cog.toml config"
    )?;

    let message = "Merge toto into titi";

    // Act
    Command::cargo_bin("cog")?
        .arg("verify")
        .arg(message)
        // Assert
        .assert()
        .success();

    Ok(())
}
