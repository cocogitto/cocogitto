use anyhow::Result;
use assert_cmd::prelude::*;
use std::process::Command;

mod common;

#[test]
#[cfg(not(tarpaulin_include))]
fn init_empty_repo_in_target_dir() -> Result<()> {
    let mut command = Command::cargo_bin("cog")?;

    command.arg("init").arg("test_repo");

    let temp_dir = temp_testdir::TempDir::default();
    std::env::set_current_dir(temp_dir.as_ref())?;

    command.assert().success();

    Ok(())
}

#[test]
#[cfg(not(tarpaulin_include))]
fn init_existing_repo() -> Result<()> {
    let temp_dir = temp_testdir::TempDir::default();

    let mut command = Command::cargo_bin("cog")?;

    command.arg("init").arg("test_repo_existing");

    // Create repo with commits
    let path = temp_dir.join("test_repo_existing");
    std::fs::create_dir(&path)?;
    std::env::set_current_dir(&path)?;
    common::git_init()?;
    common::git_commit("chore: test commit")?;

    std::env::set_current_dir(temp_dir.as_ref())?;

    command.assert().success();

    Ok(())
}

#[test]
#[cfg(not(tarpaulin_include))]
fn fail_if_config_exist() -> Result<()> {
    let temp_dir = temp_testdir::TempDir::default();

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

    Ok(())
}

#[test]
#[cfg(not(tarpaulin_include))]
fn init_current_dir_with_no_arg() -> Result<()> {
    let temp_dir = temp_testdir::TempDir::default();
    let path = temp_dir.join("test_repo_no_args");

    std::fs::create_dir(&path)?;
    let mut command = Command::cargo_bin("cog")?;

    command.arg("init");

    std::env::set_current_dir(path)?;

    command.assert().success();

    Ok(())
}
