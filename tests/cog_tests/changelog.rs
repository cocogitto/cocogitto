use anyhow::Result;
use assert_cmd::Command;
use chrono::Utc;
use cmd_lib::{run_cmd, run_fun};
use indoc::{formatdoc, indoc};
use itertools::Itertools;
use pretty_assertions::assert_eq;
use sealed_test::prelude::*;
use std::fs;
use std::path::PathBuf;

use cocogitto::settings::Settings;

use crate::helpers::*;

macro_rules! assert_doc_eq {
    ($changelog:expr, $doc:literal $($arg:tt)*) => {
        assert_eq!(
            $changelog.split('\n').map(|line| line.trim()).join("\n"),
            formatdoc!($doc $($arg)*).split('\n').map(|line| line.trim()).join("\n")
        )
    };
}

#[test]
fn get_changelog_range() -> Result<()> {
    // Act
    let changelog = Command::new(assert_cmd::cargo_bin!("cog"))
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

    assert_doc_eq!(
        changelog.as_ref(),
        "## 0.32.3 - 2020-09-30
        #### Bug Fixes
        - fix openssl missing in CD - (1c0d2e9) - *oknozor*
        #### Documentation
        - tag, conventional commit and license badges to readme - (da6f63d) - *oknozor*

        - - -

        ## 0.32.2 - 2020-09-30
        #### Bug Fixes
        - (**cd**) bump setup-rust-action to v1.3.3 - (5350b11) - *oknozor*
        #### Documentation
        - add corrections to README - (9a33516) - *oknozor*

        - - -

        ## 0.32.1 - 2020-09-30
        #### Features
        - move check edit to dedicated subcommand and fix rebase - (fc74207) - *oknozor*
        - remove config commit on init existing repo - (1028d0b) - *oknozor*
        #### Bug Fixes
        - (**cd**) fix ci cross build command bin args - (7f04a98) - *oknozor*
        #### Documentation
        - rewritte readme completely - (b223f7b) - *oknozor*
        #### Refactoring
        - change config name to cog.toml - (d4aa61b) - *oknozor*


        ",
    );
    Ok(())
}

#[sealed_test]
fn get_changelog_from_untagged_repo() -> Result<()> {
    // Arrange
    git_init()?;
    let init = git_commit("chore: init")?;
    let commit_two = git_commit("feat(taef): feature")?;
    let commit_three = git_commit("fix: bug fix")?;

    // Act
    let changelog = Command::new(assert_cmd::cargo_bin!("cog"))
        .arg("changelog")
        // Assert
        .assert()
        .success();

    let changelog = changelog.get_output();
    let changelog = &changelog.stdout;
    let changelog = String::from_utf8_lossy(changelog.as_slice());

    assert_doc_eq!(
        changelog.as_ref(),
        "## Unreleased ({init}..{commit_three})
        #### Features
        - (**taef**) feature - ({commit_two}) - Tom
        #### Bug Fixes
        - bug fix - ({commit_three}) - Tom
        #### Miscellaneous Chores
        - init - ({init}) - Tom


        ",
        init = &init[0..7],
        commit_two = &commit_two[0..7],
        commit_three = &commit_three[0..7]
    );
    Ok(())
}

#[sealed_test]
fn get_changelog_from_tagged_repo() -> Result<()> {
    // Arrange
    git_init()?;
    let init = git_commit("chore: init")?;
    let commit_one = git_commit("feat(taef): feature")?;
    git_tag("1.0.0")?;
    let commit_two = git_commit("fix: bug fix")?;

    // Act
    let changelog = Command::new(assert_cmd::cargo_bin!("cog"))
        .arg("changelog")
        // Assert
        .assert()
        .success();

    let changelog = changelog.get_output();
    let changelog = &changelog.stdout;
    let changelog = String::from_utf8_lossy(changelog.as_slice());
    let today = Utc::now().date_naive().to_string();

    assert_doc_eq!(
        changelog.as_ref(),
        "## Unreleased ({commit_two}..{commit_two})
        #### Bug Fixes
        - bug fix - ({commit_two}) - Tom

        - - -

        ## 1.0.0 - {today}
        #### Features
        - (**taef**) feature - ({commit_one}) - Tom
        #### Miscellaneous Chores
        - init - ({init}) - Tom


        ",
        init = &init[0..7],
        commit_one = &commit_one[0..7],
        commit_two = &commit_two[0..7],
        today = today
    );
    Ok(())
}

