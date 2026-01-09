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

    Command::new(assert_cmd::cargo_bin!("cog"))
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

    Command::new(assert_cmd::cargo_bin!("cog"))
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

    Command::new(assert_cmd::cargo_bin!("cog"))
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

    Command::new(assert_cmd::cargo_bin!("cog"))
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

    Command::new(assert_cmd::cargo_bin!("cog"))
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
    Command::new(assert_cmd::cargo_bin!("cog"))
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
    Command::new(assert_cmd::cargo_bin!("cog"))
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

    Command::new(assert_cmd::cargo_bin!("cog"))
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

    Command::new(assert_cmd::cargo_bin!("cog"))
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

    Command::new(assert_cmd::cargo_bin!("cog"))
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

    Command::new(assert_cmd::cargo_bin!("cog"))
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

    Command::new(assert_cmd::cargo_bin!("cog"))
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

    Command::new(assert_cmd::cargo_bin!("cog"))
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

    Command::new(assert_cmd::cargo_bin!("cog"))
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
fn pre_release_bump_auto() -> Result<()> {
    git_init()?;
    git_commit("chore: init")?;
    git_tag("1.0.0")?;
    git_commit("feat: feature")?;

    Command::cargo_bin("cog")?
        .arg("bump")
        .arg("--major")
        .arg("--auto-pre")
        .arg("--pre-pattern")
        .arg("alpha.*")
        .assert()
        .success();

    assert_that!(Path::new("CHANGELOG.md")).exists();
    assert_tag_exists("2.0.0-alpha.1")?;
    Ok(())
}

