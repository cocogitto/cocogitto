use std::process::Command;

use crate::helpers::*;

use anyhow::Result;
use assert_cmd::prelude::*;
use cmd_lib::run_cmd;
use indoc::indoc;
use sealed_test::prelude::*;

#[sealed_test]
fn verify_ok() -> Result<()> {
    // Arrange
    git_init()?;
    let message = "chore: a commit message";
    let expected = indoc!(
        "a commit message (not committed) - now
            \tAuthor: Tom
            \tType: chore
            \tScope: none

            ",
    );

    // Act
    Command::new(assert_cmd::cargo_bin!("cog"))
        .arg("verify")
        .arg(message)
        // Assert
        .assert()
        .success()
        .stderr(expected);

    Ok(())
}

#[sealed_test]
fn verify_with_scope() -> Result<()> {
    // Arrange
    git_init()?;
    let message = "feat(feature): a commit message";
    let expected = indoc!(
        "a commit message (not committed) - now
            \tAuthor: Tom
            \tType: feat
            \tScope: feature

            ",
    );

    // Act
    Command::new(assert_cmd::cargo_bin!("cog"))
        .arg("verify")
        .arg(message)
        // Assert
        .assert()
        .success()
        .stderr(expected);
    Ok(())
}

#[sealed_test]
fn verify_with_valid_scopes_setting_ok() -> Result<()> {
    // Arrange
    git_init()?;
    let settings = r#"scopes = ["valid"]"#;
    run_cmd!(
        echo $settings > cog.toml;
    )?;

    let message = "feat(valid): a commit message";

    let expected = indoc!(
        "a commit message (not committed) - now
            \tAuthor: Tom
            \tType: feat
            \tScope: valid

            ",
    );

    // Act
    Command::new(assert_cmd::cargo_bin!("cog"))
        .arg("verify")
        .arg(message)
        // Assert
        .assert()
        .success()
        .stderr(expected);
    Ok(())
}

#[sealed_test]
fn verify_with_invalid_scopes_setting_fails() -> Result<()> {
    // Arrange
    git_init()?;
    let settings = r#"scopes = ["valid"]"#;
    run_cmd!(
        echo $settings > cog.toml;
    )?;

    let message = "feat(invalid): a commit message";

    // Act
    Command::new(assert_cmd::cargo_bin!("cog"))
        .arg("verify")
        .arg(message)
        // Assert
        .assert()
        .failure();
    Ok(())
}

#[sealed_test]
fn verify_with_empty_scopes_setting_fails() -> Result<()> {
    // Arrange
    git_init()?;
    let settings = r#"scopes = []"#;
    run_cmd!(
        echo $settings > cog.toml;
    )?;

    let message = "feat(valid): a commit message";

    // Act
    Command::new(assert_cmd::cargo_bin!("cog"))
        .arg("verify")
        .arg(message)
        // Assert
        .assert()
        .failure();
    Ok(())
}

#[test]
fn verify_fails() -> Result<()> {
    // Arrange
    let message = "invalid message";

    // Act
    Command::new(assert_cmd::cargo_bin!("cog"))
        .arg("verify")
        .arg(message)
        // Assert
        .assert()
        .failure();

    Ok(())
}

#[test]
fn verify_with_unknown_commit_type_fails() -> Result<()> {
    // Arrange
    let message = "toto: la totomobile";

    // Act
    Command::new(assert_cmd::cargo_bin!("cog"))
        .arg("verify")
        .arg(message)
        // Assert
        .assert()
        .failure();

    Ok(())
}

#[sealed_test]
fn should_not_ignore_merge_commit_by_default() -> Result<()> {
    // Arrange
    let message = "Merge toto into titi";

    // Act
    Command::new(assert_cmd::cargo_bin!("cog"))
        .arg("verify")
        .arg(message)
        // Assert
        .assert()
        .failure();

    Ok(())
}

#[test]
fn should_ignore_merge_commit_with_ignore_flag() -> Result<()> {
    // Arrange
    let message = "Merge toto into titi";

    // Act
    Command::new(assert_cmd::cargo_bin!("cog"))
        .arg("verify")
        .arg("--ignore-merge-commits")
        .arg(message)
        // Assert
        .assert()
        .success();

    Ok(())
}

