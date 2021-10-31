use crate::git::repository::Repository;
use anyhow::{anyhow, Result};

impl Repository {
    pub(crate) fn stash_failed_version(&mut self, version: &str) -> Result<()> {
        let sig = self.0.signature()?;
        let message = &format!("cog_bump_{}", version);
        self.0
            .stash_save(&sig, message, None)
            .map(|_| ())
            .map_err(|err| anyhow!(err))
    }
}

#[cfg(test)]
mod test {
    use crate::git::repository::Repository;
    use crate::test_helpers::run_test_with_context;
    use anyhow::Result;
    use speculoos::prelude::*;

    #[test]
    fn should_stash_failed_bump() -> Result<()> {
        run_test_with_context(|context| {
            let mut repo = Repository::init(&context.test_dir)?;
            std::fs::write(&context.test_dir.join("file"), "changes")?;
            repo.add_all()?;
            repo.commit("Initial commit")?;

            let statuses = repo.get_statuses()?.0;
            assert_that!(statuses).is_empty();

            std::fs::write(&context.test_dir.join("second_file"), "more changes")?;
            repo.add_all()?;
            let statuses = repo.get_statuses()?.0;
            assert_that!(statuses).has_length(1);

            repo.stash_failed_version("1.0.0")?;

            let statuses = repo.get_statuses()?.0;
            assert_that!(statuses).is_empty();
            Ok(())
        })
    }
}
