use crate::git::error::Git2Error;
use crate::git::repository::Repository;
use crate::Tag;

impl Repository {
    pub(crate) fn stash_failed_version(&mut self, tag: Tag) -> Result<(), Git2Error> {
        let sig = self.0.signature()?;
        let message = &format!("cog_bump_{tag}");
        self.0
            .stash_save(&sig, message, None)
            .map(|_| ())
            .map_err(Git2Error::StashError)
    }
}

#[cfg(test)]
mod test {
    use crate::git::repository::Repository;
    use crate::Tag;
    use anyhow::Result;
    use cmd_lib::run_cmd;
    use sealed_test::prelude::*;
    use speculoos::prelude::*;

    #[sealed_test]
    fn should_stash_failed_bump() -> Result<()> {
        let mut repo = Repository::init(".")?;
        run_cmd!(git commit -m "Initial commit" --allow-empty;)?;

        let statuses = repo.get_statuses()?.0;
        assert_that!(statuses).is_empty();

        run_cmd!(
            echo "changes" > file;
            git add .;
        )?;
        let statuses = repo.get_statuses()?.0;

        assert_that!(statuses).has_length(1);
        repo.stash_failed_version(Tag::from_str("1.0.0", None)?)?;

        let statuses = repo.get_statuses()?.0;
        assert_that!(statuses).is_empty();
        Ok(())
    }
}
