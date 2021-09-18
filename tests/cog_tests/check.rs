use crate::helpers::*;

use anyhow::Result;
use assert_cmd::Command;
use predicates::prelude::predicate;

#[test]
fn cog_check_ok() -> Result<()> {
    run_test_with_context(|_| {
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
            .stdout(predicate::str::contains("No errored commits"));
        Ok(())
    })
}

#[test]
fn cog_check_failure() -> Result<()> {
    run_test_with_context(|_| {
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
    })
}

#[test]
fn cog_check_from_latest_tag_ok() -> Result<()> {
    run_test_with_context(|_| {
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
            .stdout(predicate::str::contains("No errored commits"));
        Ok(())
    })
}

#[test]
fn cog_check_from_latest_tag_failure() -> Result<()> {
    run_test_with_context(|_| {
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
    })
}
