use crate::helpers::*;
use anyhow::Result;
use assert_cmd::Command;
use predicates::prelude::{predicate, PredicateBooleanExt};

#[test]
fn get_changelog_from_untagged_repo() -> Result<()> {
    run_test_with_context(|_| {
        // Arrange
        git_init()?;
        git_commit("chore: init")?;
        git_commit("feat(taef): feature")?;
        git_commit("fix: bug fix")?;

        // Act
        Command::cargo_bin("cog")?
            .arg("changelog")
            // Assert
            .assert()
            .success()
            .stdout(predicate::str::contains("Bug Fixes"))
            .stdout(predicate::str::contains("Features"));
        Ok(())
    })
}

#[test]
fn get_changelog_from_tagged_repo() -> Result<()> {
    run_test_with_context(|_| {
        // Arrange
        git_init()?;
        git_commit("chore: init")?;
        git_commit("feat(taef): feature")?;
        git_tag("1.0.0")?;
        git_commit("fix: bug fix")?;

        // Act
        Command::cargo_bin("cog")?
            .arg("changelog")
            // Assert
            .assert()
            .success()
            .stdout(predicate::str::contains("Bug Fixes"))
            .stdout(predicate::str::contains("Features").not());
        Ok(())
    })
}

#[test]
fn get_changelog_from_at_tag() -> Result<()> {
    run_test_with_context(|_| {
        // Arrange
        git_init()?;
        git_commit("chore: init")?;
        git_commit("feat(taef): feature")?;
        git_commit("feat: feature 2")?;
        git_tag("1.0.0")?;
        git_commit("fix: bug fix")?;
        git_log()?;

        // Act
        Command::cargo_bin("cog")?
            .arg("changelog")
            .arg("--at")
            .arg("1.0.0")
            // Assert
            .assert()
            .success()
            .stdout(predicate::str::contains("Bug Fixes").not());
        Ok(())
    })
}
