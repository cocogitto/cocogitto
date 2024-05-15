use crate::helpers::*;

use anyhow::Result;
use assert_cmd::Command;
use predicates::prelude::predicate;
use sealed_test::prelude::*;

#[sealed_test]
fn cog_check_ok() -> Result<()> {
    // Arrange
    git_init()?;
    git_commit("chore: init")?;
    git_commit("feat: feature")?;
    git_commit("fix: bug fix")?;

    // Act
    Command::cargo_bin("cog")?
        .arg("check")
        // Assert
        .assert()
        .success()
        .stderr(predicate::str::contains("No errored commits"));
    Ok(())
}

#[sealed_test]
fn cog_check_failure() -> Result<()> {
    // Arrange
    git_init()?;
    git_commit("chore: init")?;
    git_commit("toto: feature")?;
    git_commit("fix: bug fix")?;

    // Act
    Command::cargo_bin("cog")?
        .arg("check")
        // Assert
        .assert()
        .failure()
        .stderr(predicate::str::contains("Found 1 non compliant commits"));
    Ok(())
}

#[sealed_test]
fn cog_check_from_latest_tag_ok() -> Result<()> {
    // Arrange
    git_init()?;
    git_commit("chore: init")?;
    git_commit("toto: errored commit")?;
    git_commit("feat: feature")?;
    git_tag("1.0.0")?;
    git_commit("fix: bug fix")?;

    // Act
    Command::cargo_bin("cog")?
        .arg("check")
        .arg("--from-latest-tag")
        // Assert
        .assert()
        .success()
        .stderr(predicate::str::contains("No errored commits"));
    Ok(())
}

#[sealed_test]
fn cog_check_from_latest_tag_failure() -> Result<()> {
    // Arrange
    git_init()?;
    git_commit("chore: init")?;
    git_commit("toto: errored commit")?;
    git_commit("feat: feature")?;
    git_tag("1.0.0")?;
    git_commit("fix: bug fix")?;
    git_commit("toto: africa")?;

    // Act
    Command::cargo_bin("cog")?
        .arg("check")
        .arg("--from-latest-tag")
        // Assert
        .assert()
        .failure()
        .stderr(predicate::str::contains("Found 1 non compliant commits"));
    Ok(())
}

#[sealed_test]
fn cog_check_commit_range_ok() -> Result<()> {
    // Arrange
    git_init()?;
    let range_start = git_commit("chore: init")?;
    git_commit("feat: feature")?;
    let range_end = git_commit("fix: bug fix")?;
    let range = format!("{range_start}..{range_end}");

    // Act
    Command::cargo_bin("cog")?
        .arg("check")
        .arg(range)
        // Assert
        .assert()
        .success()
        .stderr(predicate::str::contains("No errored commits"));
    Ok(())
}

#[sealed_test]
fn cog_check_commit_range_failure() -> Result<()> {
    // Arrange
    git_init()?;
    let range_start = git_commit("chore: init")?;
    git_commit("toto: errored commit")?;
    git_commit("feat: feature")?;
    git_commit("fix: bug fix")?;
    let range_end = git_commit("toto: africa")?;
    let range = format!("{range_start}..{range_end}");

    // Act
    Command::cargo_bin("cog")?
        .arg("check")
        .arg(range)
        // Assert
        .assert()
        .failure()
        .stderr(predicate::str::contains("Found 2 non compliant commits"));
    Ok(())
}

#[sealed_test]
fn cog_check_from_latest_tag_and_commit_range_failure() -> Result<()> {
    // Arrange

    // Act
    Command::cargo_bin("cog")?
        .arg("check")
        .arg("--from-latest-tag")
        .arg("abcdef..fedcba")
        // Assert
        .assert()
        .failure()
        .stderr(predicate::str::contains(
            "the argument '--from-latest-tag' cannot be used with '[RANGE]'",
        ));
    Ok(())
}
