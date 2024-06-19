use anyhow::anyhow;
use anyhow::Result;
use cocogitto::CogCommand;
use cocogitto_commit::{Commit, ConventionalCommitError};
use cocogitto_git::tag::TagLookUpOptions;
use cocogitto_oid::OidOf;
use colored::*;
use log::info;
use std::fmt;
use std::fmt::{Display, Formatter};

pub struct CogCheckCommand {
    from_latest_tag: bool,
    ignore_merge_commits: bool,
    range: Option<String>,
}

impl CogCheckCommand {
    pub fn try_new(
        check_from_latest_tag: bool,
        ignore_merge_commits: bool,
        range: Option<String>,
    ) -> Result<Self> {
        let path = Self::default_path()?;
        let settings = Self::settings(path.as_path())?;
        let from_latest_tag = check_from_latest_tag || settings.from_latest_tag;
        let ignore_merge_commits = ignore_merge_commits || settings.ignore_merge_commits;
        Ok(Self {
            from_latest_tag,
            ignore_merge_commits,
            range,
        })
    }
}

impl CogCommand for CogCheckCommand {
    fn execute(self) -> Result<()> {
        let repository = &Self::repository()?;
        let path = Self::default_path()?;
        let settings = &Self::settings(path.as_path())?;
        let commit_range = if let Some(range) = &self.range {
            repository.revwalk(range)?
        } else if self.from_latest_tag {
            let tag = repository.get_latest_tag(TagLookUpOptions::default())?;
            repository.revwalk(&format!("{tag}.."))?
        } else {
            repository.revwalk("..")?
        };

        let errors: Vec<_> = if self.ignore_merge_commits {
            commit_range
                .iter_commits()
                .filter(|commit| commit.parent_count() <= 1)
                .map(|commit| Commit::from_git_commit(commit, &settings.allowed_commit_types()))
                .filter_map(Result::err)
                .collect()
        } else {
            commit_range
                .iter_commits()
                .map(|commit| Commit::from_git_commit(commit, &settings.allowed_commit_types()))
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

#[derive(Debug)]
pub(crate) struct CogCheckReport {
    pub from: OidOf,
    pub errors: Vec<ConventionalCommitError>,
}

impl Display for CogCheckReport {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        let header = format!(
            "\nFound {} non compliant commits in {}..HEAD:\n",
            self.errors.len(),
            self.from
        )
        .red()
        .bold();

        writeln!(f, "{header}")?;

        for err in &self.errors {
            let underline = format!("{:>57}", " ").underline();
            writeln!(f, "{underline:>5}\n")?;
            write!(f, "{err}")?;
        }
        Ok(())
    }
}

#[cfg(test)]
mod test {
    use crate::CogCheckCommand;
    use cmd_lib::run_cmd;
    use cocogitto::CogCommand;
    use cocogitto_test_helpers::{
        commit, create_empty_config, git_add, git_commit, git_init, git_init_and_set_current_path,
        git_tag,
    };
    use sealed_test::prelude::*;
    use speculoos::prelude::*;

    #[sealed_test]
    fn check_commit_history_ok() -> anyhow::Result<()> {
        // Arrange
        git_init()?;
        git_commit("feat: a valid commit")?;
        git_commit("chore(test): another valid commit")?;

        // Act
        let check = CogCheckCommand::try_new(false, false, None)?.execute();

        // Assert
        assert_that!(check).is_ok();
        Ok(())
    }

    #[sealed_test]
    fn check_commit_history_err_with_merge_commit() -> anyhow::Result<()> {
        // Arrange
        git_init()?;
        git_commit("feat: a valid commit")?;
        git_commit("Merge feature one into main")?;

        // Act
        let check = CogCheckCommand::try_new(false, false, None)?.execute();

        // Assert
        assert_that!(check).is_err();
        Ok(())
    }

    #[sealed_test]
    fn check_commit_history_ok_with_merge_commit_ignored() -> anyhow::Result<()> {
        // Arrange
        git_init()?;
        git_commit("feat: a valid commit")?;
        run_cmd!(git checkout -b branch;)?;
        git_commit("feat: commit on another branch")?;
        run_cmd!(
            git checkout -;
            git merge branch --no-ff;
            git --no-pager log;
        )?;

        // Act
        let check = CogCheckCommand::try_new(false, true, None)?.execute();

        // Assert
        assert_that!(check).is_ok();
        Ok(())
    }

    #[sealed_test]
    fn check_commit_history_err() -> anyhow::Result<()> {
        // Arrange
        git_init_and_set_current_path("commit_history_err")?;
        create_empty_config()?;
        git_commit("feat: a valid commit")?;
        git_commit("errored commit")?;

        // Act
        let check = CogCheckCommand::try_new(false, false, None)?.execute();

        // Assert
        assert_that!(check).is_err();
        Ok(())
    }

    #[sealed_test]
    fn check_commit_ok_from_latest_tag() -> anyhow::Result<()> {
        // Arrange
        git_init_and_set_current_path("commit_ok_from_tag")?;

        create_empty_config()?;
        git_commit("this one should not be picked")?;
        git_tag("0.1.0")?;
        git_commit("feat: another commit")?;

        // Act
        let check = CogCheckCommand::try_new(true, false, None)?.execute();

        // Assert
        assert_that!(check).is_ok();
        Ok(())
    }

    #[sealed_test]
    fn check_commit_err_from_latest_tag() -> anyhow::Result<()> {
        // Arrange
        git_init_and_set_current_path("commit_err_from_tag")?;
        create_empty_config()?;
        git_commit("this one should not be picked")?;
        git_tag("0.1.0")?;
        git_commit("Oh no!")?;

        // Act
        let check = CogCheckCommand::try_new(true, false, None)?.execute();

        // Assert
        assert_that!(check).is_err();
        Ok(())
    }

    #[sealed_test]
    fn long_commit_summary_does_not_panic() -> anyhow::Result<()> {
        git_init()?;
        let message =
            "aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa…"
                .to_string();

        git_add("Hello", "file")?;
        commit(&format!("feat: {message}"))?;

        let check = CogCheckCommand::try_new(false, false, None)?.execute();

        assert_that!(check.is_ok());
        Ok(())
    }

    #[sealed_test]
    fn check_commit_ok_commit_range() -> anyhow::Result<()> {
        // Arrange
        git_init()?;
        let range_start = git_commit("feat: a valid commit")?;
        let range_end = git_commit("chore(test): another valid commit")?;
        let range = format!("{range_start}..{range_end}");

        // Act
        let check = CogCheckCommand::try_new(true, false, Some(range))?.execute();

        // Assert
        assert_that!(check).is_ok();
        Ok(())
    }

    #[sealed_test]
    fn check_commit_err_commit_range() -> anyhow::Result<()> {
        // Arrange
        git_init_and_set_current_path("commit_history_err")?;
        create_empty_config()?;
        let range_start = git_commit("feat: a valid commit")?;
        let range_end = git_commit("errored commit")?;
        let range = format!("{range_start}..{range_end}");

        // Act
        let check = CogCheckCommand::try_new(true, false, Some(range))?.execute();

        // Assert
        assert_that!(check).is_err();
        Ok(())
    }

    #[sealed_test]
    fn check_commit_range_err_with_merge_commit() -> anyhow::Result<()> {
        // Arrange
        git_init()?;
        let range_start = git_commit("feat: a valid commit")?;
        let range_end = git_commit("Merge feature one into main")?;
        let range = format!("{range_start}..{range_end}");

        // Act
        let check = CogCheckCommand::try_new(false, false, Some(range))?.execute();

        // Assert
        assert_that!(check).is_err();
        Ok(())
    }

    #[sealed_test]
    fn check_commit_range_ok_with_merge_commit_ignored() -> anyhow::Result<()> {
        // Arrange
        git_init()?;
        let range_start = git_commit("feat: a valid commit")?;
        run_cmd!(git checkout -b branch;)?;
        git_commit("feat: commit on another branch")?;
        run_cmd!(
            git checkout -;
            git merge branch --no-ff;
            git --no-pager log;
        )?;
        let range_end = git_commit("chore(test): another valid commit")?;
        let range = format!("{range_start}..{range_end}");

        // Act
        let check = CogCheckCommand::try_new(false, true, Some(range))?.execute();

        // Assert
        assert_that!(check).is_ok();
        Ok(())
    }
}
