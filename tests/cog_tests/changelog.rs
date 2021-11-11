use anyhow::Result;
use assert_cmd::Command;
use chrono::Utc;
use indoc::formatdoc;
use pretty_assertions::assert_eq;

use cocogitto::settings::Settings;

use crate::helpers::*;

#[test]
fn get_changelog_range() -> Result<()> {
    // Running against cocogitto git history here

    // Act
    let changelog = Command::cargo_bin("cog")?
        .arg("changelog")
        .arg("0.30.0..0.32.3")
        .arg("-t")
        .arg("default")
        // Assert
        .assert()
        .success();

    let changelog = changelog.get_output();
    let changelog = &changelog.stdout;
    let changelog = String::from_utf8_lossy(changelog.as_slice());
    let today = Utc::today().naive_utc().to_string();

    assert_eq!(
        changelog.as_ref(),
        formatdoc!(
            "## 0.32.3 - {today}
                #### Bug Fixes
                - fix openssl missing in CD - (1c0d2e9) - oknozor
                #### Documentation
                - tag, conventional commit and license badges to readme - (da6f63d) - oknozor
                #### Miscellaneous Chores
                - **(version)** 0.32.3 - (0939f4c) - *oknozor*
                - - -
                ## 0.32.2 - {today}
                #### Bug Fixes
                - **(cd)** bump setup-rust-action to v1.3.3 - (5350b11) - *oknozor*
                #### Documentation
                - add corrections to README - (9a33516) - oknozor
                #### Miscellaneous Chores
                - **(version)** 0.32.2 - (ef4803b) - *oknozor*
                - - -
                ## 0.32.1 - {today}
                #### Bug Fixes
                - **(cd)** fix ci cross build command bin args - (7f04a98) - *oknozor*
                #### Documentation
                - rewritte readme completely - (b223f7b) - oknozor
                #### Features
                - move check edit to dedicated subcommand and fix rebase - (fc74207) - oknozor
                - remove config commit on init existing repo - (1028d0b) - oknozor
                #### Miscellaneous Chores
                - **(version)** 0.32.1 - (5bcfd6f) - *oknozor*
                - update Cargo.toml - (72bd1e4) - oknozor
                #### Refactoring
                - change config name to cog.toml - (d4aa61b) - oknozor
                - - -
                ## 0.30.0 - {today}
                #### Continuous Integration
                - **(cd)** fix publish action script - (d0d0ae9) - *oknozor*
                #### Features
                - **(changelog)** improve changelog title formatting - (d713886) - *oknozor*
                #### Miscellaneous Chores
                - **(version)** 0.30.0 - (d7ff5d9) - *oknozor*
                - remove test generated dir - (a6fba9c) - oknozor
                #### Tests
                - **(cli)** add verify it tests - (9da7321) - *oknozor*
                ",
            today = today
        )
    );
    Ok(())
}

#[test]
fn get_changelog_from_untagged_repo() -> Result<()> {
    run_test_with_context(|_| {
        // Arrange
        git_init()?;
        let _ = git_commit("chore: init")?;
        let commit_two = git_commit("feat(taef): feature")?;
        let commit_three = git_commit("fix: bug fix")?;

        // Act
        let changelog = Command::cargo_bin("cog")?
            .arg("changelog")
            // Assert
            .assert()
            .success();

        let changelog = changelog.get_output();
        let changelog = &changelog.stdout;
        let changelog = String::from_utf8_lossy(changelog.as_slice());

        assert_eq!(
            changelog.as_ref(),
            formatdoc!(
                "## Unreleased ({commit_two}..{commit_three})
                    #### Bug Fixes
                    - bug fix - ({commit_three}) - Tom
                    #### Features
                    - **(taef)** feature - ({commit_two}) - Tom
                    ",
                commit_two = &commit_two[0..7],
                commit_three = &commit_three[0..7]
            )
        );
        Ok(())
    })
}

#[test]
fn get_changelog_from_tagged_repo() -> Result<()> {
    run_test_with_context(|_| {
        // Arrange
        git_init()?;
        git_commit("chore: init")?;
        let commit_one = git_commit("feat(taef): feature")?;
        git_tag("1.0.0")?;
        let commit_two = git_commit("fix: bug fix")?;

        // Act
        let changelog = Command::cargo_bin("cog")?
            .arg("changelog")
            // Assert
            .assert()
            .success();

        let changelog = changelog.get_output();
        let changelog = &changelog.stdout;
        let changelog = String::from_utf8_lossy(changelog.as_slice());
        let today = Utc::today().naive_utc().to_string();

        assert_eq!(
            changelog.as_ref(),
            formatdoc!(
                "## Unreleased ({commit_two}..{commit_two})
                    #### Bug Fixes
                    - bug fix - ({commit_two}) - Tom
                    - - -
                    ## 1.0.0 - {today}
                    #### Features
                    - **(taef)** feature - ({commit_one}) - Tom
                    ",
                commit_one = &commit_one[0..7],
                commit_two = &commit_two[0..7],
                today = today
            )
        );
        Ok(())
    })
}

