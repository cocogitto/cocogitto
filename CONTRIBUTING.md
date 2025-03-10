# Cocogitto's contributing guide

Thank you for taking your time to contribute to Cocogitto! Below is a non-exhaustive list of what you can
do to help Cocogitto.

## Opening issues

If you spot a bug or want to request a feature you are welcome to [open an issue](https://github.com/cocogitto/cocogitto/issues/new/choose).

## Writing documentation

The official Cocogitto documentation is hosted on the [cocogitto/website](https://github.com/cocogitto/website) repository.
If you find something is missing or unclear please open a pull request or an issue there.

## Submitting a pull request

Before submitting a PR don't hesitate to talk about your feature request or bug fix either on the issue board or on
our [discord server](https://discord.gg/951009223121195021).
If you need an early review or guidance on how to implement a feature or bug fix, draft PRs are welcome.

### Prerequisites

Cocogitto is a toolset for [Conventional Commits](https://www.conventionalcommits.org/en/v1.0.0/) and [SemVer](https://semver.org/)
, and we aim to follow these specifications.

In addition, our CI pipeline uses the following formatter and linter:

- [rustfmt](https://github.com/rust-lang/rustfmt)

  rustfmt is a code formatter for rust, refer to their documentation for installation.

  Before committing your changes please run `cargo fmt --all`

- [clippy](https://github.com/rust-lang/rust-clippy)

  clippy is a code linter for rust, refer to their documentation for installation.

  Use `cargo clippy` instead of `cargo build` to spot lints before committing your changes.

- **git-hooks**
  Cocogitto provides sharable git-hooks, if you want to save the hassle of manually running lints and formatters before
  each commit you can simply run `cog install-hook --all`.

### Error handling
* `unwrap()` usage should be avoided, if needed use `expect("why")` instead.
* For convenience, errors in the [cog](src/bin/cog.rs) CLI and the public API are wrapped with the
  [`anyhow!`](https://docs.rs/anyhow/latest/anyhow/macro.anyhow.html) macro.
  On the other hand, in order to be predictable, every error in the private part of the crate should be defined
  in the corresponding `error.rs` file. For instance, git2 errors are defined under [src/git/error.rs](src/git/error.rs).

### Testing

Before submitting a PR please make sure your code is tested. The CI pipeline ensures that coverage never drops by more than
1%, and total coverage should never go below 80%.

**Testing git repositories:**

Due to the extensive usage of [git2](https://crates.io/crates/git2) in Cocogitto,
testing within a repository is often required.


This kind of tests must use the [`#[sealed_test]`](https://crates.io/crates/sealed_test)
macro in place off the `#[test]` macro. [sealed_test](https://crates.io/crates/sealed_test) allow to run a test
within a dedicated environment (a temporary working directory, and its own environment variables).

If writing an integration test you can set up a git repository with the [tests/helpers.rs](tests/helpers.rs).
If you need to run some additional shell command or are not writing an integration test use the `run_cmd!`/`run_fun!` macros.

**Unit test:**

Cocogitto tests are organized as described in [The Rust Programing Language book](https://doc.rust-lang.org/stable/book/ch11-03-test-organization.html).
Private function should be tested right where they live

Example:
```rust
// mod commit ...
#[cfg(test)]
mod test {
  use crate::git::repository::Repository;
  use anyhow::Result;
  use cmd_lib::run_cmd;
  use sealed_test::prelude::*;
  use speculoos::prelude::*;

  #[sealed_test]
  fn create_commit_ok() -> Result<()> {
    // Arrange
    let repo = git_init_no_gpg()?;

    run_cmd!(
            echo changes > file;
            git add .;
        )?;

    // Act
    let oid = repo.commit("feat: a test commit");

    // Assert
    assert_that!(oid).is_ok();
    Ok(())
  }
// ...
}
```

**Integration test:**

These tests live in the [tests](tests) directory.
- [tests/lib_tests](tests/lib_tests) contains tests for the public API function
- [tests/cog_tests](tests/cog_tests) contains CLI integration tests using the [asserd_cmd crate](https://crates.io/crates/assert_cmd)

**Example:**
```rust
#[sealed_test]
fn should_skip_initialization_if_repository_exists() -> Result<()> {
    // Arrange
    git_init()?;
    git_commit("The first commit")?;

    // Act
    cocogitto::init(".")?;

    // Assert
    assert_that!(Path::new("cog.toml")).exists();
    assert_that!(git_log_head()?).is_equal_to("The first commit\n".to_string());
    assert_that!(git_status()?).contains("new file:   cog.toml");
    Ok(())
}
```
