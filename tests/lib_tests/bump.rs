use std::collections::HashMap;
use std::path::Path;
use std::path::PathBuf;

use anyhow::Result;
use cmd_lib::run_cmd;
use sealed_test::prelude::*;
use speculoos::prelude::*;

use cocogitto::settings::{MonoRepoPackage, Settings};
use cocogitto::{conventional::version::IncrementCommand, CocoGitto};

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
    let result = cocogitto.create_version(
        IncrementCommand::Auto,
        None,
        None,
        None,
        false,
        false,
        None,
        false,
    );

    // Assert
    assert_that!(result).is_ok();
    assert_latest_tag("1.1.0")?;
    Ok(())
}

#[sealed_test]
fn annotated_bump_ok() -> Result<()> {
    // Arrange
    git_init()?;
    git_commit("chore: first commit")?;
    git_commit("feat: add a feature commit")?;
    git_tag("1.0.0")?;
    git_commit("feat: add another feature commit")?;

    let mut cocogitto = CocoGitto::get()?;

    // Act
    let result = cocogitto.create_version(
        IncrementCommand::Auto,
        None,
        None,
        Some(String::from("Release version {{version}}")),
        false,
        false,
        None,
        false,
    );

    // Assert
    assert_that!(result).is_ok();
    assert_latest_tag("1.1.0")?;
    assert_tag_is_annotated("1.1.0")?;
    Ok(())
}

#[sealed_test]
fn monorepo_bump_ok() -> Result<()> {
    // Arrange
    let mut settings = Settings {
        ..Default::default()
    };

    init_monorepo(&mut settings)?;

    let mut cocogitto = CocoGitto::get()?;

    // Act
    let result = cocogitto.create_monorepo_version(
        IncrementCommand::Auto,
        None,
        None,
        None,
        false,
        false,
        None,
        false,
    );

    // Assert
    assert_that!(result).is_ok();
    assert_tag_exists("0.1.0")?;
    assert_tag_exists("one-0.1.0")?;
    Ok(())
}

#[sealed_test]
fn monorepo_bump_manual_ok() -> Result<()> {
    // Arrange
    let mut settings = Settings {
        ..Default::default()
    };

    init_monorepo(&mut settings)?;
    run_cmd!(
        git tag "one-0.1.0";
    )?;

    let mut cocogitto = CocoGitto::get()?;

    // Act
    let result = cocogitto.create_monorepo_version(
        IncrementCommand::Major,
        None,
        None,
        None,
        false,
        false,
        None,
        false,
    );

    // Assert
    assert_that!(result).is_ok();
    assert_tag_exists("1.0.0")?;
    Ok(())
}

#[sealed_test]
fn monorepo_bump_manual_disable_changelog_ok() -> Result<()> {
    // Arrange
    let mut settings = Settings {
        disable_changelog: true,
        ..Default::default()
    };

    init_monorepo(&mut settings)?;
    run_cmd!(
        git tag "one-0.1.0";
    )?;

    let mut cocogitto = CocoGitto::get()?;

    // Act
    let result = cocogitto.create_monorepo_version(
        IncrementCommand::Major,
        None,
        None,
        None,
        false,
        false,
        None,
        false,
    );

    // Assert
    assert_that!(result).is_ok();
    assert_tag_exists("1.0.0")?;
    assert_that!(Path::new("CHANGELOG.md")).does_not_exist();
    Ok(())
}

#[sealed_test]
fn monorepo_with_tag_prefix_bump_ok() -> Result<()> {
    // Arrange
    let mut settings = Settings {
        tag_prefix: Some("v".to_string()),
        ..Default::default()
    };

    init_monorepo(&mut settings)?;

    let mut cocogitto = CocoGitto::get()?;

    // Act
    let result = cocogitto.create_monorepo_version(
        IncrementCommand::Auto,
        None,
        None,
        None,
        false,
        false,
        None,
        false,
    );

    // Assert
    assert_that!(result).is_ok();
    assert_tag_exists("v0.1.0")?;
    assert_tag_exists("one-v0.1.0")?;
    Ok(())
}

