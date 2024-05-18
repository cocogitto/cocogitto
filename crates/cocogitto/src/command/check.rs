use crate::error::CogCheckReport;

use crate::CocoGitto;
use anyhow::anyhow;
use anyhow::Result;
use cocogitto_commit::Commit;
use cocogitto_git::tag::TagLookUpOptions;
use colored::*;
use log::info;

impl CocoGitto {
    pub fn check(
        &self,
        check_from_latest_tag: bool,
        ignore_merge_commits: bool,
        range: Option<String>,
    ) -> Result<()> {
        let commit_range = if let Some(range) = range {
            self.repository.revwalk(&range)?
        } else if check_from_latest_tag {
            let tag = self
                .repository
                .get_latest_tag(TagLookUpOptions::default())?;
            self.repository.revwalk(&format!("{tag}.."))?
        } else {
            self.repository.revwalk("..")?
        };

        let errors: Vec<_> = if ignore_merge_commits {
            commit_range
                .iter_commits()
                .filter(|commit| commit.parent_count() <= 1)
                .map(Commit::from_git_commit)
                .filter_map(Result::err)
                .collect()
        } else {
            commit_range
                .iter_commits()
                .map(Commit::from_git_commit)
                .filter_map(Result::err)
                .collect()
        };

        if errors.is_empty() {
            let msg = "No errored commits".green();
            info!("{}", msg);
            Ok(())
        } else {
            let report = CogCheckReport {
                from: commit_range.from_oid(),
                errors: errors.into_iter().map(|err| *err).collect(),
            };
            Err(anyhow!("{}", report))
        }
    }
}
