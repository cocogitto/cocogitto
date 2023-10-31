use std::process::Command;

use crate::helpers::*;

use anyhow::Result;
use assert_cmd::prelude::*;
use cmd_lib::run_cmd;
use cocogitto::settings::Settings;
use indoc::indoc;
use sealed_test::prelude::*;
use speculoos::prelude::*;
use std::path::Path;

#[sealed_test]
fn auto_bump_from_start_ok() -> Result<()> {
    git_init()?;
    git_commit("chore: init")?;
    git_commit("feat(taef): feature")?;
    git_commit("fix: bug fix")?;

    Command::cargo_bin("cog")?
        .arg("bump")
        .arg("--auto")
        .assert()
        .success();

    assert_that!(Path::new("CHANGELOG.md")).exists();
    assert_tag_exists("0.1.0")?;
    Ok(())
}

#[sealed_test]
fn auto_bump_minor_from_latest_tag() -> Result<()> {
    git_init()?;
    git_commit("chore: init")?;
    git_commit("feat(taef): feature")?;
    git_commit("fix: bug fix")?;
    git_tag("1.0.0")?;
    git_commit("feat(taef): feature")?;
    git_commit("feat: feature 1")?;
    git_commit("feat: feature 2")?;

    Command::cargo_bin("cog")?
        .arg("bump")
        .arg("--auto")
        .assert()
        .success();

    assert_that!(Path::new("CHANGELOG.md")).exists();
    assert_tag_exists("1.1.0")?;
    Ok(())
}

#[sealed_test]
fn auto_bump_dry_run_from_latest_tag() -> Result<()> {
    git_init()?;
    git_commit("chore: init")?;
    git_commit("feat(taef): feature")?;
    git_commit("fix: bug fix")?;
    git_tag("1.0.0")?;
    git_commit("feat(taef): feature")?;
    git_commit("feat: feature 1")?;
    git_commit("feat: feature 2")?;

    Command::cargo_bin("cog")?
        .arg("bump")
        .arg("--auto")
        .arg("--dry-run")
        .assert()
        .success()
        .stdout("1.1.0\n");

    assert_that!(Path::new("CHANGELOG.md")).does_not_exist();
    assert_tag_does_not_exist("1.1.0")?;
    Ok(())
}

#[sealed_test]
fn auto_bump_major_from_latest_tag() -> Result<()> {
    git_init()?;
    git_commit("chore: init")?;
    git_commit("feat(taef): feature")?;
    git_commit("fix: bug fix")?;
    git_tag("1.0.0")?;
    git_commit("feat!(taef): feature")?;
    git_commit("feat!: feature 1")?;
    git_commit("feat: feature 2")?;

    Command::cargo_bin("cog")?
        .arg("bump")
        .arg("--auto")
        .assert()
        .success();

    assert_that!(Path::new("CHANGELOG.md")).exists();
    assert_tag_exists("2.0.0")?;
    Ok(())
}

#[sealed_test]
fn auto_bump_with_prefix() -> Result<()> {
    git_init()?;
    git_add("tag_prefix = \"v\"", "cog.toml")?;
    git_commit("chore: init")?;
    git_commit("feat(taef): feature")?;
    git_commit("fix: bug fix")?;
    git_tag("v1.0.0")?;
    git_commit("feat(taef)!: feature")?;
    git_commit("feat!: feature 1")?;
    git_commit("feat: feature 2")?;

    Command::cargo_bin("cog")?
        .arg("bump")
        .arg("--auto")
        .assert()
        .success();

    assert_that!(Path::new("CHANGELOG.md")).exists();
    assert_tag_exists("v2.0.0")?;
    Ok(())
}

#[sealed_test]
fn disable_changelog_disables_changelog_generation() -> Result<()> {
    git_init()?;
    git_add("disable_changelog = true", "cog.toml")?;
    git_commit("chore: first commit")?;
    git_commit("feat: add a feature commit")?;
    git_tag("1.0.0")?;
    git_commit("feat: add another feature commit")?;
    Command::cargo_bin("cog")?
        .arg("bump")
        .arg("--auto")
        .assert()
        .success();

    assert_that!(Path::new("CHANGELOG.md")).does_not_exist();
    assert_tag_exists("1.0.0")?;
    assert_tag_exists("1.1.0")?;
    Ok(())
}

