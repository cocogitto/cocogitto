use crate::conventional::commit::Commit;
use crate::error::CogCheckReport;

use crate::git::tag::TagLookUpOptions;
use crate::CocoGitto;
use anyhow::anyhow;
use anyhow::Result;
use colored::*;
use log::info;

impl CocoGitto {
    pub fn check(
        &self,
        check_from_latest_tag: bool,
        ignore_merge_commits: bool,
        ignore_fixup_commits: bool,
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

        let ignore_merge_commit_fn = |commit: &git2::Commit| commit.parent_count() <= 1;
        let ignore_fixup_commit_fn = |commit: &git2::Commit| !commit.message().unwrap().starts_with("fixup!")
            && !commit.message().unwrap().starts_with("squash!")
            && !commit.message().unwrap().starts_with("amend!");


        let errors: Vec<_> = commit_range
            .iter_commits()
            .filter(|commit| if ignore_merge_commits && ignore_fixup_commits {
                ignore_merge_commit_fn(commit) && ignore_fixup_commit_fn(commit)
            } else if ignore_fixup_commits {
                ignore_fixup_commit_fn(commit)
            } else if ignore_merge_commits {
                ignore_merge_commit_fn(commit)
            } else {
                true
            })
            .map(Commit::from_git_commit)
            .filter_map(Result::err)
            .collect();

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
