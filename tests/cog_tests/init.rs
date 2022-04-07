use std::path::{Path, PathBuf};
use std::process::Command;
use std::str::FromStr;

use cocogitto::CONFIG_PATH;

use crate::helpers::*;

use anyhow::Result;
use assert_cmd::prelude::*;
use sealed_test::prelude::*;
use speculoos::prelude::*;

#[sealed_test]
fn init_empty_repo_in_target_dir() -> Result<()> {
    // Act
    Command::cargo_bin("cog")?
        .arg("init")
        .arg("test_repo")
        .assert()
        .success();

    // Assert
    assert_that!(Path::new("test_repo")).exists();
    Ok(())
}

#[sealed_test]
fn init_existing_repo() -> Result<()> {
    // Arrange
    git_init_and_set_current_path("test_repo_existing")?;
    git_commit("chore: test commit")?;

    // Act
    Command::cargo_bin("cog")?
        .arg("init")
        // Assert
        .assert()
        .success();
    Ok(())
}

#[sealed_test]
fn fail_if_config_exist() -> Result<()> {
    // Arrange
    git_init_and_set_current_path("test_repo_existing")?;
    std::fs::write(PathBuf::from_str(CONFIG_PATH)?, "[hooks]")?;
    git_commit("chore: test commit")?;

    // Act
    Command::cargo_bin("cog")?
        .arg("init")
        .arg("test_repo_existing")
        // Assert
        .assert()
        .stderr("Found git repository in \"test_repo_existing\", skipping initialisation\n")
        .success();

    assert_that!(PathBuf::from("cog.toml")).exists();
    Ok(())
}

#[sealed_test]
fn init_current_dir_with_no_arg() -> Result<()> {
    // Act
    Command::cargo_bin("cog")?
        .arg("init")
        // Assert
        .assert()
        .success();
    Ok(())
}
