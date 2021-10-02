use std::path::PathBuf;
use std::process::Command;

use cocogitto::CONFIG_PATH;

use crate::helpers::*;

use anyhow::Result;
use assert_cmd::prelude::*;
use speculoos::prelude::*;

#[test]
fn init_empty_repo_in_target_dir() -> Result<()> {
    // Arrange
    run_test_with_context(|context| {
        // Act
        Command::cargo_bin("cog")?
            .arg("init")
            .arg("test_repo")
            .assert()
            .success();

        // Assert
        let repo_directory = context.test_dir.join("test_repo");
        assert_that(&repo_directory).exists();
        Ok(())
    })
}

#[test]
fn init_existing_repo() -> Result<()> {
    run_test_with_context(|context| {
        // Arrange
        git_init_and_set_current_path("test_repo_existing")?;
        assert_that(&context.test_dir.join("test_repo_existing")).exists();
        git_commit("chore: test commit")?;

        // Act
        Command::cargo_bin("cog")?
            .arg("init")
            .arg("test_repo_existing")
            // Assert
            .assert()
            .success();
        Ok(())
    })
}

#[test]
fn fail_if_config_exist() -> Result<()> {
    run_test_with_context(|context| {
        // Arrange
        git_init_and_set_current_path("test_repo_existing")?;
        std::fs::write(
            &context
                .test_dir
                .join("test_repo_existing")
                .join(CONFIG_PATH),
            "[hooks]",
        )?;
        git_commit("chore: test commit")?;

        // Act
        Command::cargo_bin("cog")?
            .arg("init")
            .arg("test_repo_existing")
            // Assert
            .assert()
            .stdout("Found git repository in \"test_repo_existing\", skipping initialisation\n")
            .success();

        assert_that(&PathBuf::from("cog.toml")).exists();
        Ok(())
    })
}

#[test]
fn init_current_dir_with_no_arg() -> Result<()> {
    run_test_with_context(|context| {
        // Arrange
        let path = context.test_dir.join("test_repo_no_args");
        std::fs::create_dir(&path)?;
        std::env::set_current_dir(&path)?;

        // Act
        Command::cargo_bin("cog")?
            .arg("init")
            // Assert
            .assert()
            .success();
        Ok(())
    })
}