#[sealed_test]
fn package_bump_ok() -> Result<()> {
    // Arrange
    let mut settings = Settings {
        ..Default::default()
    };

    init_monorepo(&mut settings)?;
    let package = settings.packages.get("one").unwrap();
    let mut cocogitto = CocoGitto::get()?;

    // Act
    let result = cocogitto.create_package_version(
        ("one", package),
        IncrementCommand::AutoPackage("one".to_string()),
        None,
        None,
        None,
        false,
        false,
        None,
        false,
    );

    // Assert
    assert_that!(result).is_ok();
    assert_tag_does_not_exist("0.1.0")?;
    assert_tag_exists("one-0.1.0")?;
    Ok(())
}

#[sealed_test]
fn consecutive_package_bump_ok() -> Result<()> {
    // Arrange
    let mut packages = HashMap::new();
    let jenkins = || MonoRepoPackage {
        path: PathBuf::from("jenkins"),
        public_api: false,
        changelog_path: Some("jenkins/CHANGELOG.md".to_owned()),
        ..Default::default()
    };

    packages.insert("jenkins".to_owned(), jenkins());

    let thumbor = || MonoRepoPackage {
        path: PathBuf::from("thumbor"),
        public_api: false,
        changelog_path: Some("thumbor/CHANGELOG.md".to_owned()),
        ..Default::default()
    };

    packages.insert("thumbor".to_owned(), thumbor());

    let settings = Settings {
        packages,
        ignore_merge_commits: true,
        ..Default::default()
    };

    let settings = toml::to_string(&settings)?;

    git_init()?;
    run_cmd!(
        echo Hello > README.md;
        git add .;
        git commit -m "first commit";
        mkdir jenkins;
        echo "some jenkins stuff" > jenkins/file;
        git add .;
        git commit -m "feat(jenkins): add jenkins stuffs";
        mkdir thumbor;
        echo "some thumbor stuff" > thumbor/file;
        git add .;
        git commit -m "feat(thumbor): add thumbor stuffs";
        echo $settings > cog.toml;
        git add .;
        git commit -m "chore: add cog.toml";
    )?;

    let mut cocogitto = CocoGitto::get()?;

    // Act
    cocogitto.create_package_version(
        ("thumbor", &thumbor()),
        IncrementCommand::AutoPackage("thumbor".to_owned()),
        None,
        None,
        None,
        false,
        false,
        None,
        false,
    )?;

    cocogitto.create_package_version(
        ("jenkins", &jenkins()),
        IncrementCommand::AutoPackage("jenkins".to_owned()),
        None,
        None,
        None,
        false,
        false,
        None,
        false,
    )?;

    run_cmd!(
        echo "fix jenkins bug" > jenkins/fix;
        git add .;
        git commit -m "fix(jenkins): bug fix on jenkins package";
    )?;

    cocogitto.create_package_version(
        ("jenkins", &jenkins()),
        IncrementCommand::AutoPackage("jenkins".to_owned()),
        None,
        None,
        None,
        false,
        false,
        None,
        false,
    )?;

    // Assert
    assert_tag_exists("jenkins-0.1.0")?;
    assert_tag_exists("thumbor-0.1.0")?;
    assert_tag_exists("jenkins-0.1.1")?;
    assert_tag_does_not_exist("jenkins-0.2.0")?;
    assert_tag_does_not_exist("0.1.0")?;
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
    let result = cocogitto.create_version(
        IncrementCommand::Auto,
        None,
        None,
        None,
        false,
        false,
        None,
        false,
    );

    // Assert
    assert_that!(result).is_ok();
    assert_latest_tag("0.1.0")?;
    Ok(())
}

