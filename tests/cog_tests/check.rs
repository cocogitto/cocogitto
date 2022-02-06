use crate::helpers::*;

use anyhow::Result;
use assert_cmd::Command;
use cmd_lib::{init_builtin_logger, run_cmd};
use cocogitto::CocoGitto;
use predicates::prelude::predicate;
use sealed_test::prelude::*;
use speculoos::assert_that;
use speculoos::prelude::ResultAssertions;

#[sealed_test]
fn cog_check_ok() -> Result<()> {
    // Arrange
    git_init()?;
    git_commit("chore: init")?;
    git_commit("feat: feature")?;
    git_commit("fix: bug fix")?;

    // Act
    Command::cargo_bin("cog")?
        .arg("check")
        // Assert
        .assert()
        .success()
        .stdout(predicate::str::contains("No errored commits"));
    Ok(())
}

#[sealed_test]
fn cog_check_failure() -> Result<()> {
    // Arrange
    git_init()?;
    git_commit("chore: init")?;
    git_commit("toto: feature")?;
    git_commit("fix: bug fix")?;

    // Act
    Command::cargo_bin("cog")?
        .arg("check")
        // Assert
        .assert()
        .failure()
        .stderr(predicate::str::contains("Found 1 non compliant commits"));
    Ok(())
}

#[sealed_test]
fn cog_check_from_latest_tag_ok() -> Result<()> {
    // Arrange
    git_init()?;
    git_commit("chore: init")?;
    git_commit("toto: errored commit")?;
    git_commit("feat: feature")?;
    git_tag("1.0.0")?;
    git_commit("fix: bug fix")?;

    // Act
    Command::cargo_bin("cog")?
        .arg("check")
        .arg("--from-latest-tag")
        // Assert
        .assert()
        .success()
        .stdout(predicate::str::contains("No errored commits"));
    Ok(())
}

#[sealed_test]
fn cog_check_from_latest_tag_failure() -> Result<()> {
    // Arrange
    git_init()?;
    git_commit("chore: init")?;
    git_commit("toto: errored commit")?;
    git_commit("feat: feature")?;
    git_tag("1.0.0")?;
    git_commit("fix: bug fix")?;
    git_commit("toto: africa")?;

    // Act
    Command::cargo_bin("cog")?
        .arg("check")
        .arg("--from-latest-tag")
        // Assert
        .assert()
        .failure()
        .stderr(predicate::str::contains("Found 1 non compliant commits"));
    Ok(())
}

#[sealed_test]
fn shallow_clone_check_err() -> Result<()> {
    // Arrange
    init_builtin_logger();
    let current_dir = std::env::current_dir()?;
    let current_dir = current_dir.to_str().expect("Cannot get current directory");
    let a = format!("file://{current_dir}/a");

    run_cmd!(
        mkdir a;
        cd a;
        git init;
        git commit --allow-empty -m "feat: Add the 1 feature";
        git commit --allow-empty -m "feat: Add the 2 feature";
        git commit --allow-empty -m "feat: Add the 3 feature";
        git commit --allow-empty -m "feat: Add the 4 feature";
        git commit --allow-empty -m "feat: Add the 5 feature";
        cd $current_dir;
        git clone --depth 3 "$a" b;
    )?;

    std::env::set_current_dir("b")?;

    let cocogitto = CocoGitto::get()?;

    // Act
    let check_result = cocogitto.check(false);

    // Assert
    assert_that!(check_result).is_ok();

    Ok(())
}

#[sealed_test]
fn shallow_clone_check_ok() -> Result<()> {
    // Arrange
    init_builtin_logger();
    let current_dir = std::env::current_dir()?;
    let current_dir = current_dir.to_str().expect("Cannot get current directory");
    let a = format!("file://{current_dir}/a");

    run_cmd!(
        mkdir a;
        cd a;
        git init;
        git commit --allow-empty -m "feat: Add the 1 feature";
        git commit --allow-empty -m "feat: Add the 2 feature";
        git commit --allow-empty -m "feat: Add the 3 feature";
        git commit --allow-empty -m "feat: Add the 4 feature";
        git commit --allow-empty -m "toto";
        cd $current_dir;
        git clone --depth 3 "$a" b;
    )?;

    std::env::set_current_dir("b")?;

    // Act
    let cocogitto = CocoGitto::get()?;

    let result = cocogitto.check(false);

    // Assert
    assert_that!(result).is_err();

    Ok(())
}

#[sealed_test]
fn shallow_clone_from_latest_tag_check_ok() -> Result<()> {
    // Arrange
    init_builtin_logger();
    let current_dir = std::env::current_dir()?;
    let current_dir = current_dir.to_str().expect("Cannot get current directory");
    let a = format!("file://{current_dir}/a");

    run_cmd!(
        mkdir a;
        cd a;
        git init;
        git commit --allow-empty -m "feat: Add the 1 feature";
        git commit --allow-empty -m "feat: Add the 2 feature";
        git tag 1.0.0;
        git commit --allow-empty -m "feat: Add the 3 feature";
        git commit --allow-empty -m "feat: Add the 4 feature";
        git commit --allow-empty -m "feat: Add the 5 feature";
        cd $current_dir;
        git clone --depth 3 "$a" b;
    )?;

    std::env::set_current_dir("b")?;

    // Act
    let cocogitto = CocoGitto::get()?;

    let check_result = cocogitto.check(true);

    // Assert
    assert_that!(check_result).is_ok();

    Ok(())
}

#[sealed_test]
fn shallow_clone_from_latest_tag_check_err() -> Result<()> {
    // Arrange
    init_builtin_logger();
    let current_dir = std::env::current_dir()?;
    let current_dir = current_dir.to_str().expect("Cannot get current directory");
    let a = format!("file://{current_dir}/a");

    run_cmd!(
        mkdir a;
        cd a;
        git init;
        git commit --allow-empty -m "feat: Add the 1 feature";
        git commit --allow-empty -m "feat: Add the 2 feature";
        git tag 1.0.0;
        git commit --allow-empty -m "feat: Add the 3 feature";
        git commit --allow-empty -m "feat: Add the 4 feature";
        git commit --allow-empty -m "toto";
        cd $current_dir;
        git clone --depth 3 "$a" b;
    )?;

    std::env::set_current_dir("b")?;

    let cocogitto = CocoGitto::get()?;

    // Act
    let check_result = cocogitto.check(true);

    // Assert
    assert_that!(check_result).is_err();

    Ok(())
}
