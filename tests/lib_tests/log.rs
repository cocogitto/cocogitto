use cocogitto::log::filter::{CommitFilter, CommitFilters};
use cocogitto::CocoGitto;

use crate::helpers::*;

use anyhow::Result;
use cmd_lib::run_cmd;
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

#[sealed_test]
fn get_log_on_master_only() -> Result<()> {
    // Arrange
    git_init()?;
    let settings = r#"only_first_parent = true"#;
    run_cmd!(echo $settings > cog.toml;)?;
    git_commit("chore: initial commit")?;

    run_cmd!(git checkout -b feature)?;
    git_commit("feat: a commit on feature branch")?;
    git_commit("fix: a commit on feature branch")?;
    run_cmd!(
        git checkout master;
        git merge --no-ff -m "feat: feature" feature;
    )?;

    let filters = CommitFilters(vec![CommitFilter::NoError]);
    let cocogitto = CocoGitto::get()?;

    // Act
    let logs = cocogitto.get_log(filters)?;

    // Assert we don't see feature branch commit
    assert_that!(logs).does_not_contain("a commit on feature branch");

    Ok(())
}