#[sealed_test]
fn disable_changelog_disables_changelog_generation_for_monorepos() -> Result<()> {
    let mut settings = Settings {
        disable_changelog: true,
        ..Default::default()
    };
    init_monorepo(&mut settings)?;
    Command::cargo_bin("cog")?
        .arg("bump")
        .arg("--auto")
        .assert()
        .success();

    assert_that!(Path::new("CHANGELOG.md")).does_not_exist();
    assert_tag_exists("0.1.0")?;
    assert_tag_exists("one-0.1.0")?;
    Ok(())
}

#[sealed_test]
fn disable_changelog_disables_changelog_generation_for_packages() -> Result<()> {
    let mut settings = Settings {
        disable_changelog: true,
        ..Default::default()
    };
    init_monorepo(&mut settings)?;

    Command::cargo_bin("cog")?
        .arg("bump")
        .arg("--auto")
        .arg("--package")
        .arg("one")
        .assert()
        .success();

    assert_that!(Path::new("CHANGELOG.md")).does_not_exist();
    assert_tag_exists("one-0.1.0")?;
    Ok(())
}

#[sealed_test]
fn auto_bump_patch_from_latest_tag() -> Result<()> {
    git_init()?;
    git_commit("chore: init")?;
    git_commit("feat(taef): feature")?;
    git_commit("fix: bug fix")?;
    git_tag("1.0.0")?;
    git_commit("fix(the_fix): the_fix")?;
    git_commit("fix: fix 1")?;
    git_commit("fix: fix 2")?;

    Command::cargo_bin("cog")?
        .arg("bump")
        .arg("--auto")
        .assert()
        .success();

    assert_that!(Path::new("CHANGELOG.md")).exists();
    assert_tag_exists("1.0.1")?;
    Ok(())
}

#[sealed_test]
fn auto_bump_respect_semver_sorting() -> Result<()> {
    git_init()?;
    git_commit("chore: init")?;
    git_commit("feat(taef): feature")?;
    git_commit("fix: bug fix")?;
    git_tag("0.9.1")?;
    git_commit("feat(the_fix): feature")?;
    git_tag("0.10.0")?;
    git_commit("fix: fix 1")?;
    git_commit("fix: fix 2")?;

    Command::cargo_bin("cog")?
        .arg("bump")
        .arg("--auto")
        .assert()
        .success();

    assert_that!(Path::new("CHANGELOG.md")).exists();
    assert_tag_exists("0.10.1")?;
    Ok(())
}

#[sealed_test]
fn minor_bump() -> Result<()> {
    git_init()?;
    git_commit("chore: init")?;
    git_tag("1.0.0")?;
    git_commit("feat: feature")?;

    Command::cargo_bin("cog")?
        .arg("bump")
        .arg("--minor")
        .assert()
        .success();

    assert_that!(Path::new("CHANGELOG.md")).exists();
    assert_tag_exists("1.1.0")?;
    Ok(())
}

#[sealed_test]
fn major_bump() -> Result<()> {
    git_init()?;
    git_commit("chore: init")?;
    git_tag("1.0.0")?;
    git_commit("feat: feature")?;

    Command::cargo_bin("cog")?
        .arg("bump")
        .arg("--major")
        .assert()
        .success();

    assert_that!(Path::new("CHANGELOG.md")).exists();
    assert_tag_exists("2.0.0")?;
    Ok(())
}

#[sealed_test]
fn patch_bump() -> Result<()> {
    git_init()?;
    git_commit("chore: init")?;
    git_tag("1.0.0")?;
    git_commit("feat: feature")?;

    Command::cargo_bin("cog")?
        .arg("bump")
        .arg("--patch")
        .assert()
        .success();
    assert_that!(Path::new("CHANGELOG.md")).exists();
    assert_tag_exists("1.0.1")?;
    Ok(())
}

#[sealed_test]
fn pre_release_bump() -> Result<()> {
    git_init()?;
    git_commit("chore: init")?;
    git_tag("1.0.0")?;
    git_commit("feat: feature")?;

    Command::cargo_bin("cog")?
        .arg("bump")
        .arg("--major")
        .arg("--pre")
        .arg("alpha")
        .assert()
        .success();

    assert_that!(Path::new("CHANGELOG.md")).exists();
    assert_tag_exists("2.0.0-alpha")?;
    Ok(())
}

