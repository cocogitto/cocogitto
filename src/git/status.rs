use std::fmt::{self, Display, Formatter};

use crate::git::status::Changes::{Deleted, Modified, New, Renamed, TypeChange};

use crate::git::error::Git2Error;
use crate::git::repository::Repository;
use colored::*;
use git2::Statuses as Git2Statuses;
use git2::{StatusEntry as Git2StatusEntry, StatusOptions};

impl Repository {
    pub(crate) fn get_statuses(&self) -> Result<Statuses, Git2Error> {
        let mut options = StatusOptions::new();
        options.include_untracked(true);
        options.exclude_submodules(true);
        options.include_unmodified(false);

        let statuses = self
            .0
            .statuses(Some(&mut options))
            .map_err(Git2Error::StatusError)?;

        Ok(Statuses::from(statuses))
    }
}

#[derive(Debug)]
pub struct Statuses(pub Vec<Status>);

#[derive(Debug, Eq, PartialEq)]
pub enum Status {
    Untracked(Changes),
    UnCommitted(Changes),
}

#[derive(Debug, Eq, PartialEq)]
pub enum Changes {
    New(String),
    Renamed(String),
    Deleted(String),
    TypeChange(String),
    Modified(String),
}

impl Changes {
    pub(crate) fn to_string(&self, color: &str) -> String {
        match &self {
            New(p) => format!("{}: {}", "new".color(color), p),
            Renamed(p) => format!("{}: {}", "renamed".color(color), p),
            Deleted(p) => format!("{}: {}", "deleted".color(color), p),
            TypeChange(p) => format!("{}:  {}", "type changed".color(color), p),
            Modified(p) => format!("{}: {}", "modified".color(color), p),
        }
    }
}

impl Display for Statuses {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        let mut untracked = vec![];
        let mut uncommitted = vec![];

        self.0.iter().for_each(|status| {
            match status {
                Status::Untracked(changes) => untracked.push(changes),
                Status::UnCommitted(changes) => uncommitted.push(changes),
            };
        });

        let has_untracked_changes = !untracked.is_empty();
        let has_uncommitted_changes = !uncommitted.is_empty();

        if has_untracked_changes {
            writeln!(f, "Untracked files :")?;
            for change in untracked {
                writeln!(f, "\t{}", change.to_string("red"))?;
            }
            writeln!(
                f,
                "\nnothing added to commit but untracked files present (use \"git add\" to track)"
            )?;
        }

        if has_untracked_changes && has_uncommitted_changes {
            writeln!(f)?;
        }

        if has_uncommitted_changes {
            writeln!(f, "Changes to be committed :")?;
            writeln!(
                f,
                "(use \"git add <file>...\" to include in what will be committed)"
            )?;
            for change in uncommitted {
                writeln!(f, "\t{}", change.to_string("green"))?;
            }
            writeln!(f, "\nUse `cog commit <type>` to commit changes")?;
        }
        fmt::Result::Ok(())
    }
}

impl From<Git2Statuses<'_>> for Statuses {
    fn from(statuses: Git2Statuses) -> Self {
        Self(statuses.iter().map(Status::from).collect())
    }
}

impl<'a, 'b: 'a> From<Git2StatusEntry<'b>> for Status {
    fn from(status: Git2StatusEntry<'b>) -> Self {
        let path = status.path().unwrap_or("invalid utf8 path").to_string();
        match status.status() {
            s if s.is_wt_new() => Status::Untracked(New(path)),
            s if s.is_wt_renamed() => Status::Untracked(Renamed(path)),
            s if s.is_wt_deleted() => Status::Untracked(Deleted(path)),
            s if s.is_wt_typechange() => Status::Untracked(TypeChange(path)),
            s if s.is_wt_modified() => Status::Untracked(Modified(path)),
            s if s.is_index_new() => Status::UnCommitted(New(path)),
            s if s.is_index_modified() => Status::UnCommitted(Modified(path)),
            s if s.is_index_deleted() => Status::UnCommitted(Deleted(path)),
            s if s.is_index_renamed() => Status::UnCommitted(Renamed(path)),
            s if s.is_index_typechange() => Status::UnCommitted(TypeChange(path)),
            _ => unreachable!(),
        }
    }
}

#[cfg(test)]
mod test {
    use std::fs;

    use crate::git::status::{Changes, Statuses};

    use crate::git::repository::Repository;
    use anyhow::{anyhow, Result};
    use git2::StatusOptions;
    use sealed_test::prelude::*;
    use speculoos::prelude::*;

    #[sealed_test]
    fn get_repo_statuses_empty() -> Result<()> {
        // Arrange
        let repo = Repository::init(".")?;

        // Act
        let statuses = repo.get_statuses()?;

        // Assert
        assert_that!(statuses.0).has_length(0);
        Ok(())
    }

    #[sealed_test]
    fn get_repo_statuses_not_empty() -> Result<()> {
        // Arrange
        let repo = Repository::init(".")?;
        std::fs::write("file", "changes")?;

        // Act
        let statuses = repo.get_statuses()?;

        // Assert
        assert_that!(statuses.0).has_length(1);
        Ok(())
    }

    #[sealed_test]
    fn should_get_statuses_from_git_statuses() -> Result<()> {
        // Arrange
        let repo = Repository::init(".")?;
        fs::write("file", "content")?;

        let mut options = StatusOptions::new();
        options.include_untracked(true);
        options.exclude_submodules(true);
        options.include_unmodified(false);

        let git_statuses = repo
            .0
            .statuses(Some(&mut options))
            .map_err(|err| anyhow!(err))?;

        // Act
        let statuses = Statuses::from(git_statuses).0;

        // Assert
        assert_that!(statuses.iter())
            .contains(&super::Status::Untracked(Changes::New("file".into())));
        assert_that!(statuses).has_length(1);
        Ok(())
    }
}
