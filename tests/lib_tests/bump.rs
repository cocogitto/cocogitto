use anyhow::Result;

use cmd_lib::run_cmd;
use cocogitto::{conventional::version::VersionIncrement, CocoGitto};
use sealed_test::prelude::*;
use speculoos::prelude::*;

use crate::helpers::*;

#[sealed_test]
fn bump_no_whitelisted_branch_ok() -> Result<()> {
    // Arrange
    git_init().expect("Could not init repository");
    git_commit("chore: first commit").expect("Could not create commit");
    git_commit("feat: add a feature commit").expect("Could not create commit");

    let mut cocogitto = CocoGitto::get()?;

    // Act
    let result = cocogitto.create_version(VersionIncrement::Auto, None, None);

    // Assert
    assert_that!(result).is_ok();

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
    )
    .unwrap();

    git_commit("chore: first commit").expect("Could not create commit");
    git_commit("feat: add a feature commit").expect("Could not create commit");

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
    )
    .unwrap();
    git_commit("chore: first commit").expect("Could not create commit");
    git_commit("feat: add a feature commit").expect("Could not create commit");

    let mut cocogitto = CocoGitto::get()?;

    // Act
    let result = cocogitto.create_version(VersionIncrement::Auto, None, None);

    // Assert
    assert_that!(result.unwrap_err().to_string())
        .is_equal_to("Version bump not allowed on branch master".to_string());

    Ok(())
}
