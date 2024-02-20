use crate::conventional::commit::Commit;
use crate::git::tag::TagLookUpOptions;
use crate::log::filter::CommitFilters;
use crate::CocoGitto;
use anyhow::Result;
use std::fmt::Write;

impl CocoGitto {
    pub fn get_log(&self, filters: CommitFilters) -> Result<String> {
        let commits = self.repository.revwalk("..")?;
        let logs = commits
            .iter_commits()
            // Remove merge commits
            .filter(|commit| !commit.message().unwrap_or("").starts_with("Merge"))
            .filter(|commit| filters.filter_git2_commit(commit))
            .map(|commit| Commit::from_git_commit(commit))
            // Apply filters
            .filter(|commit| match commit {
                Ok(commit) => filters.filters(commit),
                Err(_) => filters.no_error(),
            })
            // Format
            .map(|commit| match commit {
                Ok(commit) => commit.get_log(),
                Err(err) => err.to_string(),
            })
            .collect::<Vec<String>>()
            .join("\n");

        Ok(logs)
    }

    pub fn get_repo_tag_name(&self) -> Option<String> {
        let repo_path = self.repository.get_repo_dir()?.iter().last()?;
        let mut repo_tag_name = repo_path.to_str()?.to_string();

        if let Some(branch_shorthand) = self.repository.get_branch_shorthand() {
            write!(&mut repo_tag_name, " on {branch_shorthand}").unwrap();
        }

        if let Ok(latest_tag) = self
            .repository
            .get_latest_tag(TagLookUpOptions::default().include_pre_release())
        {
            write!(&mut repo_tag_name, " {latest_tag}").unwrap();
        };

        Some(repo_tag_name)
    }
}
