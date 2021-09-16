use anyhow::Result;
use assert_cmd::Command;
use predicates::prelude::{predicate, PredicateBooleanExt};

mod helper;
use helper::run_test_with_context;

#[test]
#[cfg(not(tarpaulin))]
fn get_changelog_from_untagged_repo() -> Result<()> {
    run_test_with_context(|_| {
        // Arrange
        helper::git_init()?;
        helper::git_commit("chore: init")?;
        helper::git_commit("feat(taef): feature")?;
        helper::git_commit("fix: bug fix")?;

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
#[cfg(not(tarpaulin))]
fn get_changelog_from_tagged_repo() -> Result<()> {
    run_test_with_context(|_| {
        // Arrange
        helper::git_init()?;
        helper::git_commit("chore: init")?;
        helper::git_commit("feat(taef): feature")?;
        helper::git_tag("1.0.0")?;
        helper::git_commit("fix: bug fix")?;

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
#[cfg(not(tarpaulin))]
fn get_changelog_from_at_tag() -> Result<()> {
    run_test_with_context(|_| {
        // Arrange
        helper::git_init()?;
        helper::git_commit("chore: init")?;
        helper::git_commit("feat(taef): feature")?;
        helper::git_commit("feat: feature 2")?;
        helper::git_tag("1.0.0")?;
        helper::git_commit("fix: bug fix")?;
        helper::git_log()?;

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
