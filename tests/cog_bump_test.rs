use anyhow::Result;
use assert_cmd::prelude::*;
use std::process::Command;
use temp_testdir::TempDir;

mod helper;

#[test]
#[cfg(not(tarpaulin))]
fn auto_bump_from_start_ok() -> Result<()> {
    let current_dir = std::env::current_dir()?;
    let mut command = Command::cargo_bin("cog")?;
    command.arg("bump").arg("--auto");
    let temp_dir = TempDir::default();
    std::env::set_current_dir(&temp_dir)?;
    helper::git_init(".")?;
    helper::git_commit("chore: init")?;
    helper::git_commit("feat(taef): feature")?;
    helper::git_commit("fix: bug fix")?;

    command.assert().success();
    assert!(temp_dir.join("CHANGELOG.md").exists());
    helper::assert_tag("0.1.0")?;

    Ok(std::env::set_current_dir(current_dir)?)
}

#[test]
#[cfg(not(tarpaulin))]
fn auto_bump_minor_from_latest_tag() -> Result<()> {
    let current_dir = std::env::current_dir()?;
    let mut command = Command::cargo_bin("cog")?;
    command.arg("bump").arg("--auto");

    let temp_dir = TempDir::default();
    std::env::set_current_dir(&temp_dir)?;
    helper::git_init(".")?;
    helper::git_commit("chore: init")?;
    helper::git_commit("feat(taef): feature")?;
    helper::git_commit("fix: bug fix")?;
    helper::git_tag("1.0.0")?;
    helper::git_commit("feat(taef): feature")?;
    helper::git_commit("feat: feature 1")?;
    helper::git_commit("feat: feature 2")?;

    command.assert().success();
    assert!(temp_dir.join("CHANGELOG.md").exists());
    helper::assert_tag("1.1.0")?;

    Ok(std::env::set_current_dir(current_dir)?)
}

#[test]
#[cfg(not(tarpaulin))]
fn auto_bump_major_from_latest_tag() -> Result<()> {
    let current_dir = std::env::current_dir()?;
    let mut command = Command::cargo_bin("cog")?;
    command.arg("bump").arg("--auto");

    let temp_dir = TempDir::default();
    std::env::set_current_dir(&temp_dir)?;
    helper::git_init(".")?;
    helper::git_commit("chore: init")?;
    helper::git_commit("feat(taef): feature")?;
    helper::git_commit("fix: bug fix")?;
    helper::git_tag("1.0.0")?;
    helper::git_commit("feat!(taef): feature")?;
    helper::git_commit("feat!: feature 1")?;
    helper::git_commit("feat: feature 2")?;

    command.assert().success();
    assert!(temp_dir.join("CHANGELOG.md").exists());
    helper::assert_tag("2.0.0")?;

    Ok(std::env::set_current_dir(current_dir)?)
}

#[test]
#[cfg(not(tarpaulin))]
fn auto_bump_patch_from_latest_tag() -> Result<()> {
    let current_dir = std::env::current_dir()?;
    let mut command = Command::cargo_bin("cog")?;
    command.arg("bump").arg("--auto");

    let temp_dir = TempDir::default();
    std::env::set_current_dir(&temp_dir)?;
    helper::git_init(".")?;
    helper::git_commit("chore: init")?;
    helper::git_commit("feat(taef): feature")?;
    helper::git_commit("fix: bug fix")?;
    helper::git_tag("1.0.0")?;
    helper::git_commit("fix(the_fix): the_fix")?;
    helper::git_commit("fix: fix 1")?;
    helper::git_commit("fix: fix 2")?;

    command.assert().success();
    assert!(temp_dir.join("CHANGELOG.md").exists());
    helper::assert_tag("1.0.1")?;

    Ok(std::env::set_current_dir(current_dir)?)
}

#[test]
#[cfg(not(tarpaulin))]
fn minor_bump() -> Result<()> {
    let current_dir = std::env::current_dir()?;
    let mut command = Command::cargo_bin("cog")?;
    command.arg("bump").arg("--minor");

    let temp_dir = TempDir::default();
    std::env::set_current_dir(&temp_dir)?;
    helper::git_init(".")?;
    helper::git_commit("chore: init")?;
    helper::git_tag("1.0.0")?;
    helper::git_commit("feat: feature")?;

    command.assert().success();
    assert!(temp_dir.join("CHANGELOG.md").exists());
    helper::assert_tag("1.1.0")?;

    Ok(std::env::set_current_dir(current_dir)?)
}

#[test]
#[cfg(not(tarpaulin))]
fn major_bump() -> Result<()> {
    let current_dir = std::env::current_dir()?;
    let mut command = Command::cargo_bin("cog")?;
    command.arg("bump").arg("--major");

    let temp_dir = TempDir::default();
    std::env::set_current_dir(&temp_dir)?;
    helper::git_init(".")?;
    helper::git_commit("chore: init")?;
    helper::git_tag("1.0.0")?;
    helper::git_commit("feat: feature")?;

    command.assert().success();
    assert!(temp_dir.join("CHANGELOG.md").exists());
    helper::assert_tag("2.0.0")?;

    Ok(std::env::set_current_dir(current_dir)?)
}

#[test]
#[cfg(not(tarpaulin))]
fn patch_bump() -> Result<()> {
    let current_dir = std::env::current_dir()?;
    let mut command = Command::cargo_bin("cog")?;
    command.arg("bump").arg("--patch");

    let temp_dir = TempDir::default();
    std::env::set_current_dir(&temp_dir)?;
    helper::git_init(".")?;
    helper::git_commit("chore: init")?;
    helper::git_tag("1.0.0")?;
    helper::git_commit("feat: feature")?;

    command.assert().success();
    assert!(temp_dir.join("CHANGELOG.md").exists());
    helper::assert_tag("1.0.1")?;

    Ok(std::env::set_current_dir(current_dir)?)
}

#[test]
#[cfg(not(tarpaulin))]
fn pre_release_bump() -> Result<()> {
    let current_dir = std::env::current_dir()?;
    let mut command = Command::cargo_bin("cog")?;
    command.arg("bump").arg("--major").arg("--pre").arg("alpha");

    let temp_dir = TempDir::default();
    std::env::set_current_dir(&temp_dir)?;
    helper::git_init(".")?;
    helper::git_commit("chore: init")?;
    helper::git_tag("1.0.0")?;
    helper::git_commit("feat: feature")?;

    command.assert().success();
    assert!(temp_dir.join("CHANGELOG.md").exists());
    helper::assert_tag("2.0.0-alpha")?;

    Ok(std::env::set_current_dir(current_dir)?)
}

#[test]
#[cfg(not(tarpaulin))]
#[cfg(target_os = "linux")]
fn bump_with_hook() -> Result<()> {
    let current_dir = std::env::current_dir()?;
    let mut command = Command::cargo_bin("cog")?;
    command.arg("bump").arg("--major");

    let temp_dir = TempDir::default();
    std::env::set_current_dir(&temp_dir)?;

    std::fs::write("cog.toml", r#"pre_bump_hooks = ["touch %version"]"#)?;

    helper::git_init(".")?;
    helper::git_commit("chore: init")?;
    helper::git_tag("1.0.0")?;
    helper::git_commit("feat: feature")?;

    command.assert().success();
    assert!(temp_dir.join("2.0.0").exists());
    helper::assert_tag("2.0.0")?;

    Ok(std::env::set_current_dir(current_dir)?)
}