#[sealed_test]
fn get_changelog_at_tag() -> Result<()> {
    // Arrange
    git_init()?;
    let init = git_commit("chore: init")?;
    let commit_one = git_commit("feat(taef): feature")?;
    let commit_two = git_commit("feat: feature 2")?;
    git_tag("1.0.0")?;
    let _ = git_commit("fix: bug fix")?;

    // Act
    let changelog = Command::new(assert_cmd::cargo_bin!("cog"))
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

    assert_doc_eq!(
        changelog.as_ref(),
        "## 1.0.0 - {today}
        #### Features
        - (**taef**) feature - ({commit_one}) - Tom
        - feature 2 - ({commit_two}) - Tom
        #### Miscellaneous Chores
        - init - ({init}) - Tom


        ",
        today = today,
        init = &init[0..7],
        commit_one = &commit_one[0..7],
        commit_two = &commit_two[0..7]
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
    fs::write("cog.toml", settings?)?;

    git_init()?;
    let init = git_commit("chore: init")?;
    let commit_one = git_commit("feat: feature 1")?;
    git_tag("v1.0.0")?;
    let commit_two = git_commit("fix: bug fix 1")?;

    // Act
    let changelog = Command::new(assert_cmd::cargo_bin!("cog"))
        .arg("changelog")
        // Assert
        .assert()
        .success();

    let changelog = changelog.get_output();
    let changelog = &changelog.stdout;
    let changelog = String::from_utf8_lossy(changelog.as_slice());
    let today = Utc::now().date_naive();

    assert_doc_eq!(
        changelog.as_ref(),
        "## Unreleased ({commit_two}..{commit_two})
        #### Bug Fixes
        - bug fix 1 - ({commit_two}) - Tom

        - - -

        ## v1.0.0 - {today}
        #### Features
        - feature 1 - ({commit_one}) - Tom
        #### Miscellaneous Chores
        - init - ({init}) - Tom


        ",
        today = today,
        init = &init[0..7],
        commit_one = &commit_one[0..7],
        commit_two = &commit_two[0..7]
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
    let changelog = Command::new(assert_cmd::cargo_bin!("cog"))
        .arg("changelog")
        .arg("--at")
        .arg("v2.0.0")
        // Assert
        .assert()
        .success();

    let changelog = changelog.get_output();
    let changelog = String::from_utf8_lossy(&changelog.stdout);
    let today = Utc::now().date_naive();

    assert_doc_eq!(
        changelog.as_ref(),
        "## v2.0.0 - {today}
        #### Features
        - feature 1 - ({commit_two}) - Tom
        #### Bug Fixes
        - bug fix 1 - ({commit_three}) - Tom
        #### Miscellaneous Chores
        - (**version**) v2.0.0 - ({commit_four}) - Tom


        ",
        today = today,
        commit_two = &commit_two[0..7],
        commit_three = &commit_three[0..7],
        commit_four = &commit_four[0..7]
    );
    Ok(())
}

#[sealed_test]
fn get_changelog_from_tag_to_tagged_head() -> Result<()> {
    // Arrange
    git_init()?;
    let init = git_commit("chore: init")?;
    let commit_one = git_commit("feat: start")?;
    let commit_two = git_commit("feat: feature 1")?;
    git_tag("1.0.0")?;
    let commit_three = git_commit("feat: feature 2")?;
    let commit_four = git_commit("fix: bug fix 1")?;
    let commit_five = git_commit("chore(version): 2.0.0")?;
    git_tag("2.0.0")?;

    // Act
    let changelog = Command::new(assert_cmd::cargo_bin!("cog"))
        .arg("changelog")
        // Assert
        .assert()
        .success();

    let changelog = changelog.get_output();
    let changelog = String::from_utf8_lossy(&changelog.stdout);
    let today = Utc::now().date_naive();
    assert_doc_eq!(
        changelog.as_ref(),
        "## 2.0.0 - {today}
        #### Features
        - feature 2 - ({commit_three}) - Tom
        #### Bug Fixes
        - bug fix 1 - ({commit_four}) - Tom
        #### Miscellaneous Chores
        - (**version**) 2.0.0 - ({commit_five}) - Tom

        - - -

        ## 1.0.0 - {today}
        #### Features
        - feature 1 - ({commit_two}) - Tom
        - start - ({commit_one}) - Tom
        #### Miscellaneous Chores
        - init - ({init}) - Tom


        ",
        today = today,
        init = &init[0..7],
        commit_one = &commit_one[0..7],
        commit_two = &commit_two[0..7],
        commit_three = &commit_three[0..7],
        commit_four = &commit_four[0..7],
        commit_five = &commit_five[0..7],
    );

    Ok(())
}

#[sealed_test]
fn get_changelog_is_unaffected_by_disable_changelog() -> Result<()> {
    // Arrange
    git_init()?;

    let cog_toml = indoc!("disable_changelog = true");

    run_cmd!(echo $cog_toml > cog.toml;)?;
    let init = git_commit("chore: init")?;
    let commit_one = git_commit("feat: start")?;
    let commit_two = git_commit("feat: feature 1")?;
    git_tag("1.0.0")?;

    // Act
    let changelog = Command::new(assert_cmd::cargo_bin!("cog"))
        .arg("changelog")
        // Assert
        .assert()
        .success();

    let changelog = changelog.get_output();
    let changelog = String::from_utf8_lossy(&changelog.stdout);
    let today = Utc::now().date_naive();

    assert_doc_eq!(
        changelog.as_ref(),
        "## 1.0.0 - {today}
        #### Features
        - feature 1 - ({commit_two}) - Tom
        - start - ({commit_one}) - Tom
        #### Miscellaneous Chores
        - init - ({init}) - Tom


        ",
        today = today,
        init = &init[0..7],
        commit_one = &commit_one[0..7],
        commit_two = &commit_two[0..7],
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
    let init = git_commit("chore: init")?;
    let commit_one = git_commit("feat(scope1): start")?;
    let commit_two = git_commit("feat: feature 1")?;
    git_tag("1.0.0")?;
    let commit_three = git_commit("feat: feature 2")?;
    let commit_four = git_commit("fix: bug fix 1")?;
    let commit_five = git_commit("chore(version): 2.0.0")?;
    git_tag("2.0.0")?;

    // Act
    let changelog = Command::new(assert_cmd::cargo_bin!("cog"))
        .arg("changelog")
        .arg("-t")
        .arg(template)
        // Assert
        .assert()
        .success();

    let changelog = changelog.get_output();
    let changelog = String::from_utf8_lossy(&changelog.stdout);
    let today = Utc::now().date_naive();

    assert_doc_eq!(
        changelog.as_ref(),
        "## [2.0.0](https://github.com/test/test/compare/1.0.0..2.0.0) - {today}
        #### Bug Fixes
        -  bug fix 1 - ([{commit_four_short}](https://github.com/test/test/commit/{commit_four})) - Tom
        #### Features
        -  feature 2 - ([{commit_three_short}](https://github.com/test/test/commit/{commit_three})) - Tom
        #### Miscellaneous Chores
        - **(version)** 2.0.0 - ([{commit_five_short}](https://github.com/test/test/commit/{commit_five})) - Tom

        - - -

        ## [1.0.0](https://github.com/test/test/compare/{init}..1.0.0) - {today}
        #### Features
        -  feature 1 - ([{commit_two_short}](https://github.com/test/test/commit/{commit_two})) - Tom
        - **(scope1)** start - ([{commit_one_short}](https://github.com/test/test/commit/{commit_one})) - Tom
        #### Miscellaneous Chores
        -  init - ([{init_commit}](https://github.com/test/test/commit/{init})) - Tom


        ",
        today = today,
        init = &init,
        init_commit = &init[0..7],
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
    );
    Ok(())
}

#[sealed_test]
fn should_ignore_merge_commit() -> Result<()> {
    // Arrange
    git_init()?;

    run_cmd!(git config merge.ff false;)?;
    run_cmd!(echo "ignore_merge_commits = true" > cog.toml;)?;
    git_commit("chore: init")?;
    git_commit("feat: first commit")?;
    run_cmd!(git checkout -b branch1;)?;
    git_commit("fix: fon branch 2")?;
    run_cmd!(
        git checkout master;
        git merge branch1;
    )?;

    // Act
    Command::new(assert_cmd::cargo_bin!("cog"))
        .arg("changelog")
        // Assert
        .assert()
        .success();

    Ok(())
}

#[sealed_test]
fn should_not_ignore_fixup_commit() -> Result<()> {
    // Arrange
    git_init()?;
    run_cmd!(echo "ignore_fixup_commits = false" > cog.toml;)?;
    git_commit("chore: init")?;
    let sha = git_commit("feat: first commit")?;
    run_cmd!(git checkout -b branch1;)?;
    git_commit("fix: fon branch 2")?;
    run_cmd!(
        echo toto > titi;
        git add .;
        git commit --fixup $sha;
    )?;

    // Act
    Command::new(assert_cmd::cargo_bin!("cog"))
        .arg("changelog")
        // Assert
        .assert()
        .success()
        .stderr(predicates::str::contains("1 | fixup! feat: first commit"));

    Ok(())
}

#[sealed_test]
fn should_ignore_fixup_commit() -> Result<()> {
    // Arrange
    git_init()?;
    git_commit("chore: init")?;
    let sha = git_commit("feat: first commit")?;
    run_cmd!(git checkout -b branch1;)?;
    git_commit("fix: fon branch 2")?;
    run_cmd!(
        echo toto > titi;
        git add .;
        git commit --fixup $sha;
    )?;

    // Act
    Command::new(assert_cmd::cargo_bin!("cog"))
        .arg("changelog")
        // Assert
        .assert()
        .success();

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

    let changelog = Command::new(assert_cmd::cargo_bin!("cog"))
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

    let changelog = Command::new(assert_cmd::cargo_bin!("cog"))
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

#[sealed_test]
fn should_get_global_changelog() -> anyhow::Result<()> {
    // Arrange
    git_init()?;
    mkdir(&["packages/pkg1", "packages/pkg1"])?;

    let cog = indoc!(
        r#"[changelog]
        remote = "github.com"
        repository = "test"
        owner = "test"

        [packages]
        pkg1 = { path = "packages/pkg1" }
        pkg2 = { path = "packages/pkg2" }
        "#
    );
    git_add(cog, "cog.toml")?;
    git_commit("chore: init")?;
    git_add("pkg1", "packages/pkg1/README.md")?;
    let _ = git_commit("feat: package 1 feat")?;
    git_add("pkg2", "packages/pkg2/README.md")?;
    let _ = git_commit("feat: package 2 fix")?;

    Command::new(assert_cmd::cargo_bin!("cog"))
        .arg("changelog")
        .arg("--template")
        .arg("monorepo_default")
        .assert()
        .success();

    Ok(())
}

/// Test that the `order` configuration
/// directive is honored if/when it is specified for
/// a given commit type and used in a sort.
#[sealed_test]
fn order_from_changelog() -> Result<()> {
    // Arrange
    git_init()?;

    let cog_toml = indoc!(
        "[changelog]
        remote = \"github.com\"
        repository = \"test\"
        owner = \"test\"

        [commit_types]
        feat = { changelog_title = \"Features\", order = 1 }
        fix = { changelog_title = \"Bug Fixes\", order = 2 }
        chore = { changelog_title = \"Miscellaneous Chores\", order = 3 }"
    );

    let template = indoc!(
        "{% for order, ordered_commits in commits | sort(attribute=\"type_order\") | group_by(attribute=\"type_order\")-%}
        #### {{ ordered_commits[0].type }}
        {% for commit in ordered_commits -%}
        - {{ commit.summary }}
        {% endfor %}
        {% endfor -%}"
    );

    let _setup = (
        run_cmd!(echo $cog_toml > cog.toml;)?,
        fs::read_to_string("cog.toml")?,
        run_cmd!(echo $template > template.md;)?,
        fs::read_to_string("template.md")?,
        git_commit("chore: init")?,
        git_commit("feat(scope1): start")?,
        git_commit("feat: feature 1")?,
        git_commit("feat: feature 2")?,
        git_commit("fix: bug fix 1")?,
    );

    let changelog = Command::new(assert_cmd::cargo_bin!("cog"))
        .arg("changelog")
        .arg("-t")
        .arg("template.md")
        // Assert
        .assert()
        .success();

    let changelog = changelog.get_output();
    let changelog = String::from_utf8_lossy(&changelog.stdout);

    assert_eq!(
        changelog.as_ref(),
        formatdoc!(
            "#### Features
            - feature 2
            - feature 1
            - start

            #### Bug Fixes
            - bug fix 1

            #### Miscellaneous Chores
            - init



            "
        )
    );

    let cog_toml = cog_toml.replace("order = 1", "order = 5");

    run_cmd!(echo $cog_toml > cog.toml;)?;

    let changelog = Command::new(assert_cmd::cargo_bin!("cog"))
        .arg("changelog")
        .arg("-t")
        .arg("template.md")
        // Assert
        .assert()
        .success();

    let changelog = changelog.get_output();
    let changelog = String::from_utf8_lossy(&changelog.stdout);

    assert_eq!(
        changelog.as_ref(),
        formatdoc!(
            "#### Bug Fixes
            - bug fix 1

            #### Miscellaneous Chores
            - init

            #### Features
            - feature 2
            - feature 1
            - start



            "
        )
    );

    Ok(())
}

#[sealed_test]
/// Test that the `order` configuration
/// directive is honored if/when it is specified for
/// a given commit type and used in a sort.
fn group_by_type() -> Result<()> {
    // Arrange
    git_init()?;

    let cog_toml = indoc!(
        "[changelog]
        remote = \"github.com\"
        repository = \"test\"
        owner = \"test\"

        [commit_types]
        feat = { changelog_title = \"Features\", order = 1 }
        fix = { changelog_title = \"Bug Fixes\", order = 2 }
        chore = { changelog_title = \"Miscellaneous Chores\", order = 3 }"
    );

    let template = indoc!(
        "{% for value in commits | group_by_type -%}
        #### {{ value.0 }}
        {% for commit in value.1 -%}
        - {{ commit.summary }}
        {% endfor %}
        {% endfor -%}"
    );

    let _setup = (
        run_cmd!(echo $cog_toml > cog.toml;)?,
        fs::read_to_string("cog.toml")?,
        run_cmd!(echo $template > template.md;)?,
        fs::read_to_string("template.md")?,
        git_commit("chore: init")?,
        git_commit("feat(scope1): start")?,
        git_commit("feat: feature 1")?,
        git_commit("feat: feature 2")?,
        git_commit("fix: bug fix 1")?,
    );

    let changelog = Command::new(assert_cmd::cargo_bin!("cog"))
        .arg("changelog")
        .arg("-t")
        .arg("template.md")
        // Assert
        .assert()
        .success();

    let changelog = changelog.get_output();
    let changelog = String::from_utf8_lossy(&changelog.stdout);

    assert_eq!(
        changelog.as_ref(),
        formatdoc!(
            "#### Features
            - feature 2
            - feature 1
            - start

            #### Bug Fixes
            - bug fix 1

            #### Miscellaneous Chores
            - init



            "
        )
    );

    let cog_toml = cog_toml.replace("order = 1", "order = 5");

    run_cmd!(echo $cog_toml > cog.toml;)?;

    let changelog = Command::new(assert_cmd::cargo_bin!("cog"))
        .arg("changelog")
        .arg("-t")
        .arg("template.md")
        // Assert
        .assert()
        .success();

    let changelog = changelog.get_output();
    let changelog = String::from_utf8_lossy(&changelog.stdout);

    assert_eq!(
        changelog.as_ref(),
        formatdoc!(
            "#### Bug Fixes
            - bug fix 1

            #### Miscellaneous Chores
            - init

            #### Features
            - feature 2
            - feature 1
            - start



            "
        )
    );

    Ok(())
}

#[sealed_test]
fn changelog_from_commit_range_should_be_the_same_as_changelog_from_tag_range() -> Result<()> {
    // Arrange
    git_init()?;

    git_commit("feat: feature 1")?;
    let sha_0_1 = git_commit("feat: feature 2")?;
    let _ = git_tag("0.1.0");
    git_commit("feat: feature 3")?;

    Command::new(assert_cmd::cargo_bin!("cog"))
        .arg("bump")
        .arg("--auto")
        .assert()
        .success();

    let sha_0_2 = run_fun!(git log --format=%H -n 1)?;

    // Act
    let changelog_from_commit_range = Command::new(assert_cmd::cargo_bin!("cog"))
        .arg("changelog")
        .arg(&format!("{sha_0_1}..{sha_0_2}"))
        .assert()
        .success();

    let changelog_from_commit_range =
        String::from_utf8_lossy(&changelog_from_commit_range.get_output().stdout);

    let changelog_from_tag_range = Command::new(assert_cmd::cargo_bin!("cog"))
        .arg("changelog")
        .arg(&"0.1.0..0.2.0".to_string())
        .assert()
        .success();

    let changelog_from_tag_range =
        String::from_utf8_lossy(&changelog_from_tag_range.get_output().stdout);

    // Assert
    pretty_assertions::assert_eq!(changelog_from_commit_range, changelog_from_tag_range);

    Ok(())
}

#[sealed_test]
fn chore_commit_with_breaking_change_should_be_displayed_in_changelog() -> Result<()> {
    // Arrange
    git_init()?;
    let cog_toml = indoc!(
        "[commit_types]
        chore = { omit_from_changelog = true }"
    );
    git_add(cog_toml, "cog.toml")?;
    let _ = git_commit("chore: init")?;
    let commit_one = git_commit("feat: feature")?;
    git_tag("1.0.0")?;
    let commit_two = git_commit("chore!: breaking change in chore")?;

    // Act
    let changelog = Command::new(assert_cmd::cargo_bin!("cog"))
        .arg("changelog")
        // Assert
        .assert()
        .success();

    let changelog = changelog.get_output();
    let changelog = &changelog.stdout;
    let changelog = String::from_utf8_lossy(changelog.as_slice());
    let today = Utc::now().date_naive().to_string();

    assert_doc_eq!(
        changelog.as_ref(),
        r#"## Unreleased ({commit_two}..{commit_two})
        #### Miscellaneous Chores
        - <span style="background-color: #d73a49; color: white; padding: 2px 6px; border-radius: 3px; font-weight: bold; font-size: 0.85em;">BREAKING</span>breaking change in chore - ({commit_two}) - Tom

        - - -

        ## 1.0.0 - {today}
        #### Features
        - feature - ({commit_one}) - Tom


        "#,
        commit_one = &commit_one[0..7],
        commit_two = &commit_two[0..7],
        today = today
    );
    Ok(())
}

#[sealed_test]
fn changelog_monorepo_multi_versions() -> Result<()> {
    // Arrange
    git_init()?;
    let today = Utc::now().date_naive();
    run_cmd!(mkdir -p zz)?;

    let cog = indoc!(
        r#"
        tag_prefix = "v"

        [packages]
        zz = { path = "zz" }
        "#
    );

    git_add(cog, "cog.toml")?;
    let sha_1 = git_commit_short("chore: init")?;
    git_add("", "zz/lib.rs")?;
    git_add("", "main.rs")?;
    let sha_2 = git_commit_short("feat: add implementation")?;

    let sha_3 = cog_bump_auto()?;

    git_add(".", "zz/lib.rs")?;
    git_add(".", "main.rs")?;
    let sha_4 = git_commit_short("fix: combat nasty bug")?;

    let sha_5 = cog_bump_auto()?;

    // Act
    let result = Command::new(assert_cmd::cargo_bin!("cog"))
        .arg("changelog")
        // ignore package bumps until they are fixed
        .arg("-t")
        .arg("default")
        // Assert
        .assert()
        .success();

    let changelog = result.get_output();
    let changelog = &changelog.stdout;
    let changelog = String::from_utf8_lossy(changelog.as_slice());

    assert_eq!(
        changelog,
        format!(
            "## v0.1.1 - {today}
#### Bug Fixes
- combat nasty bug - ({sha_4}) - Tom
#### Miscellaneous Chores
- (**version**) v0.1.1 - ({sha_5}) - Tom

- - -

## v0.1.0 - {today}
#### Features
- add implementation - ({sha_2}) - Tom
#### Miscellaneous Chores
- (**version**) v0.1.0 - ({sha_3}) - Tom
- init - ({sha_1}) - Tom


",
            today = today,
            sha_1 = sha_1,
            sha_2 = sha_2,
            sha_3 = sha_3,
            sha_4 = sha_4,
            sha_5 = sha_5,
        )
    );

    Ok(())
}

#[sealed_test]
fn unified_changelog() -> Result<()> {
    // Arrange
    git_init()?;
    let today = Utc::now().date_naive();
    let cog = indoc!(
        r#"
        [packages]
        a = { path = "a" }
        b = { path = "b" }
        "#
    );
    git_add(cog, "cog.toml")?;
    let sha_1 = git_commit_short("chore: init")?;
    git_add(".", "a/feat")?;
    let sha_2 = git_commit_short("feat(a): implement a")?;
    git_add(".", "b/feat")?;
    let sha_3 = git_commit_short("feat(b): implement b")?;
    git_add(".", "a/fix")?;
    git_add(".", "b/fix")?;
    let sha_4 = git_commit_short("fix: everything")?;
    git_tag("1.0.0")?;
    git_tag("a-1.0.0")?;
    git_tag("b-1.0.0")?;

    // Act
    let result = Command::new(assert_cmd::cargo_bin!("cog"))
        .arg("changelog")
        .arg("--unified")
        // Assert
        .assert()
        .success();

    let changelog = String::from_utf8_lossy(&result.get_output().stdout);

    assert_eq!(
        changelog,
        formatdoc! {
            r#"
            ## 1.0.0 - {today}
            ### Package updates
            - a bumped to a-1.0.0
            - b bumped to b-1.0.0
            ### All changes
            #### Features
            - (**a**) implement a - ({sha_2}) - Tom
            - (**b**) implement b - ({sha_3}) - Tom
            #### Bug Fixes
            - everything - ({sha_4}) - Tom
            #### Miscellaneous Chores
            - init - ({sha_1}) - Tom


            "#,
            today = today,
            sha_1 = sha_1,
            sha_2 = sha_2,
            sha_3 = sha_3,
            sha_4 = sha_4,
        }
    );

    Ok(())
}

#[sealed_test]
fn monorepo_changelog_default_template() -> Result<()> {
    // Arrange
    git_init()?;
    let today = Utc::now().date_naive();
    let cog = indoc!(
        r#"
        [packages]
        pkg = { path = "pkg" }
        "#
    );
    git_add(cog, "cog.toml")?;
    let sha_1 = git_commit_short("chore: init")?;
    git_add(".", "pkg/feat")?;
    git_commit("feat(pkg): implement pkg")?;
    git_add(".", "global/fix")?;
    git_add(".", "pkg/fix")?;
    let sha_2 = git_commit_short("fix: everything")?;
    git_tag("0.1.0")?;
    git_tag("pkg-0.1.0")?;

    // Act
    let result = Command::new(assert_cmd::cargo_bin!("cog"))
        .arg("changelog")
        // Assert
        .assert()
        .success();

    let changelog = String::from_utf8_lossy(&result.get_output().stdout);

    assert_eq!(
        changelog,
        formatdoc! {
            r#"
            ## 0.1.0 - {today}
            ### Package updates
            - pkg bumped to pkg-0.1.0
            ### Global changes
            #### Bug Fixes
            - everything - ({sha_2}) - Tom
            #### Miscellaneous Chores
            - init - ({sha_1}) - Tom


            "#,
            today = today,
            sha_1 = sha_1,
        }
    );

    Ok(())
}

#[test]
fn should_render_github_changelog() -> anyhow::Result<()> {
    let result = Command::new(assert_cmd::cargo_bin!("cog"))
        .arg("changelog")
        .arg("-t")
        .arg("github")
        .arg("--at")
        .arg("6.4.0")
        // Assert
        .assert()
        .success();

    let changelog = String::from_utf8_lossy(&result.get_output().stdout);

    assert_doc_eq!(
        changelog,
        r#"## [6.4.0](https://github.com/cocogitto/cocogitto/compare/c8e46aa2b28ff1bad94852b1abbec568adbff5aa..6.4.0) - 2025-10-18
            #### Features
            - add breaking changes badge in all built-in templates - ([1b7f965](https://github.com/cocogitto/cocogitto/commit/1b7f96576e98e89838ba7608904fad65983a9e9d)) - [@oknozor](https://github.com/oknozor)
            - apply sort order in template - ([306da76](https://github.com/cocogitto/cocogitto/commit/306da76e540b6bc6736af11753b916806cd8c9e0)) - [@oknozor](https://github.com/oknozor)
            - Allow sort order to be specified for commit types - ([1949b50](https://github.com/cocogitto/cocogitto/commit/1949b505ba47367145f8f55529c9d798e0745568)) - [@mofojed](https://github.com/mofojed)
            - only read tags reachable from HEAD - ([aecd76b](https://github.com/cocogitto/cocogitto/commit/aecd76bd47a31644bb89158aa17f7246a8740765)) - [@ba-lindner](https://github.com/ba-lindner)
            - allow specifying config path via command line (#466) - ([b5443e8](https://github.com/cocogitto/cocogitto/commit/b5443e8d8da0a4674ababbc0e031aedf0b795a85)) - [@TECHNOFAB11](https://github.com/TECHNOFAB11)
            - resolve tilde in the signingkey to home (#460) - ([daa983b](https://github.com/cocogitto/cocogitto/commit/daa983b1b0507a326099fcbe50b1a80bdab8a296)) - [@kristof-mattei](https://github.com/kristof-mattei)
            #### Bug Fixes
            - respect disable_changelog setting for package changelogs - ([d6ac8bf](https://github.com/cocogitto/cocogitto/commit/d6ac8bf253b1f3dc6082af91c82039c13d5acc1a)) - [@ba-lindner](https://github.com/ba-lindner)
            - allow packages without bump in changelog command - ([71dc621](https://github.com/cocogitto/cocogitto/commit/71dc6218bcba2c96bd622deefdf0a4136aa5c84f)) - [@ba-lindner](https://github.com/ba-lindner)
            - don't reverse commit list - ([bec8de8](https://github.com/cocogitto/cocogitto/commit/bec8de8b1ef6618880fc556d7b53083f324a5432)) - [@ba-lindner](https://github.com/ba-lindner)
            - save multiple tags per commit - ([cc0e64d](https://github.com/cocogitto/cocogitto/commit/cc0e64d2c1e075ac9b782258783212b4d7917892)) - [@ba-lindner](https://github.com/ba-lindner)
            - cog changelog failing on single commit - ([a155a9d](https://github.com/cocogitto/cocogitto/commit/a155a9d3e331517398efefab0dbd0a426ff38f1b)) - [@oknozor](https://github.com/oknozor)
            - set explicit binstall pkg-url, bin-dir and pkg-fmt - ([97fe60e](https://github.com/cocogitto/cocogitto/commit/97fe60ed25e6458b842afb52bc1ac30fdcef5c11)) - [@kristof-mattei](https://github.com/kristof-mattei)
            - fix github pages build - ([c8e46aa](https://github.com/cocogitto/cocogitto/commit/c8e46aa2b28ff1bad94852b1abbec568adbff5aa)) - [@oknozor](https://github.com/oknozor)
            #### Documentation
            - (**typo**) fix various doc typo (#458) - ([707a80a](https://github.com/cocogitto/cocogitto/commit/707a80acafac209d20e20fef19ecb3b539b791ef)) - [@JxJxxJxJ](https://github.com/JxJxxJxJ)
            - add commit order doc - ([7e4e6ef](https://github.com/cocogitto/cocogitto/commit/7e4e6ef80503f71748e369e8b92e4c8c135ad68a)) - [@oknozor](https://github.com/oknozor)
            #### Refactoring
            - remove field target of Tag - ([3a4472b](https://github.com/cocogitto/cocogitto/commit/3a4472beacf7679b6cbd6a8903693822b27297ea)) - [@ba-lindner](https://github.com/ba-lindner)


            "#
    );

    Ok(())
}
