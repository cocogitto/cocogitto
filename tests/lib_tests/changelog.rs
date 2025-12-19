use anyhow::Result;
use assert_cmd::Command;
use cmd_lib::run_cmd;
use sealed_test::prelude::*;
use speculoos::prelude::*;

use cocogitto::CocoGitto;

use crate::helpers::{git_commit, git_init, git_log_head_sha, git_tag};

#[sealed_test]
fn getting_changelog_from_tags_should_produce_the_same_range_either_from_tags_or_from_commits(
) -> Result<()> {
    // Arrange
    git_init()?;

    git_commit("feat: feature 1")?;
    let sha_0_1 = git_commit("feat: feature 2")?;
    git_tag("0.1.0")?;
    git_commit("feat: feature 3")?;
    git_commit("feat: feature 4")?;

    Command::new(assert_cmd::cargo_bin!("cog"))
        .arg("bump")
        .arg("--auto")
        .assert()
        .success();

    let head = git_log_head_sha()?;

    run_cmd!(
        git log --graph --abbrev-commit;
    )
    .unwrap();

    // Act
    let cocogitto = CocoGitto::get()?;
    let changelog_from_commit_range =
        cocogitto.get_changelog(&format!("{sha_0_1}..{head}"), false)?;
    let changelog_tag_range = cocogitto.get_changelog("0.1.0..0.2.0", false)?;
    let at_tag = cocogitto.get_changelog("..0.2.0", false)?;

    let commit_range_oids: Vec<String> = changelog_from_commit_range
        .commits
        .into_iter()
        .map(|commit| commit.commit.conventional.summary)
        .collect();

    let tag_range_oids: Vec<String> = changelog_tag_range
        .commits
        .into_iter()
        .map(|commit| commit.commit.conventional.summary)
        .collect();

    let at_tag_oids: Vec<String> = at_tag
        .commits
        .into_iter()
        .map(|commit| commit.commit.conventional.summary)
        .collect();

    // Assert
    asserting!("Changelog commits generated from a commit range should be equivalent to when generated from the same tag range")
        .that(&commit_range_oids)
        .is_equal_to(&tag_range_oids);

    asserting!("Changelog commits generated from a commit range should be equivalent to when generated from the same tag")
        .that(&commit_range_oids)
        .is_equal_to(&at_tag_oids);

    Ok(())
}

#[sealed_test]
fn from_commit_should_be_drained() -> Result<()> {
    // Arrange
    git_init()?;

    git_commit("feat: feature 1")?;
    git_commit("feat: feature 2")?;
    git_tag("0.1.0")?;
    git_commit("feat: feature 3")?;
    let unttaged_sha = git_commit("feat: feature 4")?;

    Command::new(assert_cmd::cargo_bin!("cog"))
        .arg("bump")
        .arg("--auto")
        .assert()
        .success();

    let head = git_log_head_sha()?;

    // Act
    let cocogitto = CocoGitto::get()?;
    let changelog_from_commit_range =
        cocogitto.get_changelog(&format!("{unttaged_sha}..{head}"), true)?;

    let commit_range_oids: Vec<String> = changelog_from_commit_range
        .commits
        .into_iter()
        .map(|commit| commit.commit.oid)
        .collect();

    // Assert
    asserting!("Changelog commits generated from a commit range should be equivalent to when generated from the same tag range")
        .that(&commit_range_oids)
        .is_equal_to(vec![head]);

    Ok(())
}

#[sealed_test]
fn changelog_date_should_come_from_commit_date_not_current_time() -> Result<()> {
    // Arrange
    git_init()?;

    run_cmd!(
        GIT_AUTHOR_DATE="2023-01-15 10:00:00" GIT_COMMITTER_DATE="2023-01-15 10:00:00" git commit --allow-empty -q -m "feat: initial feature";
    )?;

    git_tag("0.1.0")?;

    // Create another commit with current date
    git_commit("feat: another feature")?;
    git_tag("0.2.0")?;

    // Act
    let cmd = Command::new(assert_cmd::cargo_bin!("cog"))
        .arg("changelog")
        .arg("0.1.0..0.2.0")
        .arg("-t")
        .arg("default")
        .assert()
        .success();

    // Assert
    let changelog_output = cmd.get_output();
    let changelog_text = String::from_utf8_lossy(&changelog_output.stdout);

    asserting!("Changelog should contain the commit date from the v0.2.0 tag")
        .that(&changelog_text)
        .contains("## 0.2.0 - 2025-12-19");

    Ok(())
}
