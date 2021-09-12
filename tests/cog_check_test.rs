use anyhow::Result;
use assert_cmd::Command;
use predicates::prelude::predicate;
use tempfile::TempDir;

mod helper;

#[test]
#[cfg(not(tarpaulin))]
fn cog_check_ok() -> Result<()> {
    let current_dir = std::env::current_dir()?;
    let mut command = Command::cargo_bin("cog")?;
    command.arg("check");

    let temp_dir = TempDir::new()?;
    std::env::set_current_dir(&temp_dir)?;

    helper::git_init(".")?;
    helper::git_commit("chore: init")?;
    helper::git_commit("feat: feature")?;
    helper::git_commit("fix: bug fix")?;

    command
        .assert()
        .success()
        .stdout(predicate::str::contains("No errored commits"));

    Ok(std::env::set_current_dir(current_dir)?)
}

#[test]
#[cfg(not(tarpaulin))]
fn cog_check_failure() -> Result<()> {
    let current_dir = std::env::current_dir()?;
    let mut command = Command::cargo_bin("cog")?;
    command.arg("check");

    let temp_dir = TempDir::new()?;
    std::env::set_current_dir(&temp_dir)?;

    helper::git_init(".")?;
    helper::git_commit("chore: init")?;
    helper::git_commit("toto: feature")?;
    helper::git_commit("fix: bug fix")?;

    command
        .assert()
        .failure()
        .stderr(predicate::str::contains("Commit type `toto` not allowed"));

    Ok(std::env::set_current_dir(current_dir)?)
}

#[test]
#[cfg(not(tarpaulin))]
fn cog_check_from_latest_tag_ok() -> Result<()> {
    let current_dir = std::env::current_dir()?;
    let mut command = Command::cargo_bin("cog")?;
    command.arg("check");
    command.arg("--from-latest-tag");

    let temp_dir = TempDir::new()?;
    std::env::set_current_dir(&temp_dir)?;

    helper::git_init(".")?;
    helper::git_commit("chore: init")?;
    helper::git_commit("toto: errored commit")?;
    helper::git_commit("feat: feature")?;
    helper::git_tag("1.0.0")?;
    helper::git_commit("fix: bug fix")?;

    command
        .assert()
        .success()
        .stdout(predicate::str::contains("No errored commits"));

    Ok(std::env::set_current_dir(current_dir)?)
}

#[test]
#[cfg(not(tarpaulin))]
fn cog_check_from_latest_tag_failure() -> Result<()> {
    let current_dir = std::env::current_dir()?;
    let mut command = Command::cargo_bin("cog")?;
    command.arg("check");
    command.arg("--from-latest-tag");

    let temp_dir = TempDir::new()?;
    std::env::set_current_dir(&temp_dir)?;

    helper::git_init(".")?;
    helper::git_commit("chore: init")?;
    helper::git_commit("toto: errored commit")?;
    helper::git_commit("feat: feature")?;
    helper::git_tag("1.0.0")?;
    helper::git_commit("fix: bug fix")?;
    helper::git_commit("toto: africa")?;

    command
        .assert()
        .failure()
        .stderr(predicate::str::contains("Commit type `toto` not allowed"));

    Ok(std::env::set_current_dir(current_dir)?)
}
