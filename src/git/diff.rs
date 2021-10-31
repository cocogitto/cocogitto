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
    use crate::test_helpers::run_test_with_context;
    use anyhow::Result;

    #[test]
    fn get_diff_some() -> Result<()> {
        run_test_with_context(|context| {
            let repo = Repository::init(&context.test_dir)?;
            std::fs::write(context.test_dir.join("file"), "changes")?;
            repo.add_all()?;

            assert!(repo.get_diff(false).is_some());
            Ok(())
        })
    }

    #[test]
    fn get_diff_none() -> Result<()> {
        run_test_with_context(|context| {
            let repo = Repository::init(&context.test_dir)?;
            std::fs::write(context.test_dir.join("file"), "changes")?;

            assert!(repo.get_diff(false).is_none());
            Ok(())
        })
    }

    #[test]
    fn get_diff_include_untracked_some() -> Result<()> {
        run_test_with_context(|context| {
            let repo = Repository::init(&context.test_dir)?;
            std::fs::write(context.test_dir.join("file"), "changes")?;

            assert!(repo.get_diff(true).is_some());
            Ok(())
        })
    }

    #[test]
    fn get_diff_include_untracked_none() -> Result<()> {
        run_test_with_context(|context| {
            let repo = Repository::init(&context.test_dir)?;

            assert!(repo.get_diff(true).is_none());
            Ok(())
        })
    }
}
