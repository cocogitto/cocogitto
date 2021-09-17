use anyhow::Result;
use cocogitto::CocoGitto;
use helper::*;

pub mod helper;

use conventional_commit_parser::commit::CommitType;
use helper::run_test_with_context;

#[test]
fn open_repo_ok() -> Result<()> {
    run_test_with_context(|_| {
        // Arrange
        git_init_and_set_current_path("open_repo_ok")?;
        create_empty_config()?;

        // Act
        let cocogitto = CocoGitto::get();

        // Assert
        assert!(cocogitto.is_ok());
        Ok(())
    })
}

#[test]
fn open_repo_err() -> Result<()> {
    run_test_with_context(|context| {
        // Arrange
        std::fs::create_dir(context.test_dir.join("not_a_repo"))?;
        create_empty_config()?;

        // Act
        let cocogitto = CocoGitto::get();

        // Assert
        assert!(cocogitto.is_err());
        Ok(())
    })
}

#[test]
fn check_commit_history_ok() -> Result<()> {
    run_test_with_context(|_| {
        // Arrange
        git_init_and_set_current_path("commit_history_ok")?;
        create_empty_config()?;
        git_commit("feat: a valid commit")?;
        git_commit("chore(test): another valid commit")?;
        let cocogitto = CocoGitto::get()?;

        // Act
        let check = cocogitto.check(false);

        // Assert
        assert!(check.is_ok());
        Ok(())
    })
}

#[test]
fn check_commit_history_err() -> Result<()> {
    run_test_with_context(|_| {
        // Arrange
        git_init_and_set_current_path("commit_history_err")?;
        create_empty_config()?;
        git_commit("feat: a valid commit")?;
        git_commit("errored commit")?;
        let cocogitto = CocoGitto::get()?;

        // Act
        let check = cocogitto.check(false);

        // Assert
        assert!(check.is_err());
        Ok(())
    })
}

#[test]
fn check_commit_ok_from_latest_tag() -> Result<()> {
    run_test_with_context(|_| {
        // Arrange
        git_init_and_set_current_path("commit_ok_from_tag")?;

        create_empty_config()?;
        git_commit("this one should not be picked")?;
        git_tag("0.1.0")?;
        git_commit("feat: another commit")?;
        let cocogitto = CocoGitto::get()?;

        // Act
        let check = cocogitto.check(true);

        // Assert
        assert!(check.is_ok());
        Ok(())
    })
}

#[test]
fn check_commit_err_from_latest_tag() -> Result<()> {
    run_test_with_context(|_| {
        // Arrange
        git_init_and_set_current_path("commit_err_from_tag")?;
        create_empty_config()?;
        git_commit("this one should not be picked")?;
        git_tag("0.1.0")?;
        git_commit("Oh no!")?;

        // Act
        let cocogitto = CocoGitto::get()?;

        // Assert
        assert!(cocogitto.check(true).is_err());
        Ok(())
    })
}
