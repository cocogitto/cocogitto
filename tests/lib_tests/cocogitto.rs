use crate::helpers::*;

use anyhow::Result;
use cmd_lib::run_cmd;
use cocogitto::CocoGitto;
use sealed_test::prelude::*;
use speculoos::prelude::*;

#[sealed_test]
fn open_repo_ok() -> Result<()> {
    // Arrange
    git_init_and_set_current_path("open_repo_ok")?;
    create_empty_config()?;

    // Act
    let cocogitto = CocoGitto::get();

    // Assert
    assert_that!(cocogitto).is_ok();
    Ok(())
}

#[sealed_test]
fn open_repo_err() -> Result<()> {
    // Arrange
    create_empty_config()?;

    // Act
    let cocogitto = CocoGitto::get();

    // Assert
    assert_that!(cocogitto).is_err();
    Ok(())
}

#[sealed_test]
fn check_commit_history_ok() -> Result<()> {
    // Arrange
    git_init()?;
    git_commit("feat: a valid commit")?;
    git_commit("chore(test): another valid commit")?;
    let cocogitto = CocoGitto::get()?;

    // Act
    let check = cocogitto.check(false, false, None);

    // Assert
    assert_that!(check).is_ok();
    Ok(())
}

#[sealed_test]
fn check_commit_history_err_with_merge_commit() -> Result<()> {
    // Arrange
    git_init()?;
    git_commit("feat: a valid commit")?;
    git_commit("Merge feature one into main")?;
    let cocogitto = CocoGitto::get()?;

    // Act
    let check = cocogitto.check(false, false, None);

    // Assert
    assert_that!(check).is_err();
    Ok(())
}

#[sealed_test]
fn check_commit_history_ok_with_merge_commit_ignored() -> Result<()> {
    // Arrange
    git_init()?;
    git_commit("feat: a valid commit")?;
    run_cmd!(git checkout -b branch;)?;
    git_commit("feat: commit on another branch")?;
    run_cmd!(
        git checkout -;
        git merge branch --no-ff;
        git --no-pager log;
    )?;

    let cocogitto = CocoGitto::get()?;

    // Act
    let check = cocogitto.check(false, true, None);

    // Assert
    assert_that!(check).is_ok();
    Ok(())
}

#[sealed_test]
fn check_commit_history_err() -> Result<()> {
    // Arrange
    git_init_and_set_current_path("commit_history_err")?;
    create_empty_config()?;
    git_commit("feat: a valid commit")?;
    git_commit("errored commit")?;
    let cocogitto = CocoGitto::get()?;

    // Act
    let check = cocogitto.check(false, false, None);

    // Assert
    assert_that!(check).is_err();
    Ok(())
}

#[sealed_test]
fn check_commit_ok_from_latest_tag() -> Result<()> {
    // Arrange
    git_init_and_set_current_path("commit_ok_from_tag")?;

    create_empty_config()?;
    git_commit("this one should not be picked")?;
    git_tag("0.1.0")?;
    git_commit("feat: another commit")?;
    let cocogitto = CocoGitto::get()?;

    // Act
    let check = cocogitto.check(true, false, None);

    // Assert
    assert_that!(check).is_ok();
    Ok(())
}

#[sealed_test]
fn check_commit_err_from_latest_tag() -> Result<()> {
    // Arrange
    git_init_and_set_current_path("commit_err_from_tag")?;
    create_empty_config()?;
    git_commit("this one should not be picked")?;
    git_tag("0.1.0")?;
    git_commit("Oh no!")?;
    let cocogitto = CocoGitto::get()?;

    // Act
    let check = cocogitto.check(true, false, None);

    // Assert
    assert_that!(check).is_err();
    Ok(())
}

#[sealed_test]
fn long_commit_summary_does_not_panic() -> Result<()> {
    git_init()?;
    let message = "aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa…"
        .to_string();

    let cocogitto = CocoGitto::get()?;
    git_add("Hello", "file")?;
    cocogitto.conventional_commit("feat", None, message, None, None, false, false)?;

    let check = cocogitto.check(false, false, None);

    assert_that!(check.is_ok());
    Ok(())
}

#[sealed_test]
fn check_commit_ok_commit_range() -> Result<()> {
    // Arrange
    git_init()?;
    let range_start = git_commit("feat: a valid commit")?;
    let range_end = git_commit("chore(test): another valid commit")?;
    let range = format!("{range_start}..{range_end}");
    let cocogitto = CocoGitto::get()?;

    // Act
    let check = cocogitto.check(true, false, Some(range));

    // Assert
    assert_that!(check).is_ok();
    Ok(())
}

#[sealed_test]
fn check_commit_err_commit_range() -> Result<()> {
    // Arrange
    git_init_and_set_current_path("commit_history_err")?;
    create_empty_config()?;
    let range_start = git_commit("feat: a valid commit")?;
    let range_end = git_commit("errored commit")?;
    let range = format!("{range_start}..{range_end}");
    let cocogitto = CocoGitto::get()?;

    // Act
    let check = cocogitto.check(true, false, Some(range));

    // Assert
    assert_that!(check).is_err();
    Ok(())
}

#[sealed_test]
fn check_commit_range_err_with_merge_commit() -> Result<()> {
    // Arrange
    git_init()?;
    let range_start = git_commit("feat: a valid commit")?;
    let range_end = git_commit("Merge feature one into main")?;
    let range = format!("{range_start}..{range_end}");
    let cocogitto = CocoGitto::get()?;

    // Act
    let check = cocogitto.check(false, false, Some(range));

    // Assert
    assert_that!(check).is_err();
    Ok(())
}

#[sealed_test]
fn check_commit_range_ok_with_merge_commit_ignored() -> Result<()> {
    // Arrange
    git_init()?;
    let range_start = git_commit("feat: a valid commit")?;
    run_cmd!(git checkout -b branch;)?;
    git_commit("feat: commit on another branch")?;
    run_cmd!(
        git checkout -;
        git merge branch --no-ff;
        git --no-pager log;
    )?;
    let range_end = git_commit("chore(test): another valid commit")?;
    let range = format!("{range_start}..{range_end}");
    let cocogitto = CocoGitto::get()?;

    // Act
    let check = cocogitto.check(false, true, Some(range));

    // Assert
    assert_that!(check).is_ok();
    Ok(())
}