#[sealed_test]
fn should_ignore_latest_prerelease_tag() -> Result<()> {
    // Arrange
    git_init()?;
    git_commit("chore: first commit")?;
    git_commit("feat: add a feature commit")?;

    let mut cocogitto = CocoGitto::get()?;
    cocogitto.create_version(
        IncrementCommand::Auto,
        Some("alpha1"),
        None,
        None,
        false,
        false,
        None,
        false,
    )?;

    git_commit("feat: more features")?;
    // Act
    let result = cocogitto.create_version(
        IncrementCommand::Auto,
        None,
        None,
        None,
        false,
        false,
        None,
        false,
    );

    // Assert
    assert_that!(result).is_ok();
    assert_tag_exists("0.1.0-alpha1")?;
    assert_latest_tag("0.1.0")?;

    Ok(())
}

#[sealed_test]
fn auto_bump_package_only_ok() -> Result<()> {
    // Arrange
    let mut packages = HashMap::new();
    let jenkins = || MonoRepoPackage {
        path: PathBuf::from("jenkins"),
        public_api: false,
        changelog_path: Some("jenkins/CHANGELOG.md".to_owned()),
        ..Default::default()
    };

    packages.insert("jenkins".to_owned(), jenkins());

    let thumbor = || MonoRepoPackage {
        path: PathBuf::from("thumbor"),
        public_api: false,
        changelog_path: Some("thumbor/CHANGELOG.md".to_owned()),
        ..Default::default()
    };

    packages.insert("thumbor".to_owned(), thumbor());

    let settings = Settings {
        packages,
        generate_mono_repository_global_tag: false,
        ..Default::default()
    };

    let settings = toml::to_string(&settings)?;

    git_init()?;
    run_cmd!(
        echo Hello > README.md;
        git add .;
        git commit -m "first commit";
        mkdir jenkins;
        echo "some jenkins stuff" > jenkins/file;
        git add .;
        git commit -m "feat(jenkins): add jenkins stuffs";
        mkdir thumbor;
        echo "some thumbor stuff" > thumbor/file;
        git add .;
        git commit -m "feat(thumbor): add thumbor stuffs";
        echo $settings > cog.toml;
        git add .;
        git commit -m "chore: add cog.toml";
    )?;

    let mut cocogitto = CocoGitto::get()?;

    // Act
    cocogitto.create_all_package_version_auto(None, None, false, false, None, false)?;

    assert_tag_exists("jenkins-0.1.0")?;
    assert_tag_exists("thumbor-0.1.0")?;
    assert_tag_does_not_exist("0.1.0")?;

    run_cmd!(
        echo "fix jenkins bug" > jenkins/fix;
        git add .;
        git commit -m "fix(jenkins): bug fix on jenkins package";
    )?;

    cocogitto.create_all_package_version_auto(None, None, false, false, None, false)?;

    // Assert
    assert_tag_exists("jenkins-0.1.1")?;
    Ok(())
}

// FIXME: Failing on non compliant tag should be configurable
//  until it's implemented we will ignore non compliant tags
// #[sealed_test]
// fn should_fail_when_latest_tag_is_not_semver_compliant() -> Result<()> {
//     // Arrange
//     git_init()?;
//     git_commit("chore: first commit")?;
//     git_commit("feat: add a feature commit")?;
//     git_tag("toto")?;
//     git_commit("feat: add another feature commit")?;
//
//     let mut cocogitto = CocoGitto::get()?;
//
//     // Act
//     let result = cocogitto.create_version(VersionIncrement::Auto, None, None, false);
//     let error = result.unwrap_err().to_string();
//     let error = error.as_str();
//
//     // Assert
//     assert_that!(error).is_equal_to(indoc!(
//         "
//         tag `toto` is not SemVer compliant
//         \tcause: unexpected character 't' while parsing major version number
//         "
//     ));
//     Ok(())
// }

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
    let result = cocogitto.create_version(
        IncrementCommand::Auto,
        None,
        None,
        None,
        false,
        false,
        None,
        false,
    );

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
    let result = cocogitto.create_version(
        IncrementCommand::Auto,
        None,
        None,
        None,
        false,
        false,
        None,
        false,
    );

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
    let result = cocogitto.create_version(
        IncrementCommand::Auto,
        None,
        None,
        None,
        false,
        false,
        None,
        false,
    );

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
    let result = cocogitto.create_version(
        IncrementCommand::Auto,
        None,
        None,
        None,
        false,
        false,
        None,
        false,
    );

    // Assert
    assert_that!(result).is_err();

    Ok(())
}

