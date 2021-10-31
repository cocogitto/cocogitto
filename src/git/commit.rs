use crate::error::CocogittoError;
use crate::git::repository::Repository;
use anyhow::{anyhow, Result};
use git2::Oid;

impl Repository {
    pub(crate) fn commit(&self, message: &str) -> Result<Oid> {
        let sig = self.0.signature()?;
        let tree_id = self.0.index()?.write_tree()?;
        let tree = self.0.find_tree(tree_id)?;
        let is_empty = self.0.is_empty()?;
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
    use crate::test_helpers::run_test_with_context;
    use anyhow::Result;
    use speculoos::prelude::*;

    #[test]
    fn create_commit_ok() -> Result<()> {
        run_test_with_context(|context| {
            let repo = Repository::init(".")?;
            std::fs::write(context.test_dir.join("file"), "changes")?;
            repo.add_all()?;

            assert_that!(repo.commit("feat: a test commit")).is_ok();
            Ok(())
        })
    }

    #[test]
    fn not_create_empty_commit() -> Result<()> {
        run_test_with_context(|_| {
            let repo = Repository::init(".")?;

            assert_that!(repo.commit("feat: a test commit")).is_err();
            Ok(())
        })
    }

    #[test]
    fn not_create_empty_commit_with_unstaged_changed() -> Result<()> {
        run_test_with_context(|context| {
            let repo = Repository::init(".")?;
            std::fs::write(context.test_dir.join("file"), "changes")?;

            assert_that!(repo.commit("feat: a test commit")).is_err();
            Ok(())
        })
    }
}
