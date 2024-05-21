use anyhow::Result;
use cocogitto::CocoGitto;
use cocogitto_test_helpers::*;
use sealed_test::prelude::*;
use speculoos::prelude::*;

#[sealed_test]
fn open_repo_ok() -> Result<()> {
    // Arrange
    git_init_and_set_current_path("open_repo_ok")?;
    create_empty_config()?;

    // Act
    let cocogitto = CocoGitto::get();

    // Assert
    assert_that!(cocogitto).is_ok();
    Ok(())
}

#[sealed_test]
fn open_repo_err() -> Result<()> {
    // Arrange
    create_empty_config()?;

    // Act
    let cocogitto = CocoGitto::get();

    // Assert
    assert_that!(cocogitto).is_err();
    Ok(())
}
