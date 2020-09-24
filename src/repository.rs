use crate::error::ErrorKind::Git;
use anyhow::Result;
use colored::Colorize;
use git2::{
    Commit as Git2Commit, Diff, DiffOptions, IndexAddOption, Object, ObjectType, Oid,
    Repository as Git2Repository, StatusOptions, Statuses,
};
use std::path::Path;

pub(crate) struct Repository(pub(crate) Git2Repository);

impl Repository {
    pub(crate) fn init<S: AsRef<Path> + ?Sized>(path: &S) -> Result<Repository> {
        Ok(Repository(Git2Repository::init(&path)?))
    }

    pub(crate) fn open<S: AsRef<Path> + ?Sized>(path: &S) -> Result<Repository> {
        let repo = Git2Repository::discover(&path)?;
        Ok(Repository(repo))
    }

    pub(crate) fn get_repo_dir(&self) -> Option<&Path> {
        self.0.workdir()
    }

    pub(crate) fn get_diff(&self) -> Option<Diff> {
        let mut options = DiffOptions::new();

        let diff = if let Some(head) = &self.get_head() {
            self.0
                .diff_tree_to_index(head.as_tree(), None, Some(&mut options))
        } else {
            self.0
                .diff_tree_to_workdir_with_index(None, Some(&mut options))
        };

        if let Ok(diff) = diff {
            if diff.deltas().len() > 0 {
                Some(diff)
            } else {
                None
            }
        } else {
            None
        }
    }

    pub(crate) fn add_all(&self) -> Result<()> {
        let mut index = self.0.index()?;
        index.add_all(["*"].iter(), IndexAddOption::DEFAULT, None)?;
        index.write().map_err(|err| anyhow!(err))
    }

    pub(crate) fn commit(&self, message: String) -> Result<Oid> {
        let sig = self.0.signature()?;
        let tree_id = self.0.index()?.write_tree()?;
        let tree = self.0.find_tree(tree_id)?;
        let is_empty = self.0.is_empty()?;
        let has_delta = self.get_diff().is_some();

        if !is_empty && has_delta {
            let head = &self.0.head()?;
            let head_target = head.target().expect("Cannot get HEAD target");
            let tip = &self.0.find_commit(head_target)?;

            self.0
                .commit(Some("HEAD"), &sig, &sig, &message, &tree, &[&tip])
                .map_err(|err| anyhow!(err))
        } else if is_empty && has_delta {
            // First repo commit
            self.0
                .commit(Some("HEAD"), &sig, &sig, &message, &tree, &[])
                .map_err(|err| anyhow!(err))
        } else {
            let statuses = self.get_statuses()?;
            statuses.iter().for_each(|entry| {
                let status = match entry.status() {
                    s if s.contains(git2::Status::WT_NEW) => "Untracked: ",
                    s if s.contains(git2::Status::WT_RENAMED) => "Renamed: ",
                    s if s.contains(git2::Status::WT_DELETED) => "Deleted: ",
                    s if s.contains(git2::Status::WT_TYPECHANGE) => "Typechange: ",
                    s if s.contains(git2::Status::WT_MODIFIED) => "Modified: ",
                    s if s.contains(git2::Status::INDEX_NEW) => "New file: ",
                    s if s.contains(git2::Status::INDEX_MODIFIED) => "Modified: ",
                    s if s.contains(git2::Status::INDEX_DELETED) => "Deleted: ",
                    s if s.contains(git2::Status::INDEX_RENAMED) => "Renamed: ",
                    s if s.contains(git2::Status::INDEX_TYPECHANGE) => "Typechange:",
                    _ => "unknown git status",
                };
                println!("{} {}", status.red(), entry.path().unwrap());
            });
            println!();

            Err(anyhow!("nothing to commit (use \"git add\" to track)"))
        }
    }

    pub(crate) fn get_statuses(&self) -> Result<Statuses> {
        let mut options = StatusOptions::new();
        options.include_untracked(true);
        options.exclude_submodules(true);
        options.include_unmodified(false);

        self.0
            .statuses(Some(&mut options))
            .map_err(|err| anyhow!(err))
    }

    pub(crate) fn get_head_commit_oid(&self) -> Result<Oid> {
        self.get_head_object().map(|commit| commit.id())
    }

    pub(crate) fn get_head_object(&self) -> Result<Git2Commit> {
        Ok(self.0.head().unwrap().peel_to_commit().unwrap())
    }

    pub(crate) fn resolve_lightweight_tag(&self, tag: &str) -> Result<Oid> {
        self.0
            .resolve_reference_from_short_name(tag)
            .map(|reference| reference.target().unwrap())
            .map_err(|err| anyhow!("Cannot resolve tag {} : {}", tag, err.message()))
    }

    pub(crate) fn get_latest_tag(&self) -> Result<String> {
        let tag_names = self.0.tag_names(None)?;

        let tags = tag_names.iter().collect::<Vec<Option<&str>>>();
        if let Some(Some(tag)) = tags.last() {
            Ok(tag.to_string())
        } else {
            Err(anyhow!("Unable to get any tag"))
        }
    }

    pub(crate) fn get_latest_tag_oid(&self) -> Result<Oid> {
        self.get_latest_tag()
            .and_then(|oid| self.resolve_lightweight_tag(&oid))
            .map_err(|err| anyhow!("Could not resolve latest tag : {}", err))
    }

    pub(crate) fn get_first_commit(&self) -> Result<Oid> {
        let mut revwalk = self.0.revwalk()?;
        revwalk.push_head()?;
        revwalk
            .last()
            .ok_or_else(|| anyhow!("Could not find commit"))?
            .map_err(|err| anyhow!(err))
    }

    pub(crate) fn get_head(&self) -> Option<Object> {
        if let Ok(head) = Repository::tree_to_treeish(&self.0, Some(&"HEAD".to_string())) {
            head
        } else {
            None
        }
    }

    pub(crate) fn create_tag(&self, name: &str) -> Result<()> {
        let head = self.get_head().unwrap();
        self.0
            .tag_lightweight(name, &head, false)
            .map(|_| ())
            .map_err(|err| {
                let cause_key = "cause:".red();
                anyhow!(Git {
                    level: "Git error".to_string(),
                    cause: format!("{} {}", cause_key, err.message())
                })
            })
    }

    pub(crate) fn get_commit_range(&self, from: Oid, to: Oid) -> Result<Vec<Git2Commit>> {
        // Ensure commit exists
        self.0.find_commit(from)?;
        self.0.find_commit(to)?;

        let mut revwalk = self.0.revwalk()?;
        revwalk.push(to)?;
        revwalk.push(from)?;

        let mut commits: Vec<Git2Commit> = vec![];

        for oid in revwalk {
            let oid = oid?;
            let commit = self.0.find_commit(oid)?;
            commits.push(commit);
        }

        Ok(commits)
    }

    pub(crate) fn get_author(&self) -> Result<String> {
        self.0
            .signature()?
            .name()
            .map(|name| name.to_string())
            .ok_or_else(|| anyhow!("Cannot get committer name"))
    }
    fn tree_to_treeish<'a>(
        repo: &'a Git2Repository,
        arg: Option<&String>,
    ) -> Result<Option<Object<'a>>, git2::Error> {
        let arg = match arg {
            Some(s) => s,
            None => return Ok(None),
        };
        let obj = repo.revparse_single(arg)?;
        let tree = obj.peel(ObjectType::Tree)?;
        Ok(Some(tree))
    }
}
