use anyhow::Result;
use assert_cmd::Command;
use chrono::Utc;
use cmd_lib::run_cmd;
use indoc::{formatdoc, indoc};
use pretty_assertions::assert_eq;
use sealed_test::prelude::*;
use std::fs;
use std::path::PathBuf;

use cocogitto::settings::Settings;

use crate::helpers::*;

#[test]
fn get_changelog_range() -> Result<()> {
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
    let today = Utc::now().date_naive().to_string();

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


                ",
            today = today
        )
    );
    Ok(())
}

#[sealed_test]
fn get_changelog_from_untagged_repo() -> Result<()> {
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
}

#[sealed_test]
fn get_changelog_from_tagged_repo() -> Result<()> {
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
    let today = Utc::now().date_naive().to_string();

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
}

#[sealed_test]
fn get_changelog_at_tag() -> Result<()> {
    // Arrange
    git_init()?;
    git_commit("chore: init")?;
    let commit_one = git_commit("feat(taef): feature")?;
    let commit_two = git_commit("feat: feature 2")?;
    git_tag("1.0.0")?;
    let _ = git_commit("fix: bug fix")?;

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
    let today = Utc::now().date_naive();

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
}

#[sealed_test]
fn get_changelog_with_tag_prefix() -> Result<()> {
    // Arrange
    let settings = Settings {
        tag_prefix: Some("v".to_string()),
        ..Default::default()
    };

    let settings = toml::to_string(&settings);
    std::fs::write("cog.toml", settings?)?;

    git_init()?;
    let _ = git_commit("chore: init")?;
    let commit_one = git_commit("feat: feature 1")?;
    git_tag("v1.0.0")?;
    let commit_two = git_commit("fix: bug fix 1")?;

    // Act
    let changelog = Command::cargo_bin("cog")?
        .arg("changelog")
        // Assert
        .assert()
        .success();

    let changelog = changelog.get_output();
    let changelog = &changelog.stdout;
    let changelog = String::from_utf8_lossy(changelog.as_slice());
    let today = Utc::now().date_naive();

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
}

#[sealed_test]
fn get_changelog_at_tag_prefix() -> Result<()> {
    // Arrange
    let settings = Settings {
        tag_prefix: Some("v".to_string()),
        ..Default::default()
    };

    let settings = toml::to_string(&settings);
    std::fs::write("cog.toml", settings?)?;

    git_init()?;
    git_commit("chore: init")?;
    let _ = git_commit("feat: start")?;
    git_tag("v1.0.0")?;
    let commit_two = git_commit("feat: feature 1")?;
    let commit_three = git_commit("fix: bug fix 1")?;
    let commit_four = git_commit("chore(version): v2.0.0")?;
    git_tag("v2.0.0")?;
    let _ = git_commit("feat: end")?;

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
    let today = Utc::now().date_naive();

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
}

#[sealed_test]
fn get_changelog_from_tag_to_tagged_head() -> Result<()> {
    // Arrange
    git_init()?;
    git_commit("chore: init")?;
    let commit_one = git_commit("feat: start")?;
    let commit_two = git_commit("feat: feature 1")?;
    git_tag("1.0.0")?;
    let commit_three = git_commit("feat: feature 2")?;
    let commit_four = git_commit("fix: bug fix 1")?;
    let commit_five = git_commit("chore(version): 2.0.0")?;
    git_tag("2.0.0")?;

    // Act
    let changelog = Command::cargo_bin("cog")?
        .arg("changelog")
        // Assert
        .assert()
        .success();

    let changelog = changelog.get_output();
    let changelog = String::from_utf8_lossy(&changelog.stdout);
    let today = Utc::now().date_naive();

    assert_eq!(
        changelog.as_ref(),
        formatdoc!(
            "## 2.0.0 - {today}
                #### Bug Fixes
                - bug fix 1 - ({commit_four}) - Tom
                #### Features
                - feature 2 - ({commit_three}) - Tom
                #### Miscellaneous Chores
                - **(version)** 2.0.0 - ({commit_five}) - Tom

                - - -

                ## 1.0.0 - {today}
                #### Features
                - feature 1 - ({commit_two}) - Tom
                - start - ({commit_one}) - Tom


                ",
            today = today,
            commit_one = &commit_one[0..7],
            commit_two = &commit_two[0..7],
            commit_three = &commit_three[0..7],
            commit_four = &commit_four[0..7],
            commit_five = &commit_five[0..7],
        )
    );
    Ok(())
}

#[sealed_test]
fn get_changelog_is_unaffected_by_disable_changelog() -> Result<()> {
    // Arrange
    git_init()?;

    let cog_toml = indoc!("disable_changelog = true");

    run_cmd!(echo $cog_toml > cog.toml;)?;
    git_commit("chore: init")?;
    let commit_one = git_commit("feat: start")?;
    let commit_two = git_commit("feat: feature 1")?;
    git_tag("1.0.0")?;

    // Act
    let changelog = Command::cargo_bin("cog")?
        .arg("changelog")
        // Assert
        .assert()
        .success();

    let changelog = changelog.get_output();
    let changelog = String::from_utf8_lossy(&changelog.stdout);
    let today = Utc::now().date_naive();

    assert_eq!(
        changelog.as_ref(),
        formatdoc!(
            "## 1.0.0 - {today}
                #### Features
                - feature 1 - ({commit_two}) - Tom
                - start - ({commit_one}) - Tom


                ",
            today = today,
            commit_one = &commit_one[0..7],
            commit_two = &commit_two[0..7],
        )
    );
    Ok(())
}

