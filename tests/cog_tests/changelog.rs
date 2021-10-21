use crate::helpers::*;

use anyhow::Result;
use assert_cmd::Command;
use cocogitto::settings::Settings;
use predicates::prelude::{predicate, PredicateBooleanExt};
use speculoos::prelude::*;

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
fn get_changelog_from_tag() -> Result<()> {
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

#[test]
fn get_changelog_with_tag_prefix() -> Result<()> {
    run_test_with_context(|context| {
        // Arrange
        let mut settings = Settings::default();
        settings.tag_prefix = Some("v".to_string());
        let settings = toml::to_string(&settings);
        std::fs::write(context.test_dir.join("cog.toml"), settings?)?;

        git_init()?;
        git_commit("chore: init")?;
        git_commit("feat: feature 1")?;
        git_tag("v1.0.0")?;
        git_commit("fix: bug fix 1")?;
        git_log()?;

        // Act
        let output = Command::cargo_bin("cog")?
            .arg("changelog")
            // Assert
            .assert()
            .success();

        let output = output.get_output();
        let output = String::from_utf8_lossy(&output.stdout);

        assert_that!(output.as_ref()).contains("bug fix 1");
        assert_that!(output.as_ref()).does_not_contain("Features");
        Ok(())
    })
}

#[test]
fn get_changelog_at_tag_prefix() -> Result<()> {
    run_test_with_context(|context| {
        // Arrange
        let mut settings = Settings::default();
        settings.tag_prefix = Some("v".to_string());
        let settings = toml::to_string(&settings);
        std::fs::write(context.test_dir.join("cog.toml"), settings?)?;

        git_init()?;
        git_commit("chore: init")?;
        git_commit("feat: start")?;
        git_tag("v1.0.0")?;
        git_commit("feat: feature 1")?;
        git_commit("fix: bug fix 1")?;
        git_commit("chore(version): v2.0.0")?;
        git_tag("v2.0.0")?;
        git_commit("feat: end")?;
        git_log()?;

        // Act
        let output = Command::cargo_bin("cog")?
            .arg("changelog")
            .arg("--at")
            .arg("v2.0.0")
            // Assert
            .assert()
            .success();

        let output = output.get_output();
        let output = String::from_utf8_lossy(&output.stdout);

        assert_that!(output.as_ref()).contains("bug fix 1");
        assert_that!(output.as_ref()).contains("feature 1");
        assert_that!(output.as_ref()).does_not_contain("start");
        assert_that!(output.as_ref()).does_not_contain("end");
        Ok(())
    })
}
