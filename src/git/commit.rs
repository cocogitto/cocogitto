use crate::git::error::Git2Error;
use crate::git::repository::Repository;
use git2::{Commit, ObjectType, Oid, ResetType, Signature, Tree};
use std::io::Write;
use std::process::{Command, Stdio};

impl Repository {
    pub(crate) fn commit(
        &self,
        message: &str,
        sign: bool,
        allow_empty_delta: bool,
    ) -> Result<Oid, Git2Error> {
        let sig = self.0.signature()?;
        let tree_id = self.0.index()?.write_tree()?;
        let tree = self.0.find_tree(tree_id)?;
        let is_empty = self.0.head().is_err();
        let has_delta = self.get_diff(false).is_some();
        let has_delta_or_allowed_empty = has_delta || allow_empty_delta;

        if !is_empty && has_delta_or_allowed_empty {
            let head = &self.0.head()?;
            let head_target = head.target().expect("Cannot get HEAD target");
            let tip = &self.0.find_commit(head_target)?;

            self.commit_or_signed_commit(&sig, message, &tree, &[tip], sign)
                .map_err(Git2Error::from)
        } else if is_empty && has_delta_or_allowed_empty {
            // First repo commit
            self.commit_or_signed_commit(&sig, message, &tree, &[], sign)
                .map_err(Git2Error::from)
        } else {
            let statuses = self.get_statuses()?;
            let statuses = if statuses.0.is_empty() {
                None
            } else {
                Some(statuses)
            };
            let branch = self.get_branch_shorthand();
            Err(Git2Error::NothingToCommit { branch, statuses })
        }
    }

    fn commit_or_signed_commit(
        &self,
        sig: &Signature,
        commit_message: &str,
        tree: &Tree,
        parents: &[&Commit],
        sign: bool,
    ) -> Result<Oid, Git2Error> {
        if !sign {
            return self
                .0
                .commit(Some("HEAD"), sig, sig, commit_message, tree, parents)
                .map_err(Git2Error::Other);
        }

        let commit_buf = self
            .0
            .commit_create_buffer(sig, sig, commit_message, tree, parents)?;

        let commit_as_str = std::str::from_utf8(&commit_buf)
            .expect("Invalid UTF-8 commit message")
            .to_string();

        let key = self.signin_key().ok();
        let gpg_signature = gpg_sign_string(key, &commit_as_str)?;
        let oid = self
            .0
            .commit_signed(&commit_as_str, &gpg_signature, Some("gpgsig"))?;

        // This is needed because git2 does not update HEAD after creating a signed commit
        let commit = self.0.find_object(oid, Some(ObjectType::Commit))?;
        self.0.reset(&commit, ResetType::Mixed, None)?;
        Ok(oid).map_err(Git2Error::Other)
    }
}

fn gpg_sign_string(key: Option<String>, content: &str) -> Result<String, Git2Error> {
    let mut child = Command::new("gpg");
    child.args(["--armor", "--detach-sig"]);

    if let Some(key) = &key {
        child.args(["--default-key", key]);
    }

    let mut child = child
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .spawn()
        .expect("error calling gpg command, is gpg installed ?");

    {
        let stdin = child.stdin.as_mut().unwrap();
        stdin.write_all(content.as_bytes())?;
    }

    child.wait_with_output().map(|output| {
        if output.status.success() {
            Ok(String::from_utf8_lossy(&output.stdout).to_string())
        } else {
            Err(Git2Error::GpgError(
                String::from_utf8_lossy(&output.stderr).to_string(),
            ))
        }
    })?
}

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
        run_cmd!(
            git init;
            echo changes > file;
            git add .;
        )?;

        let repo = Repository::open(".")?;

        // Act
        let oid = repo.commit("feat: a test commit", false, false);

        // Assert
        assert_that!(oid).is_ok();
        Ok(())
    }

    #[cfg(not(target_os = "windows"))]
    #[sealed_test]
    fn create_signed_commit_ok() -> Result<()> {
        // Arrange
        let crate_dir = std::env::var("CARGO_MANIFEST_DIR")?;

        run_cmd!(
            gpg --import $crate_dir/tests/assets/pubkey.key;
            gpg --import $crate_dir/tests/assets/privkey.key;
            echo -e "5\ny\n" | gpg --no-tty --command-fd 0 --expert --edit-key test@cocogitto.org trust;
            git init;
            git config --local user.signingkey 35B66CC21AEBFC9B0E8C89F1FD753A01E06E05D7;
            echo changes > file;
            git add .;
        )?;

        let repo = Repository::open(".")?;

        // Act
        let oid = repo.commit("feat: a test commit", true, false);

        // Assert
        assert_that!(oid).is_ok();
        Ok(())
    }

    #[sealed_test]
    fn create_empty_commit() -> Result<()> {
        // Arrange
        run_cmd!(
            git init;
        )?;

        let repo = Repository::open(".")?;

        // Act
        let oid = repo.commit("feat: a test commit", false, true);

        // Assert
        assert_that!(oid).is_ok();
        Ok(())
    }

    #[sealed_test]
    fn first_commit_custom_branch() {
        // Arrange
        run_cmd!(
            git init -b main;
            echo changes > file;
            git add .;
        )
        .expect("could not initialize git repository");

        let repo = Repository::open(".").expect("could not open git repository");

        // Act
        let oid = repo.commit("feat: a test commit", false, false);

        // Assert
        assert_that!(oid).is_ok();
    }

    #[sealed_test]
    fn not_create_empty_commit() -> Result<()> {
        // Arrange
        let repo = Repository::init(".")?;

        // Act
        let oid = repo.commit("feat: a test commit", false, false);

        // Assert
        assert_that!(oid).is_err();
        Ok(())
    }

    #[sealed_test]
    fn not_create_empty_commit_with_unstaged_changed() -> Result<()> {
        // Arrange
        run_cmd!(
            git init;
            echo changes > file;
        )?;

        let repo = Repository::open(".")?;

        // Act
        let oid = repo.commit("feat: a test commit", false, false);

        // Assert
        assert_that!(oid).is_err();
        Ok(())
    }
}