#[sealed_test]
fn get_changelog_with_custom_template() -> Result<()> {
    // Arrange
    let crate_dir = env!("CARGO_MANIFEST_DIR");
    let template = PathBuf::from(crate_dir).join("tests/cog_tests/template.md");

    git_init()?;

    let cog_toml = indoc!(
        "[changelog]
        remote = \"github.com\"
        repository = \"test\"
        owner = \"test\""
    );

    run_cmd!(echo $cog_toml > cog.toml;)?;

    let _string = fs::read_to_string("cog.toml")?;
    let init_commit = git_commit("chore: init")?;
    let commit_one = git_commit("feat(scope1): start")?;
    let commit_two = git_commit("feat: feature 1")?;
    git_tag("1.0.0")?;
    let commit_three = git_commit("feat: feature 2")?;
    let commit_four = git_commit("fix: bug fix 1")?;
    let commit_five = git_commit("chore(version): 2.0.0")?;
    git_tag("2.0.0")?;

    // Act
    let changelog = Command::cargo_bin("cog")?
        .arg("changelog")
        .arg("-t")
        .arg(template)
        // Assert
        .assert()
        .success();

    let changelog = changelog.get_output();
    let changelog = String::from_utf8_lossy(&changelog.stdout);
    let today = Utc::now().date_naive();

    assert_eq!(
        changelog.as_ref(),
        formatdoc!(
            "## [2.0.0](https://github.com/test/test/compare/1.0.0..2.0.0) - {today}
            #### Bug Fixes
            -  bug fix 1 - ([{commit_four_short}](https://github.com/test/test/commit/{commit_four})) - Tom
            #### Features
            -  feature 2 - ([{commit_three_short}](https://github.com/test/test/commit/{commit_three})) - Tom
            #### Miscellaneous Chores
            - **(version)** 2.0.0 - ([{commit_five_short}](https://github.com/test/test/commit/{commit_five})) - Tom

            - - -

            ## [1.0.0](https://github.com/test/test/compare/{init_commit}..1.0.0) - {today}
            #### Features
            -  feature 1 - ([{commit_two_short}](https://github.com/test/test/commit/{commit_two})) - Tom
            - **(scope1)** start - ([{commit_one_short}](https://github.com/test/test/commit/{commit_one})) - Tom


            ",
            today = today,
            init_commit = &init_commit,
            commit_one = &commit_one,
            commit_one_short = &commit_one[0..7],
            commit_two = &commit_two,
            commit_two_short = &commit_two[0..7],
            commit_three = &commit_three,
            commit_three_short = &commit_three[0..7],
            commit_four = &commit_four,
            commit_four_short = &commit_four[0..7],
            commit_five = &commit_five,
            commit_five_short = &commit_five[0..7],
        )
    );
    Ok(())
}

#[sealed_test]
/// Test that the `omit_from_changelog` configuration
/// directive is honored if/when it is specified for
/// a given commit type.
fn ensure_omit_from_changelog_is_honored() -> Result<()> {
    // Arrange
    git_init()?;

    let cog_toml = indoc!(
        "[changelog]
        remote = \"github.com\"
        repository = \"test\"
        owner = \"test\"

        [commit_types]
        wip = { changelog_title = \"Work In Progress\", omit_from_changelog = false }"
    );

    let _setup = (
        run_cmd!(echo $cog_toml > cog.toml;)?,
        fs::read_to_string("cog.toml")?,
        git_commit("chore: init")?,
        git_commit("wip(some-scope): getting there")?,
        git_tag("1.0.0")?,
    );

    let changelog = Command::cargo_bin("cog")?
        .arg("changelog")
        // Assert
        .assert()
        .success();

    let changelog = changelog.get_output();
    let changelog = String::from_utf8_lossy(&changelog.stdout);

    assert!(
        changelog.as_ref().contains("#### Work In Progress"),
        "Expected changelog to contain a \"Work In Progress\" entry but got:\n\n{}",
        changelog.as_ref()
    );

    let cog_toml = cog_toml.replace("omit_from_changelog = false", "omit_from_changelog = true");

    run_cmd!(echo $cog_toml > cog.toml;)?;

    let changelog = Command::cargo_bin("cog")?
        .arg("changelog")
        // Assert
        .assert()
        .success();

    let changelog = changelog.get_output();
    let changelog = String::from_utf8_lossy(&changelog.stdout);

    assert!(
        !changelog.as_ref().contains("#### Work In Progress"),
        "Expected \"Work In Progress\" entry to be omitted from changelog but got:\n\n{}",
        changelog.as_ref()
    );

    Ok(())
}
