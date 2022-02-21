use anyhow::Result;

use cmd_lib::run_cmd;
use cocogitto::{conventional::version::VersionIncrement, CocoGitto};
use indoc::indoc;
use sealed_test::prelude::*;
use speculoos::prelude::*;

use crate::helpers::*;

#[sealed_test]
fn bump_ok() -> Result<()> {
    // Arrange
    git_init()?;
    git_commit("chore: first commit")?;
    git_commit("feat: add a feature commit")?;
    git_tag("1.0.0")?;
    git_commit("feat: add another feature commit")?;

    let mut cocogitto = CocoGitto::get()?;

    // Act
    let result = cocogitto.create_version(VersionIncrement::Auto, None, None);

    // Assert
    assert_that!(result).is_ok();
    assert_latest_tag("1.1.0")?;
    Ok(())
}

#[sealed_test]
fn should_fallback_to_0_0_0_when_there_is_no_tag() -> Result<()> {
    // Arrange
    git_init()?;
    git_commit("chore: first commit")?;
    git_commit("feat: add a feature commit")?;

    let mut cocogitto = CocoGitto::get()?;

    // Act
    let result = cocogitto.create_version(VersionIncrement::Auto, None, None);

    // Assert
    assert_that!(result).is_ok();
    assert_latest_tag("0.1.0")?;
    Ok(())
}

#[sealed_test]
fn should_fail_when_latest_tag_is_not_semver_compliant() -> Result<()> {
    // Arrange
    git_init()?;
    git_commit("chore: first commit")?;
    git_commit("feat: add a feature commit")?;
    git_tag("toto")?;
    git_commit("feat: add another feature commit")?;

    let mut cocogitto = CocoGitto::get()?;

    // Act
    let result = cocogitto.create_version(VersionIncrement::Auto, None, None);
    let error = result.unwrap_err().to_string();
    let error = error.as_str();

    // Assert
    assert_that!(error).is_equal_to(indoc!(
        "
        tag `toto` is not SemVer compliant
        \tcause: unexpected character 't' while parsing major version number
        "
    ));
    Ok(())
}

#[sealed_test]
fn bump_with_whitelisted_branch_ok() -> Result<()> {
    // Arrange
    let settings = r#"branch_whitelist = [ "master" ]"#;

    git_init()?;
    run_cmd!(
        echo $settings > cog.toml;
        git add .;
    )?;

    git_commit("chore: first commit")?;
    git_commit("feat: add a feature commit")?;

    let mut cocogitto = CocoGitto::get()?;

    // Act
    let result = cocogitto.create_version(VersionIncrement::Auto, None, None);

    // Assert
    assert_that!(result).is_ok();

    Ok(())
}

#[sealed_test]
fn bump_with_whitelisted_branch_fails() -> Result<()> {
    // Arrange
    let settings = r#"branch_whitelist = [ "main" ]"#;

    git_init()?;
    run_cmd!(
        echo $settings > cog.toml;
        git add .;
    )?;

    git_commit("chore: first commit")?;
    git_commit("feat: add a feature commit")?;

    let mut cocogitto = CocoGitto::get()?;

    // Act
    let result = cocogitto.create_version(VersionIncrement::Auto, None, None);

    // Assert
    assert_that!(result.unwrap_err().to_string()).is_equal_to(
        "No patterns matched in [\"main\"] for branch 'master', bump is not allowed".to_string(),
    );

    Ok(())
}

#[sealed_test]
fn bump_with_whitelisted_branch_pattern_ok() -> Result<()> {
    // Arrange
    let settings = r#"branch_whitelist = [ "main", "release/**" ]"#;

    git_init()?;
    run_cmd!(
        echo $settings > cog.toml;
        git add .;
    )?;

    git_commit("chore: first commit")?;
    git_commit("feat: add a feature commit")?;

    run_cmd!(git checkout -b release/1.0.0;)?;

    let mut cocogitto = CocoGitto::get()?;

    // Act
    let result = cocogitto.create_version(VersionIncrement::Auto, None, None);

    // Assert
    assert_that!(result).is_ok();

    Ok(())
}

#[sealed_test]
fn bump_with_whitelisted_branch_pattern_err() -> Result<()> {
    // Arrange
    let settings = r#"branch_whitelist = [ "release/**" ]"#;

    git_init()?;
    run_cmd!(
        echo $settings > cog.toml;
        git add .;
    )?;

    git_commit("chore: first commit")?;
    git_commit("feat: add a feature commit")?;

    let mut cocogitto = CocoGitto::get()?;

    // Act
    let result = cocogitto.create_version(VersionIncrement::Auto, None, None);

    // Assert
    assert_that!(result).is_err();

    Ok(())
}
