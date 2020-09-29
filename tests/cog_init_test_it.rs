use anyhow::Result;
use assert_cmd::prelude::*;
use cocogitto::CONFIG_PATH;
use helper::*;
use std::process::Command;
use temp_testdir::TempDir;

mod helper;

#[test]
#[cfg(not(tarpaulin))]
fn init_empty_repo_in_target_dir() -> Result<()> {
    // Current dir needs to be reset at the end of each test to get
    // tests to pass on github actions CI
    let current_dir = std::env::current_dir()?;
    let mut command = Command::cargo_bin("cog")?;
    command.arg("init").arg("test_repo");

    let temp_dir = TempDir::default();
    std::env::set_current_dir(&temp_dir)?;

    command.assert().success();
    Ok(std::env::set_current_dir(current_dir)?)
}

#[test]
#[cfg(not(tarpaulin))]
fn init_existing_repo() -> Result<()> {
    let current_dir = std::env::current_dir()?;
    let mut command = Command::cargo_bin("cog")?;
    command.arg("init").arg("test_repo_existing");

    // Create repo with commits
    let temp_dir = TempDir::default();
    std::env::set_current_dir(&temp_dir)?;
    git_init("test_repo_existing")?;
    std::env::set_current_dir(temp_dir.join("test_repo_existing"))?;

    helper::git_commit("chore: test commit")?;

    command.assert().success();
    Ok(std::env::set_current_dir(current_dir)?)
}

#[test]
#[cfg(not(tarpaulin))]
fn fail_if_config_exist() -> Result<()> {
    let current_dir = std::env::current_dir()?;
    let temp_dir = TempDir::default();

    let mut command = Command::cargo_bin("cog")?;
    command.arg("init").arg("test_repo_existing");

    // Create repo with commits
    std::env::set_current_dir(&temp_dir)?;
    helper::git_init("test_repo_existing")?;
    std::fs::write(
        &temp_dir.join("test_repo_existing").join(CONFIG_PATH),
        "[hooks]",
    )?;
    helper::git_commit("chore: test commit")?;

    command.assert().failure();

    Ok(std::env::set_current_dir(current_dir)?)
}

#[test]
#[cfg(not(tarpaulin))]
fn init_current_dir_with_no_arg() -> Result<()> {
    let current_dir = std::env::current_dir()?;
    let mut command = Command::cargo_bin("cog")?;
    command.arg("init");

    let temp_dir = TempDir::default();
    let path = temp_dir.join("test_repo_no_args");
    std::fs::create_dir(&path)?;
    std::env::set_current_dir(&path)?;

    command.assert().success();

    Ok(std::env::set_current_dir(current_dir)?)
}
