use crate::git::error::Git2Error;
use crate::git::repository::Repository;
use git2::{Commit, ObjectType, Oid, ResetType, Signature, Tree};
use std::fs;
use std::io::{Read, Write};
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

        let signature = if self.ssh_sign() {
            let program = self.ssh_program();
            ssh_sign_string(program, key, &commit_as_str)?
        } else if self.x509_sign() {
            let program = self.gpg_x509_program();
            let user = sig.email().ok_or(Git2Error::MissingEmailInSignature)?;
            x509_gitsign(program, user, &commit_as_str)?
        } else {
            let program = self.gpg_program();
            gpg_sign_string(program, key, &commit_as_str)?
        };

        let oid = self
            .0
            .commit_signed(&commit_as_str, &signature, Some("gpgsig"))?;

        // This is needed because git2 does not update HEAD after creating a signed commit
        let commit = self.0.find_object(oid, Some(ObjectType::Commit))?;
        self.0.reset(&commit, ResetType::Mixed, None)?;
        Ok(oid).map_err(Git2Error::Other)
    }
}

// Clippy does not seem to be aware that `wait_with_output` is equivalent to `wait`
#[allow(clippy::zombie_processes)]
fn x509_gitsign(program: String, user: &str, content: &str) -> Result<String, Git2Error> {
    let mut child = Command::new(program);
    child.args(["--armor", "-b", "-s", "-u", user]);

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

#[allow(clippy::zombie_processes)]
fn gpg_sign_string(
    program: String,
    key: Option<String>,
    content: &str,
) -> Result<String, Git2Error> {
    let mut child = Command::new(program);
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

fn ssh_sign_string(
    program: String,
    key: Option<String>,
    content: &str,
) -> Result<String, Git2Error> {
    let Some(key) = key else {
        return Err(Git2Error::SshError("No ssh key found".to_string()));
    };

    let mut child = Command::new(program);
    child.args(["-Y", "sign", "-n", "git"]);

    let mut signing_key = tempfile::NamedTempFile::new()?;
    let mut buffer = tempfile::NamedTempFile::new()?;
    signing_key.write_all(key.as_bytes())?;
    buffer.write_all(content.as_bytes())?;
    let signing_key_ref = signing_key.into_temp_path();
    let buffer_ref = buffer.into_temp_path();
    child.args([
        "-f",
        signing_key_ref.to_string_lossy().as_ref(),
        buffer_ref.to_string_lossy().as_ref(),
    ]);

    child
        .stdin(Stdio::null())
        .stdout(Stdio::null())
        .spawn()
        .map_err(Git2Error::IOError)?
        .wait()?;

    let mut signature = String::new();
    let sig_file = buffer_ref.to_str().unwrap().to_string() + ".sig";
    fs::File::open(sig_file)?
        .read_to_string(&mut signature)
        .map_err(Git2Error::IOError)?;

    Ok(signature)
}

#[cfg(test)]
mod test {
    use crate::git::repository::Repository;
    use crate::test_helpers::git_init_no_gpg;
    use anyhow::Result;
    use cmd_lib::{run_cmd, run_fun};
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
            git config --local user.signingkey 24CAC643C7098768E2A90E1A2A8180559460836E;
            git config --local commit.gpgsign true;
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
        let repo = git_init_no_gpg()?;

        // Act
        let oid = repo.commit("feat: a test commit", false, true);

        // Assert
        assert_that!(oid).is_ok();
        Ok(())
    }

    #[cfg(not(target_os = "windows"))]
    #[sealed_test]
    fn crate_signed_ssh_commit_ok() -> Result<()> {
        // Arrange
        let crate_dir = std::env::var("CARGO_MANIFEST_DIR")?;

        // Source a ssh-agent and set env variables
        let output = run_fun!(
            ssh-agent;
        )?;

        let variables = output.split(';').map(|s| s.trim()).collect::<Vec<&str>>();
        let ssh_auth_sock = variables[0].split('=').collect::<Vec<&str>>()[1];
        let ssh_agent_pid = variables[2].split('=').collect::<Vec<&str>>()[1];
        std::env::set_var("SSH_AUTH_SOCK", ssh_auth_sock);
        std::env::set_var("SSH_AGENT_PID", ssh_agent_pid);

        run_cmd!(
            git init;
            chmod 600 $crate_dir/tests/assets/sshkey;
            chmod 600 $crate_dir/tests/assets/sshkey.pub;
            git config --local user.signingkey "ssh-ed25519 AAAAC3NzaC1lZDI1NTE5AAAAIHsnHukmf4SX31jdbf+aZjH2pvmHwuz7ysxdjErMK+i2";
            git config --local commit.gpgSign true;
            git config --local gpg.format ssh;
            git config --local gpg.ssh.program ssh-keygen;
            ssh-add $crate_dir/tests/assets/sshkey;
            echo changes > file;
            git add .;
        )?;

        let repo = Repository::open(".")?;

        // Act
        let oid = repo.commit("feat: a test commit", true, false);

        // Clean up
        run_cmd!(
            ssh-add -d $crate_dir/tests/assets/sshkey;
            kill $ssh_agent_pid;
        )?;

        // Assert
        assert_that!(oid).is_ok();
        Ok(())
    }

    #[sealed_test]
    fn first_commit_custom_branch() {
        // Arrange
        run_cmd!(
            git init -b main;
            git config --local commit.gpgsign false;
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
        let repo = git_init_no_gpg()?;

        // Act
        let oid = repo.commit("feat: a test commit", false, false);

        // Assert
        assert_that!(oid).is_err();
        Ok(())
    }

    #[sealed_test]
    fn not_create_empty_commit_with_unstaged_changed() -> Result<()> {
        // Arrange
        let repo = git_init_no_gpg()?;
        run_cmd!(echo changes > file;)?;

        // Act
        let oid = repo.commit("feat: a test commit", false, false);

        // Assert
        assert_that!(oid).is_err();
        Ok(())
    }
}
