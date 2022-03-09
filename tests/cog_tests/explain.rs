use std::process::Command;

use anyhow::Result;
use assert_cmd::prelude::*;
use predicates::prelude::predicate;

#[test]
fn given_feat_commit_type_should_print_description() -> Result<()> {
    Command::cargo_bin("cog")?
        .arg("explain")
        .arg("feat")
        // Assert
        .assert()
        .success()
        .stdout(predicate::str::starts_with("feat:"));
    Ok(())
}

#[test]
fn given_unknown_commit_type_should_not_print_default_message() -> Result<()> {
    Command::cargo_bin("cog")?
        .arg("explain")
        .arg("feat")
        // Assert
        .assert()
        .success()
        .stdout(predicate::str::starts_with("feat:"));
    Ok(())
}
