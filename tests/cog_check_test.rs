use anyhow::Result;
use assert_cmd::Command;
use predicates::prelude::predicate;

pub mod helper;
use helper::run_test_with_context;

#[test]
#[cfg(not(tarpaulin))]
fn cog_check_ok() -> Result<()> {
    run_test_with_context(|_| {
        // Arrange
        helper::git_init()?;
        helper::git_commit("chore: init")?;
        helper::git_commit("feat: feature")?;
        helper::git_commit("fix: bug fix")?;

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
#[cfg(not(tarpaulin))]
fn cog_check_failure() -> Result<()> {
    run_test_with_context(|_| {
        // Arrange
        helper::git_init()?;
        helper::git_commit("chore: init")?;
        helper::git_commit("toto: feature")?;
        helper::git_commit("fix: bug fix")?;

        // Act
        Command::cargo_bin("cog")?
            .arg("check")
            // Assert
            .assert()
            .failure()
            .stderr(predicate::str::contains("Commit type `toto` not allowed"));
        Ok(())
    })
}

#[test]
#[cfg(not(tarpaulin))]
fn cog_check_from_latest_tag_ok() -> Result<()> {
    run_test_with_context(|_| {
        // Arrange
        helper::git_init()?;
        helper::git_commit("chore: init")?;
        helper::git_commit("toto: errored commit")?;
        helper::git_commit("feat: feature")?;
        helper::git_tag("1.0.0")?;
        helper::git_commit("fix: bug fix")?;

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
#[cfg(not(tarpaulin))]
fn cog_check_from_latest_tag_failure() -> Result<()> {
    run_test_with_context(|_| {
        // Arrange
        helper::git_init()?;
        helper::git_commit("chore: init")?;
        helper::git_commit("toto: errored commit")?;
        helper::git_commit("feat: feature")?;
        helper::git_tag("1.0.0")?;
        helper::git_commit("fix: bug fix")?;
        helper::git_commit("toto: africa")?;

        // Act
        Command::cargo_bin("cog")?
            .arg("check")
            .arg("--from-latest-tag")
            // Assert
            .assert()
            .failure()
            .stderr(predicate::str::contains("Commit type `toto` not allowed"));
        Ok(())
    })
}
