use std::process::Command;

use crate::helpers::*;

use anyhow::Result;
use assert_cmd::prelude::*;
use indoc::indoc;
use sealed_test::prelude::*;
use speculoos::prelude::*;
use std::path::Path;

#[sealed_test]
fn auto_bump_from_start_ok() -> Result<()> {
    git_init()?;
    git_commit("chore: init")?;
    git_commit("feat(taef): feature")?;
    git_commit("fix: bug fix")?;

    Command::cargo_bin("cog")?
        .arg("bump")
        .arg("--auto")
        .assert()
        .success();

    assert_that!(Path::new("CHANGELOG.md")).exists();
    assert_tag_exists("0.1.0")?;
    Ok(())
}

#[sealed_test]
fn auto_bump_minor_from_latest_tag() -> Result<()> {
    git_init()?;
    git_commit("chore: init")?;
    git_commit("feat(taef): feature")?;
    git_commit("fix: bug fix")?;
    git_tag("1.0.0")?;
    git_commit("feat(taef): feature")?;
    git_commit("feat: feature 1")?;
    git_commit("feat: feature 2")?;

    Command::cargo_bin("cog")?
        .arg("bump")
        .arg("--auto")
        .assert()
        .success();

    assert_that!(Path::new("CHANGELOG.md")).exists();
    assert_tag_exists("1.1.0")?;
    Ok(())
}

#[sealed_test]
fn auto_bump_dry_run_from_latest_tag() -> Result<()> {
    git_init()?;
    git_commit("chore: init")?;
    git_commit("feat(taef): feature")?;
    git_commit("fix: bug fix")?;
    git_tag("1.0.0")?;
    git_commit("feat(taef): feature")?;
    git_commit("feat: feature 1")?;
    git_commit("feat: feature 2")?;

    Command::cargo_bin("cog")?
        .arg("bump")
        .arg("--auto")
        .arg("--dry-run")
        .assert()
        .success()
        .stdout("1.1.0");

    assert_that!(Path::new("CHANGELOG.md")).does_not_exist();
    assert_tag_does_not_exist("1.1.0")?;
    Ok(())
}

#[sealed_test]
fn auto_bump_major_from_latest_tag() -> Result<()> {
    git_init()?;
    git_commit("chore: init")?;
    git_commit("feat(taef): feature")?;
    git_commit("fix: bug fix")?;
    git_tag("1.0.0")?;
    git_commit("feat!(taef): feature")?;
    git_commit("feat!: feature 1")?;
    git_commit("feat: feature 2")?;

    Command::cargo_bin("cog")?
        .arg("bump")
        .arg("--auto")
        .assert()
        .success();

    assert_that!(Path::new("CHANGELOG.md")).exists();
    assert_tag_exists("2.0.0")?;
    Ok(())
}

#[sealed_test]
fn auto_bump_with_prefix() -> Result<()> {
    git_init()?;
    git_add("tag_prefix = \"v\"", "cog.toml")?;
    git_commit("chore: init")?;
    git_commit("feat(taef): feature")?;
    git_commit("fix: bug fix")?;
    git_tag("v1.0.0")?;
    git_commit("feat(taef)!: feature")?;
    git_commit("feat!: feature 1")?;
    git_commit("feat: feature 2")?;

    Command::cargo_bin("cog")?
        .arg("bump")
        .arg("--auto")
        .assert()
        .success();

    assert_that!(Path::new("CHANGELOG.md")).exists();
    assert_tag_exists("v2.0.0")?;
    Ok(())
}

#[sealed_test]
fn auto_bump_patch_from_latest_tag() -> Result<()> {
    git_init()?;
    git_commit("chore: init")?;
    git_commit("feat(taef): feature")?;
    git_commit("fix: bug fix")?;
    git_tag("1.0.0")?;
    git_commit("fix(the_fix): the_fix")?;
    git_commit("fix: fix 1")?;
    git_commit("fix: fix 2")?;

    Command::cargo_bin("cog")?
        .arg("bump")
        .arg("--auto")
        .assert()
        .success();

    assert_that!(Path::new("CHANGELOG.md")).exists();
    assert_tag_exists("1.0.1")?;
    Ok(())
}

