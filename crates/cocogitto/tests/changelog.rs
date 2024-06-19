use anyhow::Result;
use assert_cmd::Command;
use cmd_lib::run_cmd;
use sealed_test::prelude::*;
use speculoos::prelude::*;

use cocogitto_changelog::get_changelog;
use cocogitto_config::Settings;

use cocogitto_test_helpers::*;

#[sealed_test]
fn getting_changelog_from_tags_should_produce_the_same_range_either_from_tags_or_from_commits(
) -> Result<()> {
    // Arrange
    let repository = git_init_no_gpg()?;

    git_commit("feat: feature 1")?;
    let sha_0_1 = git_commit("feat: feature 2")?;
    git_tag("0.1.0")?;
    git_commit("feat: feature 3")?;
    git_commit("feat: feature 4")?;

    Command::cargo_bin("cog")?
        .arg("bump")
        .arg("--auto")
        .assert()
        .success();

    let head = git_log_head_sha()?;

    run_cmd!(
        git log --graph --abbrev-commit;
    )
    .unwrap();

    let settings = Settings::default();
    let allowed_commits = &settings.allowed_commit_types();
    let omitted_commits = &settings.commit_omitted_from_changelog();
    let changelog_titles = &settings.changelog_titles();
    let usernames = &settings.commit_usernames();

    // Act
    let changelog_from_commit_range = get_changelog(
        &repository,
        &format!("{sha_0_1}..{head}"),
        allowed_commits,
        omitted_commits,
        changelog_titles,
        usernames,
    )?;
    let changelog_tag_range = get_changelog(
        &repository,
        "0.1.0..0.2.0",
        allowed_commits,
        omitted_commits,
        changelog_titles,
        usernames,
    )?;
    let at_tag = get_changelog(
        &repository,
        "..0.2.0",
        allowed_commits,
        omitted_commits,
        changelog_titles,
        usernames,
    )?;

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
    let repository = git_init_no_gpg()?;

    git_commit("feat: feature 1")?;
    git_commit("feat: feature 2")?;
    git_tag("0.1.0")?;
    git_commit("feat: feature 3")?;
    let unttaged_sha = git_commit("feat: feature 4")?;

    Command::cargo_bin("cog")?
        .arg("bump")
        .arg("--auto")
        .assert()
        .success();

    let head = git_log_head_sha()?;

    let settings = Settings::default();
    let allowed_commits = &settings.allowed_commit_types();
    let omitted_commits = &settings.commit_omitted_from_changelog();
    let changelog_titles = &settings.changelog_titles();
    let usernames = &settings.commit_usernames();

    // Act
    let changelog_from_commit_range = get_changelog(
        &repository,
        &format!("{unttaged_sha}..{head}"),
        allowed_commits,
        omitted_commits,
        changelog_titles,
        usernames,
    )?;

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
