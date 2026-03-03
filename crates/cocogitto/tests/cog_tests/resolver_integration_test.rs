use crate::helpers::*;

use assert_cmd::Command;
use cmd_lib::run_cmd;
use sealed_test::prelude::*;
use speculoos::prelude::*;
use std::fs;

#[sealed_test]
fn test_resolver_based_bump_order() -> anyhow::Result<()> {
    // Arrange
    git_init()?;

    run_cmd! {
        mkdir packages;
        mkdir packages/package-a;
        mkdir packages/package-b;
        mkdir packages/package-c;
        echo "content a" > packages/package-a/file.txt;
        echo "content b" > packages/package-b/file.txt;
        echo "content c" > packages/package-c/file.txt;
    }?;

    fs::write(
        "Cargo.toml",
        r#"
[workspace]
members = [
    "packages/package-a",
    "packages/package-b",
    "packages/package-c"
]
"#,
    )?;

    fs::write(
        "packages/package-a/Cargo.toml",
        r#"
[package]
name = "package-a"
version = "0.1.0"
edition = "2021"

[lib]
name = "package_a"
path = "src/lib.rs"

[dependencies]
package-b = { path = "../package-b" }
"#,
    )?;

    fs::write(
        "packages/package-b/Cargo.toml",
        r#"
[package]
name = "package-b"
version = "0.1.0"
edition = "2021"

[lib]
name = "package_b"
path = "src/lib.rs"

[dependencies]
package-c = { path = "../package-c" }
"#,
    )?;

    fs::write(
        "packages/package-c/Cargo.toml",
        r#"
[package]
name = "package-c"
version = "0.1.0"
edition = "2021"

[lib]
name = "package_c"
path = "src/lib.rs"
"#,
    )?;

    fs::create_dir_all("packages/package-a/src")?;
    fs::create_dir_all("packages/package-b/src")?;
    fs::create_dir_all("packages/package-c/src")?;
    fs::write("packages/package-a/src/lib.rs", "// dummy lib")?;
    fs::write("packages/package-b/src/lib.rs", "// dummy lib")?;
    fs::write("packages/package-c/src/lib.rs", "// dummy lib")?;

    let config = r#"
[packages.package-a]
path = "packages/package-a"
resolver = "Cargo"

[packages.package-b]
path = "packages/package-b"
resolver = "Cargo"

[packages.package-c]
path = "packages/package-c"
resolver = "Cargo"
"#;

    fs::write("cog.toml", config)?;

    // Commit initial setup
    run_cmd!(git add .; git commit -m "chore: initial setup")?;

    // Make changes to packages in reverse dependency order (c depends on b depends on a)
    fs::write("packages/package-c/new_file.txt", "new content c")?;
    run_cmd!(git add .; git commit -m "feat: add feature to package-c")?;

    fs::write("packages/package-b/new_file.txt", "new content b")?;
    run_cmd!(git add .; git commit -m "feat: add feature to package-b")?;

    fs::write("packages/package-a/new_file.txt", "new content a")?;
    run_cmd!(git add .; git commit -m "feat: add feature to package-a")?;

    // Act
    let output = Command::new(assert_cmd::cargo_bin!("cog"))
        .args(["bump", "--auto", "--dry-run"])
        .output()?;

    let stderr = String::from_utf8(output.stderr)?;
    let stdout = String::from_utf8(output.stdout)?;

    // Assert
    assert_that!(stdout).is_equal_to("toto".to_string());
    assert_that!(stderr).is_equal_to("toto".to_string());

    Ok(())
}
