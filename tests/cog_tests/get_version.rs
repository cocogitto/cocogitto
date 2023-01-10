use std::process::Command;

use crate::helpers::*;

use anyhow::Result;
use assert_cmd::prelude::*;
use predicates::prelude::predicate;
use sealed_test::prelude::*;

#[sealed_test]
fn get_initial_version_ok() -> Result<()> {
    git_init()?;
    git_commit("chore: init")?;
    git_commit("feat(taef): feature")?;

    Command::cargo_bin("cog")?
        .arg("get-version")
        .assert()
        .success()
        .stdout(predicate::eq(b"0.0.0\n" as &[u8]));

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
fn get_initial_version_fails_as_expected_error() -> Result<()> {
    git_init()?;
    git_commit("chore: init")?;
    git_commit("feat(taef): feature")?;

    Command::cargo_bin("cog")?
        .arg("get-version")
        .arg("--disable-fallback")
        .assert()
        .failure()
        .stderr(predicate::str::starts_with("Error: No version given\n"));

    Ok(())
}

#[sealed_test]
fn get_initial_version_fallback_conflicts_error() -> Result<()> {
    Command::cargo_bin("cog")?
        .arg("get-version")
        .arg("-f")
        .arg("1.1.1")
        .arg("--disable-fallback")
        .assert()
        .failure()
        .stderr(predicate::str::contains(
            "'--fallback <FALLBACK>' cannot be used with '--disable-fallback'",
        ));

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
