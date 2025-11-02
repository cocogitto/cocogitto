use std::process::Command;

use anyhow::Result;
use assert_cmd::prelude::*;
use predicates::prelude::predicate;
use sealed_test::prelude::*;

use cocogitto::settings::Settings;

use crate::helpers::*;

#[sealed_test]
fn get_initial_version_expected_error() -> Result<()> {
    git_init()?;
    git_commit("chore: init")?;
    git_commit("feat(taef): feature")?;

    Command::cargo_bin("cog")?
        .arg("get-version")
        .assert()
        .failure()
        .stderr(predicate::str::starts_with("Error: No version yet\n"));

    Ok(())
}

#[sealed_test]
fn get_initial_version_fallback_ok() -> Result<()> {
    git_init()?;
    git_commit("chore: init")?;
    git_commit("feat(taef): feature")?;

    Command::cargo_bin("cog")?
        .arg("get-version")
        .arg("-f")
        .arg("2.1.0-Test+xx")
        .assert()
        .success()
        .stdout(predicate::eq(b"2.1.0-Test+xx\n" as &[u8]));

    Ok(())
}

#[sealed_test]
fn get_initial_version_invalid_fallback_parse_error() -> Result<()> {
    git_init()?;
    git_commit("chore: init")?;
    git_commit("feat(taef): feature")?;

    Command::cargo_bin("cog")?
        .arg("get-version")
        .arg("-f")
        .arg("InvalidVersion")
        .assert()
        .failure()
        .stderr(predicate::str::starts_with(
            "Invalid fallback: InvalidVersion\n",
        ));

    Ok(())
}

#[sealed_test]
fn get_version_invalid_fallback_parse_error_in_versioned_repo() -> Result<()> {
    git_init()?;
    git_commit("chore: init")?;
    git_commit("feat(taef): feature")?;

    Command::cargo_bin("cog")?
        .arg("bump")
        .arg("--auto")
        .assert()
        .success();

    Command::cargo_bin("cog")?
        .arg("get-version")
        .arg("-f")
        .arg("InvalidVersion")
        .assert()
        .failure()
        .stderr(predicate::str::starts_with(
            "Invalid fallback: InvalidVersion\n",
        ));

    Ok(())
}

#[sealed_test]
fn get_initial_version_fails_as_expected_error() -> Result<()> {
    git_init()?;
    git_commit("chore: init")?;
    git_commit("feat(taef): feature")?;

    Command::cargo_bin("cog")?
        .arg("get-version")
        .assert()
        .failure()
        .stderr(predicate::str::starts_with("Error: No version yet\n"));

    Ok(())
}

#[sealed_test]
fn get_version_after_bump_ok() -> Result<()> {
    git_init()?;
    git_commit("chore: init")?;
    git_commit("feat(taef): feature")?;

    Command::cargo_bin("cog")?
        .arg("bump")
        .arg("--auto")
        .assert()
        .success();

    Command::cargo_bin("cog")?
        .arg("get-version")
        .assert()
        .success()
        .stdout(predicate::eq(b"0.1.0\n" as &[u8]))
        .stderr(predicate::eq(b"Current version:\n" as &[u8]));

    Ok(())
}

#[sealed_test]
fn get_version_after_bump_fallback_not_used_ok() -> Result<()> {
    git_init()?;
    git_commit("chore: init")?;
    git_commit("feat(taef): feature")?;

    Command::cargo_bin("cog")?
        .arg("bump")
        .arg("--auto")
        .assert()
        .success();

    Command::cargo_bin("cog")?
        .arg("get-version")
        .arg("-f")
        .arg("2.1.0-Test+xx")
        .assert()
        .success()
        .stdout(predicate::eq(b"0.1.0\n" as &[u8]))
        .stderr(predicate::eq(b"Current version:\n" as &[u8]));

    Ok(())
}

#[sealed_test]
fn get_initial_version_of_monorepo_expected_error() -> Result<()> {
    init_monorepo(&mut Settings::default())?;

    Command::cargo_bin("cog")?
        .arg("get-version")
        .assert()
        .failure()
        .stderr(predicate::eq(b"Error: No version yet\n" as &[u8]));

    Ok(())
}

#[sealed_test]
fn get_initial_version_of_monorepo_package_expected_error() -> Result<()> {
    init_monorepo(&mut Settings::default())?;

    Command::cargo_bin("cog")?
        .arg("get-version")
        .arg("--package=one")
        .assert()
        .failure()
        .stderr(predicate::eq(b"Error: No version yet\n" as &[u8]));

    Ok(())
}

#[sealed_test]
fn get_version_of_monorepo_package_having_no_own_version_expected_error() -> Result<()> {
    init_monorepo(&mut Settings::default())?;

    Command::cargo_bin("cog")?
        .arg("bump")
        .arg("--patch")
        .assert()
        .success();

    Command::cargo_bin("cog")?
        .arg("get-version")
        .arg("--package=one")
        .assert()
        .failure()
        .stderr(predicate::eq(b"Error: No version yet\n" as &[u8]));

    // Should differ
    Command::cargo_bin("cog")?
        .arg("get-version")
        .assert()
        .success()
        .stdout(predicate::eq(b"0.0.1\n" as &[u8]))
        .stderr(predicate::eq(b"Current version:\n" as &[u8]));

    Ok(())
}

#[sealed_test]
fn get_version_of_monorepo_package_having_own_version_ok() -> Result<()> {
    init_monorepo(&mut Settings::default())?;

    Command::cargo_bin("cog")?
        .arg("bump")
        .arg("--patch")
        .arg("--package=one")
        .assert()
        .success();

    Command::cargo_bin("cog")?
        .arg("get-version")
        .arg("--package=one")
        .assert()
        .success()
        .stdout(predicate::eq(b"0.0.1\n" as &[u8]))
        .stderr(predicate::eq(b"Current version:\n" as &[u8]));

    // Should differ
    Command::cargo_bin("cog")?
        .arg("get-version")
        .assert()
        .failure()
        .stderr(predicate::eq(b"Error: No version yet\n" as &[u8]));

    Ok(())
}

#[sealed_test]
fn get_version_with_prereleases() -> Result<()> {
    // Arrange
    git_init()?;
    git_commit("chore(version): 0.1.0")?;
    git_tag("0.1.0")?;
    git_commit("chore(version): 1.0.0-alpha.1")?;
    git_tag("1.0.0-alpha.1")?;

    // Act
    let full_version = Command::cargo_bin("cog")?.arg("get-version").assert();
    let prerelease = Command::cargo_bin("cog")?
        .arg("get-version")
        .arg("--include-prereleases")
        .assert();

    // Assert
    full_version.success().stdout("0.1.0\n");
    prerelease.success().stdout("1.0.0-alpha.1\n");

    Ok(())
}

#[sealed_test]
fn get_version_with_tag_prefix() -> Result<()> {
    // Arrange
    git_init()?;
    git_add("tag_prefix = \"v\"", "cog.toml")?;
    git_commit("feat: initial")?;
    git_tag("v0.1.0")?;

    // Act
    let version = Command::cargo_bin("cog")?.arg("get-version").assert();
    let tag = Command::cargo_bin("cog")?
        .arg("get-version")
        .arg("--tag")
        .assert();

    // Arrange
    version.success().stdout("0.1.0\n");
    tag.success().stdout("v0.1.0\n");

    Ok(())
}
