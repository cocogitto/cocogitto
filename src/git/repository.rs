use std::path::Path;
use std::{
    fmt::{Debug, Formatter},
    path::PathBuf,
};

use crate::git::error::Git2Error;
use git2::{
    Commit as Git2Commit, IndexAddOption, Object, ObjectType, Oid, Repository as Git2Repository,
};

pub(crate) struct Repository(pub(crate) Git2Repository);

impl Repository {
    pub(crate) fn signing_key(&self) -> Result<String, Git2Error> {
        let config = self.0.config()?;
        config.get_string("user.signingKey").map_err(Into::into)
    }

    pub(crate) fn signing_key_path(&self) -> Result<PathBuf, Git2Error> {
        let config = self.0.config()?;
        config.get_path("user.signingKey").map_err(Into::into)
    }

    pub(crate) fn gpg_sign(&self) -> bool {
        let config = self.0.config().expect("failed to retrieve gitconfig");
        config.get_bool("commit.gpgSign").unwrap_or(false)
    }

    pub(crate) fn gpg_x509_program(&self) -> String {
        let config = self.0.config().expect("failed to retrieve gitconfig");
        config
            .get_string("gpg.x509.program")
            .unwrap_or("gpg".to_string())
    }

    pub(crate) fn gpg_program(&self) -> String {
        let config = self.0.config().expect("failed to retrieve gitconfig");
        config
            .get_string("gpg.program")
            .unwrap_or("gpg".to_string())
    }

    pub(crate) fn ssh_sign(&self) -> bool {
        let config = self.0.config().expect("failed to retrieve gitconfig");
        if config.get_bool("commit.gpgSign").is_err() {
            return false;
        }

        config
            .get_string("gpg.format")
            .is_ok_and(|s| s.to_lowercase() == "ssh")
    }

    pub(crate) fn ssh_program(&self) -> String {
        let config = self.0.config().expect("failed to retrieve gitconfig");
        config
            .get_string("gpg.ssh.program")
            .unwrap_or("ssh-keygen".to_string())
    }

    pub(crate) fn x509_sign(&self) -> bool {
        let config = self.0.config().expect("failed to retrieve gitconfig");
        if config.get_bool("commit.gpgSign").is_err() {
            return false;
        }

        config
            .get_string("gpg.format")
            .is_ok_and(|s| s.to_lowercase() == "x509")
    }

    pub(crate) fn init<S: AsRef<Path> + ?Sized>(path: &S) -> Result<Repository, Git2Error> {
        let repository =
            Git2Repository::init(path).map_err(Git2Error::FailedToInitializeRepository)?;
        Ok(Repository(repository))
    }

    pub(crate) fn open<S: AsRef<Path> + ?Sized>(path: &S) -> Result<Repository, Git2Error> {
        let repo = Git2Repository::discover(path).map_err(Git2Error::FailedToOpenRepository)?;
        Ok(Repository(repo))
    }

    pub(crate) fn get_repo_dir(&self) -> Option<&Path> {
        self.0.workdir()
    }

    pub(crate) fn add_all(&self) -> Result<(), Git2Error> {
        let mut index = self.0.index()?;
        index.add_all(["."], IndexAddOption::DEFAULT, None)?;
        index.write().map_err(Git2Error::GitAddError)
    }

    pub(crate) fn update_all(&self) -> Result<(), Git2Error> {
        let mut index = self.0.index()?;
        index.update_all(["."], None)?;
        index.write().map_err(Git2Error::GitAddError)
    }

    pub(crate) fn get_head_commit_oid(&self) -> Result<Oid, Git2Error> {
        self.get_head_commit().map(|commit| commit.id())
    }

    pub(crate) fn get_head_commit(&self) -> Result<Git2Commit, Git2Error> {
        let head_ref = self.0.head();
        match head_ref {
            Ok(head) => head.peel_to_commit().map_err(Git2Error::PeelToCommitError),
            Err(err) => Err(Git2Error::UnableToGetHead(err)),
        }
    }

    pub(crate) fn get_first_commit(&self) -> Result<Oid, Git2Error> {
        let mut revwalk = self.0.revwalk()?;
        revwalk.push_head()?;
        revwalk
            .last()
            .expect("No revision found")
            .map_err(Git2Error::CommitNotFound)
    }

    pub(crate) fn get_branch_shorthand(&self) -> Option<String> {
        self.0
            .head()
            .ok()
            .and_then(|head| head.shorthand().map(|shorthand| shorthand.to_string()))
    }