#[sealed_test]
fn auto_bump_respect_semver_sorting() -> Result<()> {
    git_init()?;
    git_commit("chore: init")?;
    git_commit("feat(taef): feature")?;
    git_commit("fix: bug fix")?;
    git_tag("0.9.1")?;
    git_commit("feat(the_fix): feature")?;
    git_tag("0.10.0")?;
    git_commit("fix: fix 1")?;
    git_commit("fix: fix 2")?;

    Command::cargo_bin("cog")?
        .arg("bump")
        .arg("--auto")
        .assert()
        .success();

    assert_that!(Path::new("CHANGELOG.md")).exists();
    assert_tag_exists("0.10.1")?;
    Ok(())
}

#[sealed_test]
fn minor_bump() -> Result<()> {
    git_init()?;
    git_commit("chore: init")?;
    git_tag("1.0.0")?;
    git_commit("feat: feature")?;

    Command::cargo_bin("cog")?
        .arg("bump")
        .arg("--minor")
        .assert()
        .success();

    assert_that!(Path::new("CHANGELOG.md")).exists();
    assert_tag_exists("1.1.0")?;
    Ok(())
}

#[sealed_test]
fn major_bump() -> Result<()> {
    git_init()?;
    git_commit("chore: init")?;
    git_tag("1.0.0")?;
    git_commit("feat: feature")?;

    Command::cargo_bin("cog")?
        .arg("bump")
        .arg("--major")
        .assert()
        .success();

    assert_that!(Path::new("CHANGELOG.md")).exists();
    assert_tag_exists("2.0.0")?;
    Ok(())
}

#[sealed_test]
fn patch_bump() -> Result<()> {
    git_init()?;
    git_commit("chore: init")?;
    git_tag("1.0.0")?;
    git_commit("feat: feature")?;

    Command::cargo_bin("cog")?
        .arg("bump")
        .arg("--patch")
        .assert()
        .success();
    assert_that!(Path::new("CHANGELOG.md")).exists();
    assert_tag_exists("1.0.1")?;
    Ok(())
}

#[sealed_test]
fn pre_release_bump() -> Result<()> {
    git_init()?;
    git_commit("chore: init")?;
    git_tag("1.0.0")?;
    git_commit("feat: feature")?;

    Command::cargo_bin("cog")?
        .arg("bump")
        .arg("--major")
        .arg("--pre")
        .arg("alpha")
        .assert()
        .success();

    assert_that!(Path::new("CHANGELOG.md")).exists();
    assert_tag_exists("2.0.0-alpha")?;
    Ok(())
}

#[sealed_test]
#[cfg(target_os = "linux")]
fn bump_with_hook() -> Result<()> {
    // Arrange
    git_init()?;
    git_add(r#"pre_bump_hooks = ["touch {{version}}"]"#, "cog.toml")?;
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

    assert_that!(Path::new("2.0.0")).exists();
    assert_tag_exists("2.0.0")?;
    Ok(())
}

#[sealed_test]
#[cfg(target_os = "linux")]
fn bump_with_profile_hook() -> Result<()> {
    // Arrange
    git_init()?;

    let config = indoc! {
        "[bump_profiles.custom]
            pre_bump_hooks = [ \"echo current {{latest}}\" ]
            post_bump_hooks = [ \"echo next {{version}}\" ]
        "
    };

    git_add(config, "cog.toml")?;

    git_commit("chore: init")?;
    git_tag("1.0.0")?;
    git_commit("feat: feature")?;

    let expected_stdout = indoc!(
        "current 1.0.0
            next 1.0.1
        "
    );
    let expected_stderr = "Bumped version: 1.0.0 -> 1.0.1\n";

    // Act
    Command::cargo_bin("cog")?
        .arg("bump")
        .arg("--hook-profile")
        .arg("custom")
        .arg("--patch")
        .unwrap()
        // Assert
        .assert()
        .stdout(expected_stdout)
        .stderr(expected_stderr)
        .success();

    assert_tag_exists("1.0.1")?;
    Ok(())
}