#[sealed_test]
fn bump_no_error_should_be_thrown_on_only_chore_docs_commit() -> Result<()> {
    // Arrange
    let mut packages = HashMap::new();
    let jenkins = || MonoRepoPackage {
        path: PathBuf::from("jenkins"),
        changelog_path: Some("jenkins/CHANGELOG.md".to_owned()),
        ..Default::default()
    };

    packages.insert("jenkins".to_owned(), jenkins());

    let thumbor = || MonoRepoPackage {
        path: PathBuf::from("thumbor"),
        changelog_path: Some("thumbor/CHANGELOG.md".to_owned()),
        ..Default::default()
    };

    packages.insert("thumbor".to_owned(), thumbor());

    let settings = Settings {
        packages,
        ignore_merge_commits: true,
        ..Default::default()
    };

    let settings = toml::to_string(&settings)?;

    git_init()?;
    run_cmd!(
        echo Hello > README.md;
        git add .;
        git commit -m "first commit";
        mkdir jenkins;
        echo "some jenkins stuff" > jenkins/file;
        git add .;
        git commit -m "feat(jenkins): add jenkins stuffs";
        mkdir thumbor;
        echo "some thumbor stuff" > thumbor/file;
        git add .;
        git commit -m "feat(thumbor): add thumbor stuffs";
        echo $settings > cog.toml;
        git add .;
        git commit -m "chore: add cog.toml";
    )?;

    let mut cocogitto = CocoGitto::get()?;

    // Act
    cocogitto.create_monorepo_version(
        IncrementCommand::Auto,
        None,
        None,
        None,
        false,
        false,
        None,
        false,
    )?;

    run_cmd!(
        echo "chore on jenkins" > jenkins/fix;
        git add .;
        git commit -m "chore(jenkins): jenkins chore";
        echo "docs on jenkins" > jenkins/fix;
        git add .;
        git commit -m "docs(jenkins): jenkins docs";
    )?;

    cocogitto.create_monorepo_version(
        IncrementCommand::Auto,
        None,
        None,
        None,
        false,
        false,
        None,
        false,
    )?;

    cocogitto.create_package_version(
        ("jenkins", &jenkins()),
        IncrementCommand::AutoPackage("jenkins".to_owned()),
        None,
        None,
        None,
        false,
        false,
        None,
        false,
    )?;

    run_cmd!(
        echo "more feat on thumbor" > thumbor/feat;
        git add .;
        git commit -m "feat(thumbor): more feat on thumbor";
    )?;

    cocogitto.create_monorepo_version(
        IncrementCommand::Auto,
        None,
        None,
        None,
        false,
        false,
        None,
        false,
    )?;

    // Assert
    assert_tag_exists("jenkins-0.1.0")?;
    assert_tag_exists("thumbor-0.1.0")?;
    assert_tag_exists("thumbor-0.2.0")?;
    assert_tag_exists("0.1.0")?;
    assert_tag_exists("0.2.0")?;

    assert_tag_does_not_exist("jenkins-0.1.1")?;
    assert_tag_does_not_exist("jenkins-0.2.0")?;
    assert_tag_does_not_exist("jenkins-1.0.0")?;
    Ok(())
}

#[sealed_test]
fn error_on_no_conventionnal_commits_found_for_monorepo() -> Result<()> {
    let settings = Settings {
        ..Default::default()
    };

    let settings = toml::to_string(&settings)?;

    git_init()?;

    run_cmd!(
        echo Hello > README.md;
        git add .;
        git commit -m "chore: first commit";

        echo $settings > cog.toml;
        git add .;

        echo "first feature" > file;
        git add .;
        git commit -m "feat: feature commit";
    )?;

    let mut cocogitto = CocoGitto::get()?;

    // Act
    let first_result = cocogitto.create_version(
        IncrementCommand::Auto,
        None,
        None,
        None,
        false,
        false,
        None,
        false,
    );

    // Assert
    assert_that!(first_result).is_ok();

    run_cmd!(
        echo "second feature" > file;
        git add .;
    )?;

    git_commit("second unconventional feature commit")?;

    // Act
    let second_result = cocogitto.create_version(
        IncrementCommand::Auto,
        None,
        None,
        None,
        false,
        false,
        None,
        false,
    );

    // Assert
    assert_that!(second_result).is_err();

    Ok(())
}

