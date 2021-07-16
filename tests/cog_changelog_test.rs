use anyhow::Result;
use assert_cmd::Command;
use predicates::prelude::{predicate, PredicateBooleanExt};
use tempfile::TempDir;

mod helper;

#[test]
#[cfg(not(tarpaulin))]
fn get_changelog_from_untagged_repo() -> Result<()> {
    let current_dir = std::env::current_dir()?;
    let mut command = Command::cargo_bin("cog")?;
    command.arg("changelog");

    let temp_dir = TempDir::new()?;
    std::env::set_current_dir(&temp_dir)?;

    helper::git_init(".")?;
    helper::git_commit("chore: init")?;
    helper::git_commit("feat(taef): feature")?;
    helper::git_commit("fix: bug fix")?;

    command
        .assert()
        .success()
        .stdout(predicate::str::contains("Bug Fixes"))
        .stdout(predicate::str::contains("Features"));

    Ok(std::env::set_current_dir(current_dir)?)
}

#[test]
#[cfg(not(tarpaulin))]
fn get_changelog_from_tagged_repo() -> Result<()> {
    let current_dir = std::env::current_dir()?;
    let mut command = Command::cargo_bin("cog")?;
    command.arg("changelog");

    let temp_dir = TempDir::new()?;
    std::env::set_current_dir(&temp_dir)?;

    helper::git_init(".")?;
    helper::git_commit("chore: init")?;
    helper::git_commit("feat(taef): feature")?;
    helper::git_tag("1.0.0")?;
    helper::git_commit("fix: bug fix")?;

    command
        .assert()
        .success()
        .stdout(predicate::str::contains("Bug Fixes"))
        .stdout(predicate::str::contains("Features").not());

    Ok(std::env::set_current_dir(current_dir)?)
}

#[test]
#[cfg(not(tarpaulin))]
fn get_changelog_from_at_tag() -> Result<()> {
    let current_dir = std::env::current_dir()?;
    let mut command = Command::cargo_bin("cog")?;
    command.arg("changelog");
    command.arg("--at");
    command.arg("1.0.0");

    let temp_dir = TempDir::new()?;
    std::env::set_current_dir(&temp_dir)?;

    helper::git_init(".")?;
    helper::git_commit("chore: init")?;
    helper::git_commit("feat(taef): feature")?;
    helper::git_commit("feat: feature 2")?;
    helper::git_tag("1.0.0")?;
    helper::git_commit("fix: bug fix")?;
    helper::git_log()?;
    command
        .assert()
        .success()
        .stdout(predicate::str::contains("Bug Fixes").not());

    Ok(std::env::set_current_dir(current_dir)?)
}