#[sealed_test]
fn build_release_bump() -> Result<()> {
    git_init()?;
    git_commit("chore: init")?;
    git_tag("1.0.0")?;
    git_commit("feat: feature")?;

    Command::new(assert_cmd::cargo_bin!("cog"))
        .arg("bump")
        .arg("--major")
        .arg("--build")
        .arg("a.b.c")
        .assert()
        .success();

    assert_that!(Path::new("CHANGELOG.md")).exists();
    assert_tag_exists("2.0.0+a.b.c")?;
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
    Command::new(assert_cmd::cargo_bin!("cog"))
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
fn bump_with_hook_and_prefix() -> Result<()> {
    // Arrange
    git_init()?;
    git_add(
        r#"tag_prefix = "v"
        pre_bump_hooks = ["touch {{version}}", "touch {{version_tag}}"]"#,
        "cog.toml",
    )?;
    git_commit("chore: init")?;
    git_tag("v1.0.0")?;
    git_commit("feat: feature")?;

    // Act
    Command::new(assert_cmd::cargo_bin!("cog"))
        .arg("bump")
        .arg("--major")
        // Assert
        .assert()
        .success();

    assert_that!(Path::new("2.0.0")).exists();
    assert_that!(Path::new("v2.0.0")).exists();
    assert_tag_exists("v2.0.0")?;
    assert_tag_does_not_exist("2.0.0")?;
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
    Command::new(assert_cmd::cargo_bin!("cog"))
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

    Command::new(assert_cmd::cargo_bin!("cog"))
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

    Command::new(assert_cmd::cargo_bin!("cog"))
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
fn uncommitted_changes_should_throw_error_by_default() -> Result<()> {
    init_monorepo(&mut Settings::default())?;

    run_cmd!(
        echo two > two;
    )?;

    Command::new(assert_cmd::cargo_bin!("cog"))
        .arg("bump")
        .arg("--auto")
        .arg("--dry-run")
        .assert()
        .failure();

    Ok(())
}

#[sealed_test]
fn uncommitted_changes_should_not_throw_error_with_option() -> Result<()> {
    let mut settings = Settings {
        skip_untracked: true,
        ..Default::default()
    };

    init_monorepo(&mut settings)?;

    run_cmd!(
        echo two > two;
        echo "other changes" > one/file;
    )?;

    Command::new(assert_cmd::cargo_bin!("cog"))
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

    Command::new(assert_cmd::cargo_bin!("cog"))
        .arg("bump")
        .arg("--auto")
        .arg("--skip-ci")
        .arg("--package")
        .arg("one")
        .assert()
        .success();

    let commit_message = git_log_head_message()?;

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

    Command::new(assert_cmd::cargo_bin!("cog"))
        .arg("bump")
        .arg("--auto")
        .arg("--skip-ci")
        .arg("--package")
        .arg("one")
        .assert()
        .success();

    let commit_message = git_log_head_message()?;

    assert_tag_exists("one-0.1.0")?;
    assert!(commit_message.contains("[ci-skip]"));

    Ok(())
}

#[sealed_test]
fn bump_package_with_skip_ci_override_option_takes_precedence() -> Result<()> {
    let mut settings = Settings {
        skip_ci: String::from("[ci-skip]"),
        ..Default::default()
    };
    init_monorepo(&mut settings)?;

    Command::new(assert_cmd::cargo_bin!("cog"))
        .arg("bump")
        .arg("--auto")
        .arg("--skip-ci-override")
        .arg("[ci-skip-override]")
        .arg("--package")
        .arg("one")
        .assert()
        .success();

    let commit_message = git_log_head_message()?;

    assert_tag_exists("one-0.1.0")?;
    assert!(commit_message.contains("[ci-skip-override]"));

    Ok(())
}

#[sealed_test]
fn bump_standard_repository_with_default_skip_ci_ok() -> Result<()> {
    git_init()?;
    git_commit("chore: init")?;
    git_commit("feat: feature")?;

    Command::new(assert_cmd::cargo_bin!("cog"))
        .arg("bump")
        .arg("--skip-ci")
        .arg("--auto")
        .assert()
        .success();

    let commit_message = git_log_head_message()?;
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

    Command::new(assert_cmd::cargo_bin!("cog"))
        .arg("bump")
        .arg("--skip-ci")
        .arg("--auto")
        .assert()
        .success();

    let commit_message = git_log_head_message()?;
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

    Command::new(assert_cmd::cargo_bin!("cog"))
        .arg("bump")
        .arg("--skip-ci-override")
        .arg("[ci-skip-override]")
        .arg("--auto")
        .assert()
        .success();

    let commit_message = git_log_head_message()?;
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

    Command::new(assert_cmd::cargo_bin!("cog"))
        .arg("bump")
        .arg("--auto")
        .arg("--skip-ci")
        .assert()
        .success();

    let commit_message = git_log_head_message()?;

    assert_tag_exists("0.1.0")?;
    assert!(commit_message.contains("[skip ci]"));

    Ok(())
}

#[sealed_test]
fn bump_monorepo_manual_increment_with_default_skip_ci_ok() -> Result<()> {
    let mut settings = Settings {
        ..Default::default()
    };
    init_monorepo(&mut settings)?;

    Command::new(assert_cmd::cargo_bin!("cog"))
        .arg("bump")
        .arg("--minor")
        .arg("--skip-ci")
        .assert()
        .success();

    let commit_message = git_log_head_message()?;

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

    Command::new(assert_cmd::cargo_bin!("cog"))
        .arg("bump")
        .arg("--auto")
        .arg("--skip-ci")
        .assert()
        .success();

    let commit_message = git_log_head_message()?;

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

    Command::new(assert_cmd::cargo_bin!("cog"))
        .arg("bump")
        .arg("--auto")
        .arg("--skip-ci-override")
        .arg("[skip-ci-override]")
        .assert()
        .success();

    let commit_message = git_log_head_message()?;

    assert_tag_exists("0.1.0")?;
    assert!(commit_message.contains("[skip-ci-override]"));

    Ok(())
}

#[sealed_test]
fn bump_only_package_with_default_skip_ci_ok() -> Result<()> {
    let mut settings = Settings {
        generate_mono_repository_global_tag: false,
        ..Default::default()
    };
    init_monorepo(&mut settings)?;

    Command::new(assert_cmd::cargo_bin!("cog"))
        .arg("bump")
        .arg("--auto")
        .arg("--skip-ci")
        .assert()
        .success();

    let commit_message = git_log_head_message()?;

    assert_tag_does_not_exist("0.1.0")?;
    assert_tag_exists("one-0.1.0")?;
    assert!(commit_message.contains("[skip ci]"));

    Ok(())
}

#[sealed_test]
fn disable_commit_creation_with_config_standard_ok() -> Result<()> {
    git_init()?;

    git_add("disable_bump_commit = true", "cog.toml")?;

    git_commit("chore: init")?;
    git_commit("feat: feature")?;

    Command::new(assert_cmd::cargo_bin!("cog"))
        .arg("bump")
        .arg("--auto")
        .assert()
        .success();

    assert_tag_exists("0.1.0")?;

    Command::new("git")
        .arg("status")
        .arg("-s")
        .assert()
        .success()
        .stdout(indoc!("A  CHANGELOG.md\n"));

    Ok(())
}

#[sealed_test]
fn disable_commit_creation_with_flag_standard_ok() -> Result<()> {
    git_init()?;

    git_commit("chore: init")?;
    git_commit("feat: feature")?;

    Command::new(assert_cmd::cargo_bin!("cog"))
        .arg("bump")
        .arg("--auto")
        .arg("--disable-bump-commit")
        .assert()
        .success();

    assert_tag_exists("0.1.0")?;

    Command::new("git")
        .arg("status")
        .arg("-s")
        .assert()
        .success()
        .stdout(indoc!("A  CHANGELOG.md\n"));

    Ok(())
}

#[sealed_test]
#[cfg(target_os = "linux")]
fn disable_commit_creation_with_pre_bump_hooks_standard_ok() -> Result<()> {
    git_init()?;

    git_add(
        "pre_bump_hooks = [\"echo pre_bump_file > pre_bump_file\"]\ndisable_bump_commit = true",
        "cog.toml",
    )?;

    git_commit("chore: init")?;
    git_commit("feat: feature")?;

    Command::new(assert_cmd::cargo_bin!("cog"))
        .arg("bump")
        .arg("--auto")
        .arg("--disable-bump-commit")
        .assert()
        .success();

    assert_tag_exists("0.1.0")?;

    Command::new("git")
        .arg("status")
        .arg("-s")
        .assert()
        .success()
        .stdout(indoc!("A  CHANGELOG.md\nA  pre_bump_file\n"));

    Ok(())
}

#[sealed_test]
fn override_default_commit() -> Result<()> {
    git_init()?;

    git_add(
        indoc! {
          r#"[commit_types]
          feat = { changelog_title = "🌟 Features" }
          fix = { changelog_title = "🐛 Bug Fixes" }"#
        },
        "cog.toml",
    )?;

    git_commit("chore: init")?;
    git_commit("feat: feature")?;
    git_commit("fix: fix")?;

    Command::new(assert_cmd::cargo_bin!("cog"))
        .arg("bump")
        .arg("--auto")
        .assert()
        .success();

    assert_tag_exists("0.1.0")?;

    Ok(())
}

#[sealed_test]
fn disable_commit_creation_monorepo_ok() -> Result<()> {
    let mut settings = Settings {
        disable_bump_commit: true,
        ..Default::default()
    };
    init_monorepo(&mut settings)?;

    Command::new(assert_cmd::cargo_bin!("cog"))
        .arg("bump")
        .arg("--auto")
        .assert()
        .success();

    assert_tag_exists("0.1.0")?;
    assert_tag_exists("one-0.1.0")?;

    Command::new("git")
        .arg("status")
        .arg("-s")
        .assert()
        .success()
        .stdout(indoc!("A  CHANGELOG.md\nA  one/CHANGELOG.md\n"));

    Ok(())
}

#[sealed_test]
fn disable_commit_creation_package_ok() -> Result<()> {
    let mut settings = Settings {
        disable_bump_commit: true,
        ..Default::default()
    };
    init_monorepo(&mut settings)?;

    Command::new(assert_cmd::cargo_bin!("cog"))
        .arg("bump")
        .arg("--auto")
        .arg("--package")
        .arg("one")
        .arg("--disable-bump-commit")
        .assert()
        .success();

    assert_tag_exists("one-0.1.0")?;

    Command::new("git")
        .arg("status")
        .arg("-s")
        .assert()
        .success()
        .stdout(indoc!("A  one/CHANGELOG.md\n"));

    Ok(())
}

#[sealed_test]
fn bump_repeatedly() -> Result<()> {
    // Arrange
    git_init()?;
    git_commit("feat: first commit")?;

    // Act
    Command::new(assert_cmd::cargo_bin!("cog"))
        .arg("bump")
        .arg("--auto")
        .arg("--build")
        .arg("test.1")
        .assert()
        .success();

    run_cmd!(git reset --hard HEAD^)?;

    Command::new(assert_cmd::cargo_bin!("cog"))
        .arg("bump")
        .arg("--auto")
        .arg("--build")
        .arg("test.2")
        .assert()
        .success();

    run_cmd!(git reset --hard HEAD^)?;

    Command::new(assert_cmd::cargo_bin!("cog"))
        .arg("bump")
        .arg("--auto")
        .arg("--build")
        .arg("test.3")
        .assert()
        .success();

    // Assert
    assert_tag_exists("0.1.0+test.1")?;
    assert_tag_exists("0.1.0+test.2")?;
    assert_tag_exists("0.1.0+test.3")?;

    Ok(())
}

#[sealed_test]
fn bump_bug_fix() -> Result<()> {
    // Arrange
    git_init()?;
    git_commit("feat: first commit")?;
    git_tag("1.0.0")?;
    git_commit("feat: amazing stuff")?;
    git_commit("feat!: break everything")?;
    git_tag("2.0.0")?;
    run_cmd!(
        git branch bugfix 1.0.0;
        git switch bugfix;
    )?;
    git_commit("fix: important bug fix")?;

    // Act
    Command::new(assert_cmd::cargo_bin!("cog"))
        .arg("bump")
        .arg("--auto")
        .assert()
        .success();

    // Assert
    assert_tag_exists("1.0.1")?;

    Ok(())
}

#[sealed_test]
fn changelog_on_first_commit_with_tag_on_first_commit() -> Result<()> {
    git_init()?;
    git_commit("feat: init")?;

    Command::new(assert_cmd::cargo_bin!("cog"))
        .arg("bump")
        .arg("--auto")
        .arg("--disable-bump-commit")
        .assert()
        .success();

    Command::new(assert_cmd::cargo_bin!("cog"))
        .arg("changelog")
        .arg("--at")
        .arg("0.1.0")
        .assert()
        .success();

    Ok(())
}

#[sealed_test]
fn bump_from_latest_pre_release() -> Result<()> {
    // Arrange
    git_init()?;
    git_commit("chore: init")?;
    git_commit("feat: feature 1")?;
    git_tag("1.0.0-alpha.0")?;
    git_commit("feat: feature 2")?;

    // Act
    Command::new(assert_cmd::cargo_bin!("cog"))
        .arg("bump")
        .arg("--auto")
        .assert()
        .success();

    // Assert
    assert_tag_exists("1.0.0")?;

    Ok(())
}

#[sealed_test]
fn bump_prerelease_from_latest_pre_release() -> Result<()> {
    // Arrange
    git_init()?;
    git_commit("chore: init")?;
    git_commit("feat: feature 1")?;
    git_tag("1.0.0-alpha.0")?;
    git_commit("feat: feature 2")?;

    // Act
    Command::new(assert_cmd::cargo_bin!("cog"))
        .arg("bump")
        .arg("--auto")
        .arg("--pre")
        .arg("beta.0")
        .assert()
        .success();

    // Assert
    assert_tag_exists("1.0.0-beta.0")?;

    Ok(())
}

#[sealed_test]
fn bump_prerelease_from_latest_pre_release_auto() -> Result<()> {
    // Arrange
    git_init()?;
    git_commit("chore: init")?;
    git_commit("feat: feature 1")?;
    git_tag("1.0.0-alpha.1")?;
    git_commit("feat: feature 2")?;

    // Act
    Command::cargo_bin("cog")?
        .arg("bump")
        .arg("--auto")
        .arg("--auto-pre")
        .arg("--pre-pattern")
        .arg("alpha.*")
        .assert()
        .success();

    // Assert
    assert_tag_exists("1.0.0-alpha.2")?;

    Ok(())
}

#[sealed_test]
fn bump_prerelease_from_latest_pre_release_auto_2() -> Result<()> {
    // Arrange
    git_init()?;
    git_commit("chore: init")?;
    git_commit("feat: feature 1")?;
    git_tag("1.0.0")?;
    git_commit("feat: feature 2")?;

    // Act
    Command::cargo_bin("cog")?
        .arg("bump")
        .arg("--auto")
        .arg("--auto-pre")
        .arg("--pre-pattern")
        .arg("alpha.*")
        .assert()
        .success();

    // Assert
    assert_tag_exists("1.1.0-alpha.1")?;

    Ok(())
}

#[sealed_test]
fn bump_prerelease_from_latest_pre_release_auto_3() -> Result<()> {
    // Arrange
    git_init()?;
    git_commit("chore: init")?;
    git_commit("feat: feature 1")?;
    git_tag("1.0.0")?;
    git_commit("fix: fix 1")?;
    git_tag("1.0.1-alpha.1")?;
    git_commit("fix: fix 2")?;
    git_tag("1.0.1-alpha.2")?;
    git_commit("feat: feature 2")?;

    // Act
    Command::cargo_bin("cog")?
        .arg("bump")
        .arg("--auto")
        .arg("--auto-pre")
        .arg("--pre-pattern")
        .arg("alpha.*")
        .assert()
        .success();

    // Assert
    assert_tag_exists("1.1.0-alpha.1")?;

    Ok(())
}
#[sealed_test]
fn bump_from_latest_pre_release_monorepo() -> Result<()> {
    // Arrange
    git_init()?;
    git_add("packages.pkg.path = \"pkg\"", "cog.toml")?;
    git_commit("chore: init")?;
    git_commit("feat: feature 1")?;
    git_tag("1.0.0-alpha.0")?;
    git_tag("pkg-1.0.0-alpha.0")?;
    std::fs::create_dir("pkg")?;
    git_add("fn main() {}", "pkg/main.rs")?;
    git_commit("feat: feature 2")?;

    // Act
    Command::new(assert_cmd::cargo_bin!("cog"))
        .arg("bump")
        .arg("--auto")
        .assert()
        .success();

    // Assert
    assert_tag_exists("1.0.0")?;
    assert_tag_exists("pkg-1.0.0")?;

    Ok(())
}

#[sealed_test]
fn bump_prerelease_from_latest_pre_release_monorepo() -> Result<()> {
    // Arrange
    git_init()?;
    git_add("packages.pkg.path = \"pkg\"", "cog.toml")?;
    git_commit("chore: init")?;
    git_commit("feat: feature 1")?;
    git_tag("1.0.0-alpha.0")?;
    git_tag("pkg-1.0.0-alpha.0")?;
    std::fs::create_dir("pkg")?;
    git_add("fn main() {}", "pkg/main.rs")?;
    git_commit("feat: feature 2")?;

    // Act
    Command::new(assert_cmd::cargo_bin!("cog"))
        .arg("bump")
        .arg("--auto")
        .arg("--pre")
        .arg("beta.0")
        .assert()
        .success();

    // Assert
    assert_tag_exists("1.0.0-beta.0")?;
    assert_tag_exists("pkg-1.0.0-beta.0")?;

    Ok(())
}

#[sealed_test]
fn bump_prerelease_ignore_packages() -> Result<()> {
    // Arrange
    git_init()?;
    git_add(
        indoc! {
            r#"
            [packages.a]
            path = "a"

            [packages.b]
            path = "b"
            "#
        },
        "cog.toml",
    )?;
    git_commit("chore: init")?;
    git_tag("1.0.0")?;
    git_tag("a-1.0.0")?;
    git_tag("b-1.0.0")?;
    git_add(".", "global")?;
    git_add(".", "a/file")?;
    git_commit("feat: do stuff")?;

    // Act
    Command::new(assert_cmd::cargo_bin!("cog"))
        .arg("bump")
        .arg("--auto")
        .arg("--pre")
        .arg("rc.1")
        .assert()
        .success();

    // Assert
    assert_tag_exists("1.1.0-rc.1")?;
    assert_tag_exists("a-1.1.0-rc.1")?;
    assert_tag_does_not_exist("b-1.0.0-rc.1")
}

#[sealed_test]
fn major_bump_with_packages() -> Result<()> {
    // Arrange
    git_init()?;
    git_add(
        indoc! {
            r#"
            [packages.a]
            path = "a"

            [packages.b]
            path = "b"
            "#
        },
        "cog.toml",
    )?;
    git_commit("chore: init")?;
    git_tag("0.1.0")?;
    git_tag("a-0.1.0")?;
    git_tag("b-0.1.0")?;

    git_add(".", "a/feat")?;
    git_add(".", "b/feat")?;
    git_add(".", "feat")?;
    git_commit("feat: release 1.0")?;

    // Act
    Command::new(assert_cmd::cargo_bin!("cog"))
        .arg("bump")
        .arg("--major")
        .arg("--include-packages")
        .assert()
        .success();

    // Assert
    assert_tag_exists("1.0.0")?;
    assert_tag_exists("a-1.0.0")?;
    assert_tag_exists("b-1.0.0")?;

    Ok(())
}

#[sealed_test]
fn auto_bump_conflicts_with_include_packages() -> Result<()> {
    Command::new(assert_cmd::cargo_bin!("cog"))
        .arg("bump")
        .arg("--auto")
        .arg("--include-packages")
        .assert()
        .failure();

    Ok(())
}
