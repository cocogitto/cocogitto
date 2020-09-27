use anyhow::Result;
use assert_cmd::prelude::*;
use std::process::Command;
use temp_testdir::TempDir;

mod helper;

#[test]
#[cfg(not(tarpaulin))]
fn commit_ok() -> Result<()> {
    let current_dir = std::env::current_dir()?;
    let mut command = Command::cargo_bin("coco")?;
    let temp_dir = TempDir::default();
    std::env::set_current_dir(&temp_dir)?;
    helper::git_init(".")?;
    std::fs::write(temp_dir.join("test_file"), "content")?;
    helper::git_add()?;

    command
        .arg("feat")
        .arg("this is a commit message")
        .arg("scope")
        .arg("this is the body")
        .arg("this is the footer");

    command.assert().success();

    Ok(std::env::set_current_dir(current_dir)?)
}

#[test]
#[cfg(not(tarpaulin))]
fn unstaged_changes_commit_err() -> Result<()> {
    let current_dir = std::env::current_dir()?;
    let mut command = Command::cargo_bin("coco")?;
    let temp_dir = TempDir::default();
    std::env::set_current_dir(&temp_dir)?;
    helper::git_init(".")?;
    std::fs::write(temp_dir.join("test_file"), "content")?;

    command
        .arg("feat")
        .arg("this is a commit message")
        .arg("scope")
        .arg("this is the body")
        .arg("this is the footer");

    command.assert().failure();

    Ok(std::env::set_current_dir(current_dir)?)
}

#[test]
#[cfg(not(tarpaulin))]
fn untracked_changes_commit_ok() -> Result<()> {
    let current_dir = std::env::current_dir()?;
    let mut command = Command::cargo_bin("coco")?;
    let temp_dir = TempDir::default();
    std::env::set_current_dir(&temp_dir)?;
    helper::git_init(".")?;
    std::fs::write(temp_dir.join("staged"), "content")?;
    helper::git_add()?;

    std::fs::write(temp_dir.join("untracked"), "content")?;

    command
        .arg("feat")
        .arg("this is a commit message")
        .arg("scope")
        .arg("this is the body")
        .arg("this is the footer");

    command.assert().success();

    Ok(std::env::set_current_dir(current_dir)?)
}

#[test]
#[cfg(not(tarpaulin))]
fn empty_commit_err() -> Result<()> {
    let current_dir = std::env::current_dir()?;
    let mut command = Command::cargo_bin("coco")?;
    let temp_dir = TempDir::default();

    std::env::set_current_dir(&temp_dir.to_path_buf())?;
    helper::git_init(".")?;

    command
        .arg("feat")
        .arg("this is a commit message")
        .arg("scope")
        .arg("this is the body")
        .arg("this is the footer");

    command.assert().failure();

    Ok(std::env::set_current_dir(current_dir)?)
}
