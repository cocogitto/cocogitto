use crate::git::repository::Repository;
use git2::{Diff, DiffOptions};

impl Repository {
    pub(crate) fn get_diff(&self, include_untracked: bool) -> Option<Diff> {
        let mut options = DiffOptions::new();
        options.include_untracked(include_untracked);

        let diff = match &self.get_head() {
            Some(head) => self
                .0
                .diff_tree_to_index(head.as_tree(), None, Some(&mut options)),
            None => self
                .0
                .diff_tree_to_workdir_with_index(None, Some(&mut options)),
        };

        match diff {
            Ok(diff) => {
                if diff.deltas().len() > 0 {
                    Some(diff)
                } else {
                    None
                }
            }
            Err(..) => None,
        }
    }
}

#[cfg(test)]
mod test {
    use crate::git::repository::Repository;
    use anyhow::Result;
    use cmd_lib::run_cmd;
    use sealed_test::prelude::*;

    #[sealed_test]
    fn get_diff_some() -> Result<()> {
        // Arrange
        run_cmd!(
            git init;
            echo changes > file;
            git add .;
        )?;

        let repo = Repository::open(".")?;

        // Act
        let diffs = repo.get_diff(false);

        // Assert
        assert!(diffs.is_some());
        Ok(())
    }

    #[sealed_test]
    fn get_diff_none() -> Result<()> {
        // Arrange
        run_cmd!(
            git init;
            echo changes > file;
        )?;

        let repo = Repository::open(".")?;

        // Act
        let diffs = repo.get_diff(false);

        // Assert
        assert!(diffs.is_none());
        Ok(())
    }

    #[sealed_test]
    fn get_diff_include_untracked_some() -> Result<()> {
        // Arrange
        run_cmd!(
            git init;
            echo changes > file;
        )?;

        let repo = Repository::open(".")?;

        // Act
        let diffs = repo.get_diff(true);

        // Assert
        assert!(diffs.is_some());
        Ok(())
    }

    #[sealed_test]
    fn get_diff_include_untracked_none() -> Result<()> {
        // Arrange
        let repo = Repository::init(".")?;

        // Act
        let diffs = repo.get_diff(true);

        // Assert
        assert!(diffs.is_none());
        Ok(())
    }
}
