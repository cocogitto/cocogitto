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
