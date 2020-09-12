use crate::error::CocoGittoError::GitError;
use anyhow::Result;
use colored::Colorize;
use git2::{
    Commit as Git2Commit, DiffOptions, Object, ObjectType, Oid, Repository as Git2Repository,
};
use std::path::Path;

/// A wrapper around `git2::Repository` this is used only for
/// unitary operation on the repository.
pub struct Repository(pub(crate) Git2Repository);

impl Repository {
    pub(crate) fn open() -> Result<Repository> {
        let repo = Git2Repository::discover(".")?;
        Ok(Repository(repo))
    }

    pub(crate) fn get_repo_dir(&self) -> Option<&Path> {
        self.0.workdir()
    }

    pub(crate) fn commit(&self, message: String) -> Result<Oid> {
        let repo = &self.0;
        let sig = &&self.0.signature()?;
        let tree_id = &&self.0.index()?.write_tree()?;
        let tree = self.0.find_tree(**tree_id)?;
        let repo_is_empty = self.0.is_empty()?;
        let mut options = DiffOptions::new();

        let diff = if let Some(head) = &self.get_head() {
            repo.diff_tree_to_index(head.as_tree(), None, Some(&mut options))
        } else {
            repo.diff_tree_to_workdir_with_index(None, Some(&mut options))
        };

        let repo_has_deltas = if let Ok(diff) = diff {
            let deltas = diff.deltas();
            deltas.len() != 0
        } else {
            false
        };

        if !repo_is_empty && repo_has_deltas {
            let head = &self.0.head()?;
            let head_target = head.target().expect("Cannot get HEAD target");
            let tip = &self.0.find_commit(head_target)?;

            self.0
                .commit(Some("HEAD"), &sig, &sig, &message, &tree, &[&tip])
                .map_err(|err| anyhow!(err))
        } else if repo_is_empty && repo_has_deltas {
            // First repo commit
            self.0
                .commit(Some("HEAD"), &sig, &sig, &message, &tree, &[])
                .map_err(|err| anyhow!(err))
        } else {
            let statuses = repo.statuses(None)?;
            statuses.iter().for_each(|status| {
                eprintln!("{} : {:?}", status.path().unwrap(), status.status());
            });

            Err(anyhow!("err"))
        }
    }
    pub(crate) fn get_current_branch_name(&self) -> Result<String> {
        let head = &self.0.head()?;
        let head = head.shorthand();
        let branch_name = head.expect("Cannot get HEAT").into();
        Ok(branch_name)
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
                anyhow!(GitError {
                    level: "Git error".to_string(),
                    cause: format!("{} {}", cause_key, err.message())
                })
            })
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
