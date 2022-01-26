use crate::error::CocogittoError;
use crate::git::repository::Repository;
use anyhow::{anyhow, Result};
use git2::Oid;

impl Repository {
    pub(crate) fn commit(&self, message: &str) -> Result<Oid> {
        let sig = self.0.signature()?;
        let tree_id = self.0.index()?.write_tree()?;
        let tree = self.0.find_tree(tree_id)?;
        let is_empty = self.0.head().is_err();
        let has_delta = self.get_diff(false).is_some();

        if !is_empty && has_delta {
            let head = &self.0.head()?;
            let head_target = head.target().expect("Cannot get HEAD target");
            let tip = &self.0.find_commit(head_target)?;

            self.0
                .commit(Some("HEAD"), &sig, &sig, message, &tree, &[tip])
                .map_err(|err| anyhow!(err))
        } else if is_empty && has_delta {
            // First repo commit
            self.0
                .commit(Some("HEAD"), &sig, &sig, message, &tree, &[])
                .map_err(|err| anyhow!(err))
        } else {
            let err = self
                .get_branch_shorthand()
                .map(|branch| CocogittoError::NothingToCommitWithBranch { branch })
                .unwrap_or_else(|| CocogittoError::NothingToCommit);

            Err(anyhow!(err))
        }
    }
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
        let oid = repo.commit("feat: a test commit");

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
        let oid = repo.commit("feat: a test commit");

        // Assert
        assert_that!(oid).is_ok();
    }

    #[sealed_test]
    fn not_create_empty_commit() -> Result<()> {
        // Arrange
        let repo = Repository::init(".")?;

        // Act
        let oid = repo.commit("feat: a test commit");

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
        let oid = repo.commit("feat: a test commit");

        // Assert
        assert_that!(oid).is_err();
        Ok(())
    }
}
