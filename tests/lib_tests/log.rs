use cocogitto::log::filter::{CommitFilter, CommitFilters};
use cocogitto::CocoGitto;

use crate::helpers::*;

use anyhow::Result;
use sealed_test::prelude::*;
use speculoos::prelude::*;

#[sealed_test]
fn get_unfiltered_logs() -> Result<()> {
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
    assert_that!(logs).contains("I am afraid I can't do that Dave");
    assert_that!(logs).contains("Missing commit type separator `:`");

    Ok(())
}

#[sealed_test]
fn get_log_with_no_errors() -> Result<()> {
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
    assert_that!(logs).does_not_contain("Errored commit:");
    assert_that!(logs).does_not_contain("Commit message: 'I am afraid I can't do that Dave'");
    assert_that!(logs).does_not_contain("Missing commit type separator `:`");

    Ok(())
}
