use anyhow::Result;
use cocogitto::CocoGitto;

use crate::helpers::*;

use cocogitto::log::filter::CommitFilter;
use cocogitto::log::filter::CommitFilters;
use speculoos::prelude::*;

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

#[test]
fn long_commit_summary_does_not_panic() -> Result<()> {
    run_test_with_context(|_| {
        git_init()?;
        let message =
            "aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa…"
                .to_string();

        let cocogitto = CocoGitto::get()?;
        std::fs::write("file", "Hello")?;
        git_add()?;
        cocogitto.conventional_commit("feat", None, message, None, None, false)?;

        let result = cocogitto.check(false);

        assert!(result.is_ok());
        Ok(())
    })
}

#[test]
fn get_unfiltered_logs() -> Result<()> {
    run_test_with_context(|_| {
        // Arrange
        git_init()?;
        git_commit("feat: a commit")?;
        git_commit("test: do you test your code ?")?;
        git_commit("I am afraid I can't do that Dave")?;
        let filters = CommitFilters(Vec::with_capacity(0));
        let cocogitto = CocoGitto::get()?;

        // Act
        let logs = cocogitto.get_log(filters)?;

        // Assert
        assert_that(&logs).contains("Commit message : 'I am afraid I can't do that Dave'");
        assert_that(&logs).contains("Cause : Missing commit type separator `:`");

        Ok(())
    })
}

#[test]
fn get_log_with_no_errors() -> Result<()> {
    run_test_with_context(|_| {
        // Arrange
        git_init()?;
        git_commit("feat: a commit")?;
        git_commit("test: do you test your code ?")?;
        git_commit("I am afraid I can't do that Dave")?;

        let filters = CommitFilters(vec![CommitFilter::NoError]);
        let cocogitto = CocoGitto::get()?;

        // Act
        let logs = cocogitto.get_log(filters)?;

        // Assert
        assert_that(&logs).does_not_contain("Errored commit : ");
        assert_that(&logs).does_not_contain("Commit message : 'I am afraid I can't do that Dave'");
        assert_that(&logs).does_not_contain("Missing commit type separator `:`");

        Ok(())
    })
}
