use crate::helpers::*;
use anyhow::Result;
use assert_cmd::prelude::*;
use speculoos::prelude::*;
use std::process::Command;

use indoc::indoc;

#[test]
fn auto_bump_from_start_ok() -> Result<()> {
    run_test_with_context(|context| {
        let mut command = Command::cargo_bin("cog")?;
        command.arg("bump").arg("--auto");
        git_init()?;
        git_commit("chore: init")?;
        git_commit("feat(taef): feature")?;
        git_commit("fix: bug fix")?;

        command.assert().success();

        assert_that(&context.test_dir.join("CHANGELOG.md")).exists();
        assert_tag("0.1.0")?;
        Ok(())
    })
}

#[test]
fn auto_bump_minor_from_latest_tag() -> Result<()> {
    run_test_with_context(|context| {
        let mut command = Command::cargo_bin("cog")?;
        command.arg("bump").arg("--auto");

        git_init()?;
        git_commit("chore: init")?;
        git_commit("feat(taef): feature")?;
        git_commit("fix: bug fix")?;
        git_tag("1.0.0")?;
        git_commit("feat(taef): feature")?;
        git_commit("feat: feature 1")?;
        git_commit("feat: feature 2")?;

        command.assert().success();
        assert_that(&context.test_dir.join("CHANGELOG.md")).exists();
        assert_tag("1.1.0")?;
        Ok(())
    })
}

#[test]
fn auto_bump_major_from_latest_tag() -> Result<()> {
    run_test_with_context(|context| {
        let mut command = Command::cargo_bin("cog")?;

        command.arg("bump").arg("--auto");
        git_init()?;
        git_commit("chore: init")?;
        git_commit("feat(taef): feature")?;
        git_commit("fix: bug fix")?;
        git_tag("1.0.0")?;
        git_commit("feat!(taef): feature")?;
        git_commit("feat!: feature 1")?;
        git_commit("feat: feature 2")?;

        command.assert().success();
        assert_that(&context.test_dir.join("CHANGELOG.md")).exists();
        assert_tag("2.0.0")?;
        Ok(())
    })
}

#[test]
fn auto_bump_patch_from_latest_tag() -> Result<()> {
    run_test_with_context(|context| {
        let mut command = Command::cargo_bin("cog")?;
        command.arg("bump").arg("--auto");

        git_init()?;
        git_commit("chore: init")?;
        git_commit("feat(taef): feature")?;
        git_commit("fix: bug fix")?;
        git_tag("1.0.0")?;
        git_commit("fix(the_fix): the_fix")?;
        git_commit("fix: fix 1")?;
        git_commit("fix: fix 2")?;

        command.assert().success();
        assert_that(&context.test_dir.join("CHANGELOG.md")).exists();
        assert_tag("1.0.1")?;
        Ok(())
    })
}

#[test]
fn auto_bump_respect_semver_sorting() -> Result<()> {
    run_test_with_context(|context| {
        let mut command = Command::cargo_bin("cog")?;
        command.arg("bump").arg("--auto");

        git_init()?;
        git_commit("chore: init")?;
        git_commit("feat(taef): feature")?;
        git_commit("fix: bug fix")?;
        git_tag("0.9.1")?;
        git_commit("feat(the_fix): feature")?;
        git_tag("0.10.0")?;
        git_commit("fix: fix 1")?;
        git_commit("fix: fix 2")?;

        command.assert().success();
        assert_that(&context.test_dir.join("CHANGELOG.md")).exists();
        assert_tag("0.10.1")?;
        Ok(())
    })
}

#[test]
fn minor_bump() -> Result<()> {
    run_test_with_context(|context| {
        let mut command = Command::cargo_bin("cog")?;
        command.arg("bump").arg("--minor");

        git_init()?;
        git_commit("chore: init")?;
        git_tag("1.0.0")?;
        git_commit("feat: feature")?;

        command.assert().success();
        assert_that(&context.test_dir.join("CHANGELOG.md")).exists();
        assert_tag("1.1.0")?;
        Ok(())
    })
}

#[test]
fn major_bump() -> Result<()> {
    run_test_with_context(|context| {
        let mut command = Command::cargo_bin("cog")?;
        command.arg("bump").arg("--major");

        git_init()?;
        git_commit("chore: init")?;
        git_tag("1.0.0")?;
        git_commit("feat: feature")?;

        command.assert().success();
        assert_that(&context.test_dir.join("CHANGELOG.md")).exists();
        assert_tag("2.0.0")?;
        Ok(())
    })
}

#[test]
fn patch_bump() -> Result<()> {
    run_test_with_context(|context| {
        let mut command = Command::cargo_bin("cog")?;
        command.arg("bump").arg("--patch");

        git_init()?;
        git_commit("chore: init")?;
        git_tag("1.0.0")?;
        git_commit("feat: feature")?;

        command.assert().success();
        assert_that(&context.test_dir.join("CHANGELOG.md")).exists();
        assert_tag("1.0.1")?;
        Ok(())
    })
}

#[test]
fn pre_release_bump() -> Result<()> {
    run_test_with_context(|context| {
        let mut command = Command::cargo_bin("cog")?;
        command.arg("bump").arg("--major").arg("--pre").arg("alpha");

        git_init()?;
        git_commit("chore: init")?;
        git_tag("1.0.0")?;
        git_commit("feat: feature")?;

        command.assert().success();
        assert_that(&context.test_dir.join("CHANGELOG.md")).exists();
        assert_tag("2.0.0-alpha")?;
        Ok(())
    })
}

#[test]
#[cfg(target_os = "linux")]
fn bump_with_hook() -> Result<()> {
    run_test_with_context(|context| {
        // Arrange
        std::fs::write("cog.toml", r#"pre_bump_hooks = ["touch {{version}}"]"#)?;

        git_init()?;
        git_commit("chore: init")?;
        git_tag("1.0.0")?;
        git_commit("feat: feature")?;

        // Act
        Command::cargo_bin("cog")?
            .arg("bump")
            .arg("--major")
            // Assert
            .assert()
            .success();

        assert_that(&context.test_dir.join("2.0.0")).exists();
        assert_tag("2.0.0")?;
        Ok(())
    })
}

#[test]
#[cfg(target_os = "linux")]
fn bump_with_profile_hook() -> Result<()> {
    run_test_with_context(|_context| {
        // Arrange
        let config = indoc! {
            "[bump_profiles.custom]
            pre_bump_hooks = [ \"echo current {{latest}}\" ]
            post_bump_hooks = [ \"echo next {{version}}\" ]
        "
        };

        std::fs::write("cog.toml", config)?;

        git_init()?;
        git_commit("chore: init")?;
        git_tag("1.0.0")?;
        git_commit("feat: feature")?;

        let expected = indoc! {
            "current 1.0.0
        next 1.0.1
        Bumped version:1.0.0 -> 1.0.1
        "
        };

        // Act
        Command::cargo_bin("cog")?
            .arg("bump")
            .arg("--hook-profile")
            .arg("custom")
            .arg("--patch")
            .unwrap()
            // Assert
            .assert()
            .stdout(expected)
            .success();

        assert_tag("1.0.1")?;
        Ok(())
    })
}