#[sealed_test]
fn error_on_no_conventionnal_commits_found_for_package() -> Result<()> {
    // Arrange
    let mut packages = HashMap::new();
    let jenkins = || MonoRepoPackage {
        path: PathBuf::from("jenkins"),
        changelog_path: Some("jenkins/CHANGELOG.md".to_owned()),
        ..Default::default()
    };

    packages.insert("jenkins".to_owned(), jenkins());

    let settings = Settings {
        packages,
        ignore_merge_commits: true,
        ..Default::default()
    };

    let settings = toml::to_string(&settings)?;

    git_init()?;
    run_cmd!(
        echo Hello > README.md;
        git add .;
        git commit -m "first commit";

        echo $settings > cog.toml;
        git add .;
        git commit -m "chore: cog config";

        mkdir jenkins;
        echo "some jenkins stuff" > jenkins/file;
        git add .;
        git commit -m "feat(jenkins): some jenkins stuff";
    )?;

    let mut cocogitto = CocoGitto::get()?;

    let first_result = cocogitto.create_package_version(
        ("jenkins", &jenkins()),
        IncrementCommand::AutoPackage("jenkins".to_owned()),
        None,
        None,
        None,
        false,
        false,
        None,
        false,
    );

    assert_that!(first_result).is_ok();

    run_cmd!(
        echo "some other jenkins stuff" > jenkins/file;
        git add .;
        git commit -m "some other jenkins stuff";
    )?;

    let second_result = cocogitto.create_package_version(
        ("jenkins", &jenkins()),
        IncrementCommand::AutoPackage("jenkins".to_owned()),
        None,
        None,
        None,
        false,
        false,
        None,
        false,
    );

    assert_that!(second_result).is_err();

    Ok(())
}

#[sealed_test]
fn bump_with_unconventionnal_and_conventional_commits_found_for_packages() -> Result<()> {
    // Arrange
    let mut packages = HashMap::new();
    let jenkins = || MonoRepoPackage {
        path: PathBuf::from("jenkins"),
        changelog_path: Some("jenkins/CHANGELOG.md".to_owned()),
        ..Default::default()
    };

    packages.insert("jenkins".to_owned(), jenkins());

    let thumbor = || MonoRepoPackage {
        path: PathBuf::from("thumbor"),
        changelog_path: Some("thumbor/CHANGELOG.md".to_owned()),
        ..Default::default()
    };

    packages.insert("thumbor".to_owned(), thumbor());

    let settings = Settings {
        packages,
        ignore_merge_commits: true,
        ..Default::default()
    };

    let settings = toml::to_string(&settings)?;

    git_init()?;
    run_cmd!(
        echo Hello > README.md;
        git add .;
        git commit -m "first commit";
        mkdir jenkins;
        echo "unconventional jenkins stuff" > jenkins/file;
        git add .;
        git commit -m "unconventional jenkins stuff";
        mkdir thumbor;
        echo "conventional thumbor stuff" > thumbor/file;
        git add .;
        git commit -m "feat(thumbor): conventional thumbor stuff";
        echo $settings > cog.toml;
        git add .;
        git commit -m "chore: add cog.toml";
    )?;

    let mut cocogitto = CocoGitto::get()?;

    // Act
    let result = cocogitto.create_monorepo_version(
        IncrementCommand::Auto,
        None,
        None,
        None,
        false,
        false,
        None,
        false,
    );

    // Assert
    assert_that!(result).is_ok();
    assert_tag_exists("thumbor-0.1.0")?;
    assert_tag_exists("0.1.0")?;

    assert_tag_does_not_exist("jenkins-0.1.0")?;

    Ok(())
}
