use std::fmt::{self, Formatter};

use crate::git::status::Changes::{Deleted, Modified, New, Renamed, TypeChange};

use crate::git::repository::Repository;
use anyhow::{anyhow, Result};
use colored::*;
use git2::Statuses as Git2Statuses;
use git2::{StatusEntry as Git2StatusEntry, StatusOptions};

impl Repository {
    pub(crate) fn get_statuses(&self) -> Result<Statuses> {
        let mut options = StatusOptions::new();
        options.include_untracked(true);
        options.exclude_submodules(true);
        options.include_unmodified(false);

        let statuses = self
            .0
            .statuses(Some(&mut options))
            .map_err(|err| anyhow!(err))?;

        Ok(Statuses::from(statuses))
    }
}

pub(crate) struct Statuses(pub Vec<Status>);

#[derive(Debug, Eq, PartialEq)]
pub(crate) enum Status {
    Untracked(Changes),
    UnCommitted(Changes),
}

#[derive(Debug, Eq, PartialEq)]
pub(crate) enum Changes {
    New(String),
    Renamed(String),
    Deleted(String),
    TypeChange(String),
    Modified(String),
}

impl Changes {
    fn to_string(&self, color: &str) -> String {
        match &self {
            New(p) => format!("{}: {}", "new: ".color(color), p),
            Renamed(p) => format!("{}: {}", "renamed: ".color(color), p),
            Deleted(p) => format!("{}: {}", "deleted: ".color(color), p),
            TypeChange(p) => format!("{}  {}", "type changed: ".color(color), p),
            Modified(p) => format!("{}: {}", "modified: ".color(color), p),
        }
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

impl fmt::Display for Statuses {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        let mut untracked = vec![];
        let mut uncommitted = vec![];

        self.0.iter().for_each(|status| {
            match status {
                Status::Untracked(changes) => untracked.push(changes),
                Status::UnCommitted(changes) => uncommitted.push(changes),
            };
        });

        if !untracked.is_empty() {
            writeln!(f, "Untracked files :").unwrap();
            untracked.iter().for_each(|change| {
                writeln!(f, "\t{}", change.to_string("red")).unwrap();
            });
            writeln!(f, "Use `git add` to track").unwrap();
        }

        if !untracked.is_empty() && !uncommitted.is_empty() {
            write!(f, "\n\n")?;
        }

        if !uncommitted.is_empty() {
            writeln!(f, "Changes to be committed :").unwrap();
            uncommitted.iter().for_each(|change| {
                writeln!(f, "\t{}", change.to_string("green")).unwrap();
            });
            writeln!(f, "Use `coco <type>` to commit changes").unwrap();
        }

        write!(f, "\n\n")
    }
}

impl fmt::Debug for Statuses {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "{}", &self)
    }
}

#[cfg(test)]
mod test {
    use std::fs;

    use crate::git::status::{Changes, Statuses};

    use crate::git::repository::Repository;
    use crate::test_helpers::run_test_with_context;
    use anyhow::{anyhow, Result};
    use git2::StatusOptions;
    use speculoos::prelude::*;

    #[test]
    fn get_repo_statuses_empty() -> Result<()> {
        run_test_with_context(|context| {
            let repo = Repository::init(&context.test_dir)?;

            let statuses = repo.get_statuses()?;

            assert_that!(statuses.0).has_length(0);
            Ok(())
        })
    }

    #[test]
    fn get_repo_statuses_not_empty() -> Result<()> {
        run_test_with_context(|context| {
            let repo = Repository::init(&context.test_dir)?;
            std::fs::write(context.test_dir.join("file"), "changes")?;

            let statuses = repo.get_statuses()?;

            assert_that!(statuses.0).has_length(1);
            Ok(())
        })
    }

    #[test]
    fn should_get_statuses_from_git_statuses() -> Result<()> {
        run_test_with_context(|context| {
            let path = context.test_dir.join("test_repo");
            let repo = Repository::init(&path)?;
            fs::write(path.join("file"), "content")?;

            let mut options = StatusOptions::new();
            options.include_untracked(true);
            options.exclude_submodules(true);
            options.include_unmodified(false);

            let git_statuses = repo
                .0
                .statuses(Some(&mut options))
                .map_err(|err| anyhow!(err))?;

            let statuses = Statuses::from(git_statuses).0;

            assert_that!(statuses.iter())
                .contains(&super::Status::Untracked(Changes::New("file".into())));
            assert_that!(statuses).has_length(1);
            Ok(())
        })
    }
}