#[test]
fn get_changelog_at_tag() -> Result<()> {
    run_test_with_context(|_| {
        // Arrange
        git_init()?;
        git_commit("chore: init")?;
        let commit_one = git_commit("feat(taef): feature")?;
        let commit_two = git_commit("feat: feature 2")?;
        git_tag("1.0.0")?;
        let _ = git_commit("fix: bug fix")?;
        git_log()?;

        // Act
        let changelog = Command::cargo_bin("cog")?
            .arg("changelog")
            .arg("--at")
            .arg("1.0.0")
            // Assert
            .assert()
            .success();

        let changelog = changelog.get_output();
        let changelog = &changelog.stdout;
        let changelog = String::from_utf8_lossy(changelog.as_slice());
        let today = Utc::today().naive_utc();

        assert_eq!(
            changelog.as_ref(),
            formatdoc!(
                "## 1.0.0 - {today}
                    #### Features
                    - **(taef)** feature - ({commit_one}) - Tom
                    - feature 2 - ({commit_two}) - Tom
                    ",
                today = today,
                commit_one = &commit_one[0..7],
                commit_two = &commit_two[0..7]
            )
        );
        Ok(())
    })
}

#[test]
fn get_changelog_with_tag_prefix() -> Result<()> {
    run_test_with_context(|context| {
        // Arrange
        let settings = Settings {
            tag_prefix: Some("v".to_string()),
            ..Default::default()
        };

        let settings = toml::to_string(&settings);
        std::fs::write(context.test_dir.join("cog.toml"), settings?)?;

        git_init()?;
        let _ = git_commit("chore: init")?;
        let commit_one = git_commit("feat: feature 1")?;
        git_tag("v1.0.0")?;
        let commit_two = git_commit("fix: bug fix 1")?;
        git_log()?;

        // Act
        let changelog = Command::cargo_bin("cog")?
            .arg("changelog")
            // Assert
            .assert()
            .success();

        let changelog = changelog.get_output();
        let changelog = &changelog.stdout;
        let changelog = String::from_utf8_lossy(changelog.as_slice());
        let today = Utc::today().naive_utc();

        assert_eq!(
            changelog.as_ref(),
            formatdoc!(
                "## Unreleased ({commit_two}..{commit_two})
                    #### Bug Fixes
                    - bug fix 1 - ({commit_two}) - Tom
                    - - -
                    ## v1.0.0 - {today}
                    #### Features
                    - feature 1 - ({commit_one}) - Tom
                    ",
                today = today,
                commit_one = &commit_one[0..7],
                commit_two = &commit_two[0..7]
            )
        );

        Ok(())
    })
}

#[test]
fn get_changelog_at_tag_prefix() -> Result<()> {
    run_test_with_context(|context| {
        // Arrange
        let settings = Settings {
            tag_prefix: Some("v".to_string()),
            ..Default::default()
        };

        let settings = toml::to_string(&settings);
        std::fs::write(context.test_dir.join("cog.toml"), settings?)?;

        git_init()?;
        git_commit("chore: init")?;
        let _ = git_commit("feat: start")?;
        git_tag("v1.0.0")?;
        let commit_two = git_commit("feat: feature 1")?;
        let commit_three = git_commit("fix: bug fix 1")?;
        let commit_four = git_commit("chore(version): v2.0.0")?;
        git_tag("v2.0.0")?;
        let _ = git_commit("feat: end")?;
        git_log()?;

        // Act
        let changelog = Command::cargo_bin("cog")?
            .arg("changelog")
            .arg("--at")
            .arg("v2.0.0")
            // Assert
            .assert()
            .success();

        let changelog = changelog.get_output();
        let changelog = String::from_utf8_lossy(&changelog.stdout);
        let today = Utc::today().naive_utc();

        assert_eq!(
            changelog.as_ref(),
            formatdoc!(
                "## v2.0.0 - {today}
                    #### Bug Fixes
                    - bug fix 1 - ({commit_three}) - Tom
                    #### Features
                    - feature 1 - ({commit_two}) - Tom
                    #### Miscellaneous Chores
                    - **(version)** v2.0.0 - ({commit_four}) - Tom
                    ",
                today = today,
                commit_two = &commit_two[0..7],
                commit_three = &commit_three[0..7],
                commit_four = &commit_four[0..7]
            )
        );
        Ok(())
    })
}
