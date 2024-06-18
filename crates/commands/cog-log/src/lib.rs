mod filter;
mod output;

use crate::filter::CommitFilter;
use crate::output::Output;
use anyhow::{Context, Result};
use cocogitto::CogCommand;
use cocogitto_commit::{Commit, CommitType};
use cocogitto_git::tag::TagLookUpOptions;
use cocogitto_git::Repository;
use filter::CommitFilters;
use std::fmt::Write;

pub struct CogLogCommand {
    pub breaking_change: bool,
    pub typ: Option<Vec<String>>,
    pub author: Option<Vec<String>>,
    pub scope: Option<Vec<String>>,
    pub no_error: bool,
}

impl CogCommand for CogLogCommand {
    fn execute(self) -> Result<()> {
        let repository = &Self::repository()?;
        let settings = &Self::settings()?;
        let repo_tag_name = get_repo_tag_name(repository);
        let repo_tag_name = repo_tag_name.as_deref().unwrap_or("cog log");

        let mut output = Output::builder()
            .with_pager_from_env("PAGER")
            .with_file_name(repo_tag_name)
            .build()?;

        let mut filters = vec![];
        if let Some(commit_types) = self.typ {
            filters.extend(
                commit_types
                    .iter()
                    .map(|commit_type| CommitFilter::Type(commit_type.as_str().into())),
            );
        }

        if let Some(scopes) = self.scope {
            filters.extend(scopes.into_iter().map(CommitFilter::Scope));
        }

        if let Some(authors) = self.author {
            filters.extend(authors.into_iter().map(CommitFilter::Author));
        }

        if self.breaking_change {
            filters.push(CommitFilter::BreakingChange);
        }

        if self.no_error {
            filters.push(CommitFilter::NoError);
        }

        let filters = CommitFilters(filters);

        let content = get_log(repository, filters, &settings.allowed_commit_types())?;

        output
            .handle()?
            .write_all(content.as_bytes())
            .context("failed to write log into the pager")
    }
}

fn get_log(
    repository: &Repository,
    filters: CommitFilters,
    allowed_commits: &[CommitType],
) -> Result<String> {
    let commits = repository.revwalk("..")?;
    let logs = commits
        .iter_commits()
        // Remove merge commits
        .filter(|commit| !commit.message().unwrap_or("").starts_with("Merge"))
        .filter(|commit| filters.filter_git2_commit(commit))
        .map(|commit| Commit::from_git_commit(commit, allowed_commits))
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

fn get_repo_tag_name(repository: &Repository) -> Option<String> {
    let repo_path = repository.get_repo_dir()?.iter().last()?;
    let mut repo_tag_name = repo_path.to_str()?.to_string();

    if let Some(branch_shorthand) = repository.get_branch_shorthand() {
        write!(&mut repo_tag_name, " on {branch_shorthand}").unwrap();
    }

    if let Ok(latest_tag) =
        repository.get_latest_tag(TagLookUpOptions::default().include_pre_release())
    {
        write!(&mut repo_tag_name, " {latest_tag}").unwrap();
    };

    Some(repo_tag_name)
}

#[cfg(test)]
mod test {
    use crate::filter::{CommitFilter, CommitFilters};
    use crate::get_log;
    use anyhow::Result;
    use cocogitto_commit::CommitType;
    use cocogitto_test_helpers::*;
    use sealed_test::prelude::*;
    use speculoos::prelude::*;

    #[sealed_test]
    fn get_unfiltered_logs() -> Result<()> {
        // Arrange
        let repository = git_init_no_gpg()?;
        git_commit("feat: a commit")?;
        git_commit("test: do you test your code ?")?;
        git_commit("I am afraid I can't do that Dave")?;
        let filters = CommitFilters(Vec::with_capacity(0));

        // Act
        let logs = get_log(
            &repository,
            filters,
            &[CommitType::Feature, CommitType::Test],
        )?;

        // Assert
        assert_that!(logs).contains("I am afraid I can't do that Dave");
        assert_that!(logs).contains("Missing commit type separator `:`");

        Ok(())
    }

    #[sealed_test]
    fn get_log_with_no_errors() -> Result<()> {
        // Arrange
        let repository = git_init_no_gpg()?;
        git_commit("feat: a commit")?;
        git_commit("test: do you test your code ?")?;
        git_commit("I am afraid I can't do that Dave")?;

        let filters = CommitFilters(vec![CommitFilter::NoError]);

        // Act
        let logs = get_log(
            &repository,
            filters,
            &[CommitType::Feature, CommitType::Test],
        )?;

        // Assert
        assert_that!(logs).does_not_contain("Errored commit:");
        assert_that!(logs).does_not_contain("Commit message: 'I am afraid I can't do that Dave'");
        assert_that!(logs).does_not_contain("Missing commit type separator `:`");

        Ok(())
    }
}
