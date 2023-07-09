use crate::conventional::commit::Commit;
use crate::error::CogCheckReport;
use crate::git::revspec::RevspecPattern;
use crate::CocoGitto;
use anyhow::anyhow;
use anyhow::Result;
use colored::*;
use log::info;
use std::str::FromStr;

impl CocoGitto {
    pub fn check(
        &self,
        check_from_latest_tag: bool,
        ignore_merge_commits: bool,
        range: Option<String>,
    ) -> Result<()> {
        let commit_range = if let Some(range) = range {
            self.repository
                .get_commit_range(&RevspecPattern::from_str(range.as_str())?)?
        } else if check_from_latest_tag {
            self.repository
                .get_commit_range(&RevspecPattern::default())?
        } else {
            self.repository.all_commits()?
        };

        let errors: Vec<_> = if ignore_merge_commits {
            commit_range
                .commits
                .iter()
                .filter(|commit| commit.parent_count() <= 1)
                .map(Commit::from_git_commit)
                .filter_map(Result::err)
                .collect()
        } else {
            commit_range
                .commits
                .iter()
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
                from: commit_range.from,
                errors: errors.into_iter().map(|err| *err).collect(),
            };
            Err(anyhow!("{}", report))
        }
    }
}
