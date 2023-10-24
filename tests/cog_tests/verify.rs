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

#[sealed_test(files = ["tests/assets/commit_message.txt"])]
fn verify_file_ok() -> Result<()> {
    // Arrange
    git_init()?;
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
        .arg("--file")
        .arg("commit_message.txt")
        // Assert
        .assert()
        .success()
        .stderr(expected);

    Ok(())
}

#[test]
fn verify_with_not_existing_file_fails() -> Result<()> {
    // Act
    Command::cargo_bin("cog")?
        .arg("verify")
        .arg("--file")
        .arg("not_existing_file.txt")
        // Assert
        .assert()
        .failure();

    Ok(())
}

#[cfg(target_family = "unix")]
#[sealed_test(files = ["tests/assets/commit_message.txt"])]
fn verify_with_unreadable_file_fails() -> Result<()> {
    use std::fs;
    use std::os::unix::fs::PermissionsExt;

    let file_name = "commit_message.txt";

    // Arrange
    let mut perms = fs::metadata(file_name)?.permissions();
    perms.set_mode(0o333); // write-only
    fs::set_permissions(file_name, perms)?;

    // Act
    Command::cargo_bin("cog")?
        .arg("verify")
        .arg("--file")
        .arg(file_name)
        // Assert
        .assert()
        .failure();

    Ok(())
}