#[sealed_test]
#[cfg(target_os = "linux")]
fn bump_with_hook() -> Result<()> {
    // Arrange
    git_init()?;
    git_add(r#"pre_bump_hooks = ["touch {{version}}"]"#, "cog.toml")?;
    git_commit("chore: init")?;
    git_tag("1.0.0")?;
    git_commit("feat: feature")?;

    // Act
    Command::cargo_bin("cog")?
        .arg("bump")
        .arg("--major")
        // Assert
        .assert()
        .success();

    assert_that!(Path::new("2.0.0")).exists();
    assert_tag_exists("2.0.0")?;
    Ok(())
}

#[sealed_test]
#[cfg(target_os = "linux")]
fn bump_with_profile_hook() -> Result<()> {
    // Arrange
    git_init()?;

    let config = indoc! {
        "[bump_profiles.custom]
            pre_bump_hooks = [ \"echo current {{latest}}\" ]
            post_bump_hooks = [ \"echo next {{version}}\" ]
        "
    };

    git_add(config, "cog.toml")?;

    git_commit("chore: init")?;
    git_tag("1.0.0")?;
    git_commit("feat: feature")?;

    // Act
    Command::cargo_bin("cog")?
        .arg("bump")
        .arg("--hook-profile")
        .arg("custom")
        .arg("--patch")
        .unwrap()
        // Assert
        .assert()
        .success();

    assert_tag_exists("1.0.1")?;
    Ok(())
}

#[sealed_test]
fn monorepo_dry_run() -> Result<()> {
    init_monorepo(&mut Settings::default())?;

    Command::cargo_bin("cog")?
        .arg("bump")
        .arg("--auto")
        .arg("--dry-run")
        .assert()
        .success()
        .stdout(indoc!(
            "one-0.1.0
            0.1.0
            "
        ));

    assert_that!(Path::new("CHANGELOG.md")).does_not_exist();
    assert_tag_does_not_exist("1.1.0")?;
    Ok(())
}

#[sealed_test]
fn package_dry_run() -> Result<()> {
    init_monorepo(&mut Settings::default())?;

    Command::cargo_bin("cog")?
        .arg("bump")
        .arg("--auto")
        .arg("--package")
        .arg("one")
        .arg("--dry-run")
        .assert()
        .success()
        .stdout(indoc!("one-0.1.0\n"));

    assert_that!(Path::new("CHANGELOG.md")).does_not_exist();
    assert_tag_does_not_exist("1.1.0")?;
    Ok(())
}

#[sealed_test]
fn uncommited_changes_should_throw_error_by_default() -> Result<()> {
    init_monorepo(&mut Settings::default())?;

    run_cmd!(
        echo two > two;
    )?;

    Command::cargo_bin("cog")?
        .arg("bump")
        .arg("--auto")
        .arg("--dry-run")
        .assert()
        .failure();

    Ok(())
}

#[sealed_test]
fn uncommited_changes_should_not_throw_error_with_option() -> Result<()> {
    let mut settings = Settings {
        skip_untracked: true,
        ..Default::default()
    };

    init_monorepo(&mut settings)?;

    run_cmd!(
        echo two > two;
        echo "other changes" > one/file;
    )?;

    Command::cargo_bin("cog")?
        .arg("bump")
        .arg("--auto")
        .arg("--dry-run")
        .assert()
        .success()
        .stderr(indoc!("Untracked files :\n\tmodified: one/file\n\tnew: two\n\nnothing added to commit but untracked files present (use \"git add\" to track)\n\n"))
        .stdout(indoc!("one-0.1.0\n0.1.0\n"));

    assert_that!(Path::new("CHANGELOG.md")).does_not_exist();
    assert_tag_does_not_exist("1.1.0")?;
    Ok(())
}

#[sealed_test]
fn bump_package_with_default_skip_ci_ok() -> Result<()> {
    let mut settings = Settings {
        ..Default::default()
    };
    init_monorepo(&mut settings)?;

    Command::cargo_bin("cog")?
        .arg("bump")
        .arg("--auto")
        .arg("--skip-ci")
        .arg("--package")
        .arg("one")
        .assert()
        .success();

    let commit_message = git_log_head()?;

    assert_tag_exists("one-0.1.0")?;
    assert!(commit_message.contains("[skip ci]"));

    Ok(())
}

#[sealed_test]
fn bump_package_with_cog_toml_defined_skip_ci_ok() -> Result<()> {
    let mut settings = Settings {
        skip_ci: String::from("[ci-skip]"),
        ..Default::default()
    };
    init_monorepo(&mut settings)?;

    Command::cargo_bin("cog")?
        .arg("bump")
        .arg("--auto")
        .arg("--skip-ci")
        .arg("--package")
        .arg("one")
        .assert()
        .success();

    let commit_message = git_log_head()?;

    assert_tag_exists("one-0.1.0")?;
    assert!(commit_message.contains("[ci-skip]"));

    Ok(())
}

#[sealed_test]
fn bump_package_with_skip_ci_override_option_takes_predecence() -> Result<()> {
    let mut settings = Settings {
        skip_ci: String::from("[ci-skip]"),
        ..Default::default()
    };
    init_monorepo(&mut settings)?;

    Command::cargo_bin("cog")?
        .arg("bump")
        .arg("--auto")
        .arg("--skip-ci-override")
        .arg("[ci-skip-override]")
        .arg("--package")
        .arg("one")
        .assert()
        .success();

    let commit_message = git_log_head()?;

    assert_tag_exists("one-0.1.0")?;
    assert!(commit_message.contains("[ci-skip-override]"));

    Ok(())
}

#[sealed_test]
fn bump_standard_repository_with_default_skip_ci_ok() -> Result<()> {
    git_init()?;
    git_commit("chore: init")?;
    git_commit("feat: feature")?;

    Command::cargo_bin("cog")?
        .arg("bump")
        .arg("--skip-ci")
        .arg("--auto")
        .assert()
        .success();

    let commit_message = git_log_head()?;
    assert_tag_exists("0.1.0")?;
    assert!(commit_message.contains("[skip ci]"));

    Ok(())
}

#[sealed_test]
fn bump_standard_repository_with_cog_toml_defined_skip_ci_ok() -> Result<()> {
    git_init()?;
    git_commit("chore: init")?;
    git_add("skip_ci = \"[ci-skip]\"", "cog.toml")?;
    git_commit("feat: feature")?;

    Command::cargo_bin("cog")?
        .arg("bump")
        .arg("--skip-ci")
        .arg("--auto")
        .assert()
        .success();

    let commit_message = git_log_head()?;
    assert_tag_exists("0.1.0")?;
    assert!(commit_message.contains("[ci-skip]"));

    Ok(())
}

#[sealed_test]
fn bump_standard_repository_skip_ci_override_option_takes_precedence() -> Result<()> {
    git_init()?;
    git_commit("chore: init")?;
    git_add("skip_ci = \"[ci-skip]\"", "cog.toml")?;
    git_commit("feat: feature")?;

    Command::cargo_bin("cog")?
        .arg("bump")
        .arg("--skip-ci-override")
        .arg("[ci-skip-override]")
        .arg("--auto")
        .assert()
        .success();

    let commit_message = git_log_head()?;
    assert_tag_exists("0.1.0")?;
    assert!(commit_message.contains("[ci-skip-override]"));

    Ok(())
}

#[sealed_test]
fn bump_monorepo_with_default_skip_ci_ok() -> Result<()> {
    let mut settings = Settings {
        ..Default::default()
    };
    init_monorepo(&mut settings)?;

    Command::cargo_bin("cog")?
        .arg("bump")
        .arg("--auto")
        .arg("--skip-ci")
        .assert()
        .success();

    let commit_message = git_log_head()?;

    assert_tag_exists("0.1.0")?;
    assert!(commit_message.contains("[skip ci]"));

    Ok(())
}

#[sealed_test]
fn bump_monorepo_with_cog_toml_defined_skip_ci_ok() -> Result<()> {
    let mut settings = Settings {
        skip_ci: String::from("[ci-skip]"),
        ..Default::default()
    };
    init_monorepo(&mut settings)?;

    Command::cargo_bin("cog")?
        .arg("bump")
        .arg("--auto")
        .arg("--skip-ci")
        .assert()
        .success();

    let commit_message = git_log_head()?;

    assert_tag_exists("0.1.0")?;
    assert!(commit_message.contains("[ci-skip]"));

    Ok(())
}

#[sealed_test]
fn bump_monorepo_skip_ci_override_option_takes_precedence() -> Result<()> {
    let mut settings = Settings {
        skip_ci: String::from("[ci-skip]"),
        ..Default::default()
    };
    init_monorepo(&mut settings)?;

    Command::cargo_bin("cog")?
        .arg("bump")
        .arg("--auto")
        .arg("--skip-ci-override")
        .arg("[skip-ci-override]")
        .assert()
        .success();

    let commit_message = git_log_head()?;

    assert_tag_exists("0.1.0")?;
    assert!(commit_message.contains("[skip-ci-override]"));

    Ok(())
}
