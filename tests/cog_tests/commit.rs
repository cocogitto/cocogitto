use std::process::Command;
use std::{ffi::OsStr, fs};

use crate::helpers::*;

use anyhow::Result;
use assert_cmd::prelude::*;
use cmd_lib::run_cmd;
use indoc::{formatdoc, indoc};
use pretty_assertions::assert_eq;
use sealed_test::prelude::*;

#[sealed_test]
fn commit_ok() -> Result<()> {
    // Arrange
    git_init()?;
    git_add("content", "test_file")?;

    // Act
    Command::cargo_bin("cog")?
        .arg("commit")
        .arg("feat")
        .arg("this is a commit message")
        .arg("scope")
        // Assert
        .assert()
        .success();
    Ok(())
}

#[sealed_test]
fn commit_fail_if_not_a_repository() -> Result<()> {
    // Act
    let output = Command::cargo_bin("cog")?
        .arg("commit")
        .arg("feat")
        .arg("this is a commit message")
        .arg("scope")
        .output()?;

    let stderr = String::from_utf8(output.stderr)?;

    // Assert
    let current_dir = std::env::current_dir()?;
    let current_dir: &OsStr = current_dir.as_os_str();
    let current_dir = current_dir.to_str().expect("utf8 error");

    assert_eq!(
        stderr,
        formatdoc!(
            "Error: failed to open repository

        cause: could not find repository at '{}'; class=Repository (6); code=NotFound (-3)

        ",
            current_dir
        )
    );
    Ok(())
}

#[sealed_test]
fn unstaged_changes_commit_err() -> Result<()> {
    // Arrange
    git_init()?;
    std::fs::write("test_file", "content")?;

    // Act
    let output = Command::cargo_bin("cog")?
        .arg("commit")
        .arg("feat")
        .arg("this is a commit message")
        .arg("scope")
        .output()?;

    let stderr = String::from_utf8(output.stderr)?;

    // Assert
    assert_eq!(
        stderr,
        indoc!(
            "Error: Untracked files :
                \tnew: test_file

                nothing added to commit but untracked files present (use \"git add\" to track)\n\n"
        )
    );

    Ok(())
}

#[sealed_test]
fn untracked_changes_commit_ok() -> Result<()> {
    // Arrange
    git_init()?;
    git_add("content", "staged")?;
    std::fs::write("untracked", "content")?;

    // Act
    Command::cargo_bin("cog")?
        .arg("commit")
        .arg("feat")
        .arg("this is a commit message")
        .arg("scope")
        // Assert
        .assert()
        .success();
    Ok(())
}

#[sealed_test]
fn empty_commit_err() -> Result<()> {
    // Arrange
    git_init()?;

    // Act
    let output = Command::cargo_bin("cog")?
        .arg("commit")
        .arg("feat")
        .arg("this is a commit message")
        .arg("scope")
        .output()?;

    let stderr = String::from_utf8(output.stderr)?;

    // Assert
    assert_eq!(
        stderr,
        "Error: nothing to commit (create/copy files and use \"git add\" to track)\n\n"
    );

    Ok(())
}

#[sealed_test]
fn commit_with_default_skip_ci_ok() -> Result<()> {
    // Arrange
    git_init()?;
    git_add("content", "test_file")?;

    // Act
    Command::cargo_bin("cog")?
        .arg("commit")
        .arg("--skip-ci")
        .arg("feat")
        .arg("this is a commit message")
        .arg("scope")
        // Assert
        .assert()
        .success();

    let commit_message = git_log_head_message()?;

    assert_eq!(
        commit_message,
        "feat(scope): this is a commit message [skip ci]"
    );

    Ok(())
}

#[sealed_test]
fn commit_with_cog_toml_defined_skip_ci_ok() -> Result<()> {
    // Arrange
    git_init()?;
    git_add("content", "test_file")?;
    git_add("skip_ci = \"[ci-skip]\" ", "cog.toml")?;

    // Act
    Command::cargo_bin("cog")?
        .arg("commit")
        .arg("--skip-ci")
        .arg("feat")
        .arg("this is a commit message")
        .arg("scope")
        // Assert
        .assert()
        .success();

    let commit_message = git_log_head_message()?;

    assert_eq!(
        commit_message,
        "feat(scope): this is a commit message [ci-skip]"
    );

    Ok(())
}

#[sealed_test]
fn commit_with_skip_ci_override_option_takes_precedence() -> Result<()> {
    // Arrange
    git_init()?;
    git_add("content", "test_file")?;
    git_add("skip_ci = \"[ci-skip]\" ", "cog.toml")?;

    // Act
    Command::cargo_bin("cog")?
        .arg("commit")
        .arg("--skip-ci-override")
        .arg("[skip-ci-override]")
        .arg("feat")
        .arg("this is a commit message")
        .arg("scope")
        // Assert
        .assert()
        .success();

    let commit_message = git_log_head_message()?;

    assert_eq!(
        commit_message,
        "feat(scope): this is a commit message [skip-ci-override]"
    );

    Ok(())
}

#[sealed_test]
fn add_option_git_commit_ok() -> Result<()> {
    // Arrange
    git_init()?;
    run_cmd!(
        echo "new file" > new_file;
        echo "dot file" > .dotfile;
    )?;

    Command::cargo_bin("cog")?
        .arg("commit")
        .arg("-a")
        .arg("feat")
        .arg("feature")
        .assert()
        .success();

    let commit_message = git_log_head_message()?;

    assert_eq!(commit_message, "feat: feature");

    Command::new("git")
        .arg("status")
        .arg("-s")
        .assert()
        .success()
        .stdout(indoc!(""));

    Ok(())
}

#[sealed_test]
fn update_option_git_commit_ok() -> Result<()> {
    // Arrange
    git_init()?;

    run_cmd!(
        echo "existing file" > existing_file;
        git add .;
        git commit -m "feat: existing file";
        echo "update existing file" > existing_file;
        echo "new file" > new_file;
    )?;

    Command::cargo_bin("cog")?
        .arg("commit")
        .arg("-u")
        .arg("feat")
        .arg("update existing file")
        .assert()
        .success();

    let commit_message = git_log_head_message()?;

    assert_eq!(commit_message, "feat: update existing file");

    Command::new("git")
        .arg("status")
        .arg("-s")
        .assert()
        .success()
        .stdout(indoc!("?? new_file\n"));

    Ok(())
}

#[sealed_test]
fn should_error_on_disabled_commit_error() -> Result<()> {
    // Arrange
    git_init()?;
    git_add("content", "test_file")?;
    let settings = r#"
        [commit_types]
        perf = {}
        "#;

    fs::write("cog.toml", settings)?;

    // Act
    Command::cargo_bin("cog")?
        .arg("commit")
        .arg("perf")
        .arg("fails at the speed of light")
        // Assert
        .assert()
        .failure();
    Ok(())
}
