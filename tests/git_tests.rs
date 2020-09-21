use crate::{common::create_empty_config, common::git_commit, common::git_init};
use anyhow::Result;
use cocogitto::CocoGitto;

mod common;

#[test]
fn should_open_repo() -> Result<()> {
    git_init()?;
    create_empty_config()?;

    let gitto = CocoGitto::get();

    assert!(gitto.is_ok());
    Ok(())
}

#[test]
fn should_check_commit_history_ok() -> Result<()> {
    git_init()?;
    create_empty_config()?;
    git_commit("feat: a valid commit")?;
    git_commit("chore(test): another valid commit")?;

    let gitto = CocoGitto::get()?;
    assert!(gitto.check().is_ok());
    Ok(())
}

// check stderr and std out instead of exit status
// #[test]
// fn should_check_commit_history_err() -> Result<()> {
//     git_init()?;
//     create_empty_config()?;
//     git_commit("feat: a valid commit")?;
//     git_commit("errored commit")?;
//
//     let gitto = CocoGitto::get()?;
//
//     assert!(gitto.check().is_err());
//     Ok(())
// }
