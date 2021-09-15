use anyhow::Result;
use cocogitto::CocoGitto;
use helper::*;
use tempfile::TempDir;

mod helper;
use helper::run_test_with_context;

#[test]
fn open_repo_ok() -> Result<()> {
    let tempdir = TempDir::new()?;
    std::env::set_current_dir(&tempdir.path())?;
    git_init_with_path("open_repo_ok")?;
    std::env::set_current_dir(std::env::current_dir()?.join("open_repo_ok"))?;
    create_empty_config()?;

    let gitto = CocoGitto::get();

    assert!(gitto.is_ok());
    Ok(())
}

#[test]
fn open_repo_err() -> Result<()> {
    let tmp = TempDir::new()?;
    std::env::set_current_dir(&tmp)?;
    std::fs::create_dir(&tmp.path().join("not_a_repo"))?;
    create_empty_config()?;

    let gitto = CocoGitto::get();

    assert!(gitto.is_err());
    Ok(())
}

#[test]
fn check_commit_history_ok() -> Result<()> {
    let tmp = TempDir::new()?;
    std::env::set_current_dir(&tmp)?;
    git_init_with_path("commit_history_ok")?;
    std::env::set_current_dir(&tmp.path().join("commit_history_ok"))?;

    create_empty_config()?;
    git_commit("feat: a valid commit")?;
    git_commit("chore(test): another valid commit")?;

    let gitto = CocoGitto::get()?;

    assert!(gitto.check(false).is_ok());
    Ok(())
}

#[test]
fn check_commit_history_err() -> Result<()> {
    let tmp = TempDir::new()?;
    std::env::set_current_dir(&tmp)?;
    git_init_with_path("commit_history_err")?;
    std::env::set_current_dir(&tmp.path().join("commit_history_err"))?;

    create_empty_config()?;
    git_commit("feat: a valid commit")?;
    git_commit("errored commit")?;

    let gitto = CocoGitto::get()?;

    assert!(gitto.check(false).is_err());
    Ok(())
}

#[test]
fn check_commit_ok_from_latest_tag() -> Result<()> {
    let tmp = TempDir::new()?;
    std::env::set_current_dir(&tmp)?;
    git_init_with_path("commit_ok_from_tag")?;
    std::env::set_current_dir(&tmp.path().join("commit_ok_from_tag"))?;

    create_empty_config()?;
    git_commit("this one should not be picked")?;
    git_tag("0.1.0")?;
    git_commit("feat: another commit")?;

    let gitto = CocoGitto::get()?;

    assert!(gitto.check(true).is_ok());
    Ok(())
}

#[test]
fn check_commit_err_from_latest_tag() -> Result<()> {
    let tmp = TempDir::new()?;
    std::env::set_current_dir(&tmp)?;
    git_init_with_path("commit_err_from_tag")?;
    std::env::set_current_dir(&tmp.path().join("commit_err_from_tag"))?;

    create_empty_config()?;
    git_commit("this one should not be picked")?;
    git_tag("0.1.0")?;
    git_commit("Oh no!")?;

    let gitto = CocoGitto::get()?;

    assert!(gitto.check(true).is_err());
    Ok(())
}