    pub(crate) fn get_author(&self) -> Result<String, Git2Error> {
        self.0
            .signature()?
            .name()
            .map(|name| name.to_string())
            .ok_or(Git2Error::CommitterNotFound)
    }

    pub(crate) fn tree_to_treeish(
        &self,
        arg: Option<&String>,
    ) -> Result<Option<Object>, git2::Error> {
        let arg = match arg {
            Some(s) => s,
            None => return Ok(None),
        };
        let obj = self.0.revparse_single(arg)?;
        let tree = obj.peel(ObjectType::Tree)?;
        Ok(Some(tree))
    }
}

impl Debug for Repository {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "Repository {{ 0: {:?}}}", self.0.path())
    }
}

#[cfg(test)]
mod test {
    use std::path::PathBuf;
    use std::str::FromStr;

    use anyhow::Result;
    use cmd_lib::run_cmd;
    use sealed_test::prelude::*;
    use speculoos::prelude::*;

    use crate::git::repository::Repository;
    use crate::test_helpers::git_init_no_gpg;

    #[sealed_test]
    fn init_repo() -> Result<()> {
        let repo = git_init_no_gpg();

        assert_that!(repo).is_ok();
        Ok(())
    }

    #[sealed_test]
    fn get_repo_working_dir_some() -> Result<()> {
        // Arrange
        let expected_dir = std::env::current_dir()?.canonicalize()?;
        let repo = Repository::init(&expected_dir)?;

        let dir = PathBuf::from_str("dir")?;
        std::fs::create_dir(&dir)?;
        std::env::set_current_dir(&dir)?;

        // Act
        let root_dir = repo.get_repo_dir().map(|p| p.canonicalize().unwrap());

        // Assert
        let expected_canonical = expected_dir.canonicalize()?;
        assert_that!(root_dir).is_equal_to(Some(expected_canonical));

        Ok(())
    }

    #[sealed_test]
    fn get_repo_head_oid_ok() -> Result<()> {
        // Arrange
        let repo = git_init_no_gpg()?;

        run_cmd!(
            echo changes > file;
            git add .;
        )?;
        let commit_oid = repo.commit("first commit", false, false)?;

        // Act
        let oid = repo.get_head_commit_oid();

        // Assert
        assert_that!(oid).is_ok().is_equal_to(commit_oid);
        Ok(())
    }

    #[sealed_test]
    fn get_repo_head_oid_err() -> Result<()> {
        // Arrange
        let repo = git_init_no_gpg()?;

        // Act
        let oid = repo.get_head_commit_oid();

        // Assert
        assert_that!(oid).is_err();
        Ok(())
    }

    #[sealed_test]
    fn get_repo_head_obj_ok() -> Result<()> {
        // Arrange
        let repo = git_init_no_gpg()?;

        run_cmd!(
            echo changes > file;
            git add .;
        )?;

        let commit_oid = repo.commit("first commit", false, false)?;

        // Act
        let head = repo.get_head_commit().map(|head| head.id());

        // Assert
        assert_that!(head).is_ok().is_equal_to(commit_oid);
        Ok(())
    }

    #[sealed_test]
    fn get_repo_head_obj_err() -> Result<()> {
        // Arrange
        let repo = git_init_no_gpg()?;

        run_cmd!(
            echo changes > file;
            git add .;
        )?;

        // Act
        let head = repo.get_head_commit();

        // Assert
        assert_that!(head).is_err();
        Ok(())
    }

    #[sealed_test]
    fn get_branch_short_hand() -> Result<()> {
        // Arrange
        let repo = git_init_no_gpg()?;

        run_cmd!(
            echo changes > file;
            git add .;
        )?;

        repo.commit("hello one", false, false)?;

        // Act
        let shorthand = repo.get_branch_shorthand();

        // Assert
        assert_that!(shorthand).is_equal_to(Some("master".to_string()));
        Ok(())
    }

    #[sealed_test]
    fn signing_key_path_resolves_tilde() -> Result<()> {
        // Arrange
        let repo = git_init_no_gpg()?;

        // update path to key
        run_cmd!(
            git config --local user.signingkey ~/.ssh/key.pub;
        )?;

        let path_to_signing_key = repo.signing_key_path().unwrap();
        let path_to_signing_key = path_to_signing_key.to_string_lossy();

        let home_env_var = if cfg!(windows) { "USERPROFILE" } else { "HOME" };
        let actual_home = std::env::var(home_env_var).unwrap();

        assert_that!(path_to_signing_key).starts_with(actual_home);

        Ok(())
    }
}