#[sealed_test]
fn should_ignore_merge_commit_via_config() -> Result<()> {
    // Arrange
    git_init()?;
    let settings = r#"ignore_merge_commits = true"#;

    run_cmd!(
        echo $settings > cog.toml;
        git add .;
        git commit -m "feat: cog.toml config"
    )?;

    let message = "Merge toto into titi";

    // Act
    Command::new(assert_cmd::cargo_bin!("cog"))
        .arg("verify")
        .arg(message)
        // Assert
        .assert()
        .success();

    Ok(())
}

#[test]
fn should_not_ignore_fixup_commit_by_default() -> Result<()> {
    // Arrange + Act
    Command::new(assert_cmd::cargo_bin!("cog"))
        .arg("verify")
        .arg("fixup! this commit is wrong")
        // Assert
        .assert()
        .failure();

    Ok(())
}

#[sealed_test(files = ["tests/assets/commit_message.txt"])]
fn verify_file_ok() -> Result<()> {
    // Arrange
    git_init()?;
    let expected = indoc!(
        "a commit message (not committed) - now
            \tAuthor: Tom
            \tType: chore
            \tScope: none

            ",
    );

    // Act
    Command::new(assert_cmd::cargo_bin!("cog"))
        .arg("verify")
        .arg("--file")
        .arg("commit_message.txt")
        // Assert
        .assert()
        .success()
        .stderr(expected);

    Ok(())
}

#[test]
fn verify_with_not_existing_file_fails() -> Result<()> {
    // Act
    Command::new(assert_cmd::cargo_bin!("cog"))
        .arg("verify")
        .arg("--file")
        .arg("not_existing_file.txt")
        // Assert
        .assert()
        .failure();

    Ok(())
}

#[cfg(target_family = "unix")]
#[sealed_test(files = ["tests/assets/commit_message.txt"])]
fn verify_with_unreadable_file_fails() -> Result<()> {
    use std::fs;
    use std::os::unix::fs::PermissionsExt;

    let file_name = "commit_message.txt";

    // Arrange
    let mut perms = fs::metadata(file_name)?.permissions();
    perms.set_mode(0o333); // write-only
    fs::set_permissions(file_name, perms)?;

    // Act
    Command::new(assert_cmd::cargo_bin!("cog"))
        .arg("verify")
        .arg("--file")
        .arg(file_name)
        // Assert
        .assert()
        .failure();

    Ok(())
}

#[sealed_test]
fn verify_stdin_ok() -> Result<()> {
    use std::io::Write;
    use std::process::Stdio;

    // Arrange
    git_init()?;
    let message = "feat(grid): Add lightcycle battles to the grid";
    let expected = indoc!(
        "Add lightcycle battles to the grid (not committed) - now
            \tAuthor: Tom
            \tType: feat
            \tScope: grid

            ",
    );

    // Act
    let mut cmd = Command::new(assert_cmd::cargo_bin!("cog"));
    cmd.arg("verify")
        .arg("--file")
        .arg("-")
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped());

    let mut child = cmd.spawn()?;

    // Write to stdin
    {
        let stdin = child.stdin.as_mut().expect("Failed to open stdin");
        stdin.write_all(message.as_bytes())?;
    }

    let output = child.wait_with_output()?;

    // Assert
    assert!(output.status.success());
    assert_eq!(String::from_utf8_lossy(&output.stderr), expected);

    Ok(())
}

#[test]
fn verify_stdin_fails() -> Result<()> {
    use std::io::Write;
    use std::process::Stdio;

    // Arrange
    let message = "invalid message";

    // Act
    let mut cmd = Command::new(assert_cmd::cargo_bin!("cog"));
    cmd.arg("verify")
        .arg("--file")
        .arg("-")
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped());

    let mut child = cmd.spawn()?;

    // Write to stdin
    {
        let stdin = child.stdin.as_mut().expect("Failed to open stdin");
        stdin.write_all(message.as_bytes())?;
    }

    let output = child.wait_with_output()?;

    // Assert
    assert!(!output.status.success());

    Ok(())
}
