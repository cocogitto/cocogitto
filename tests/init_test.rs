use anyhow::Result;
use assert_cmd::prelude::*;
use std::process::Command;
use temp_testdir::TempDir;

mod common;

#[test]
#[cfg(not(tarpaulin))]
fn init_empty_repo_in_target_dir() -> Result<()> {
    // Current dir needs to be reset at the end of each test to get
    // tests to pass on github actions CI
    let current_dir = std::env::current_dir()?;
    let mut command = Command::cargo_bin("cog")?;
    command.arg("init").arg("test_repo");

    let temp_dir = TempDir::default();
    std::env::set_current_dir(temp_dir.as_ref())?;

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
    let path = temp_dir.join("test_repo_existing");
    std::fs::create_dir(&path)?;
    std::env::set_current_dir(&path)?;

    common::git_init()?;
    common::git_commit("chore: test commit")?;

    std::env::set_current_dir(temp_dir.as_ref())?;

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
    let path = temp_dir.join("test_repo_existing");
    std::fs::create_dir(&path)?;
    std::fs::write(&path.join("coco.toml"), "[hooks]")?;
    std::env::set_current_dir(&path)?;
    common::git_init()?;
    common::git_commit("chore: test commit")?;

    std::env::set_current_dir(temp_dir.as_ref())?;

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
