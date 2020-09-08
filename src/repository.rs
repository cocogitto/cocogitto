use anyhow::Result;
use git2::{DiffOptions, Object, ObjectType, Oid, Repository as Git2Repository, Tag};
use std::path::Path;

/// A wrapper around `git2::Repository` this is used only for
/// unitary operation on the repository.
pub struct Repository(pub(crate) Git2Repository);

impl Repository {
    pub fn open() -> Result<Repository> {
        let repo = Git2Repository::discover(".")?;
        Ok(Repository(repo))
    }

    pub fn get_repo_dir(&self) -> Option<&Path> {
        self.0.workdir()
    }

    pub fn commit(&self, message: String) -> Result<()> {
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
                .map(|_| ())
                .map_err(|err| anyhow!(err))
        } else if repo_is_empty && repo_has_deltas {
            // First repo commit
            self.0
                .commit(Some("HEAD"), &sig, &sig, &message, &tree, &[])
                .map(|_| ())
                .map_err(|err| anyhow!(err))
        } else {
            let statuses = repo.statuses(None)?;
            statuses.iter().for_each(|status| {
                eprintln!("{} : {:?}", status.path().unwrap(), status.status());
            });

            Err(anyhow!("err"))
        }
    }
    pub fn get_current_branch_name(&self) -> Result<String> {
        let head = &self.0.head()?;
        let head = head.shorthand();
        let branch_name = head.expect("Cannot get HEAT").into();
        Ok(branch_name)
    }

    pub fn get_head_commit_oid(&self) -> Result<Oid> {
        println!("get head oid");

        Ok(self.0.head().unwrap().peel_to_commit().unwrap().id())
    }

    pub fn resolve_lightweight_tag(&self, tag: &str) -> Result<Oid> {
        println!("tag: {}", tag);
        self.0
            .resolve_reference_from_short_name(tag)
            .map(|reference| reference.target().unwrap())
            .map_err(|err| anyhow!("Cannot resolve tag {} : {}", tag, err.message()))
    }

    pub fn get_latest_tag(&self) -> Result<Oid> {
        println!("get tag latest");
        let tag_names = self.0.tag_names(None)?;

        let tags = tag_names.iter().collect::<Vec<Option<&str>>>();

        if let Some(Some(tag)) = tags.last() {
            self.resolve_lightweight_tag(tag)
        } else {
            Err(anyhow!("Unable to get any tag"))
        }
    }

    pub fn get_first_commit(&self) -> Result<Oid> {
        let mut revwalk = self.0.revwalk()?;
        revwalk.push_head()?;
        revwalk
            .last()
            .ok_or(anyhow!("Could not find commit"))?
            .map_err(|err| anyhow!(err))
    }

    fn get_head(&self) -> Option<Object> {
        if let Ok(head) = Repository::tree_to_treeish(&self.0, Some(&"HEAD".to_string())) {
            head
        } else {
            None
        }
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
