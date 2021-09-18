use std::path::Path;

use anyhow::Result;
use colored::Colorize;
use git2::{
    Commit as Git2Commit, Diff, DiffOptions, IndexAddOption, Object, ObjectType, Oid,
    Repository as Git2Repository, StatusOptions,
};
use itertools::Itertools;
use semver::Version;

use crate::error::CocogittoError;
use crate::error::CocogittoError::Git;
use crate::OidOf;

use super::status::Statuses;

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

    pub(crate) fn get_diff(&self, include_untracked: bool) -> Option<Diff> {
        let mut options = DiffOptions::new();
        options.include_untracked(include_untracked);

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
        index.add_all(["*"], IndexAddOption::DEFAULT, None)?;
        index.write().map_err(|err| anyhow!(err))
    }

    pub(crate) fn commit(&self, message: &str) -> Result<Oid> {
        let sig = self.0.signature()?;
        let tree_id = self.0.index()?.write_tree()?;
        let tree = self.0.find_tree(tree_id)?;
        let is_empty = self.0.is_empty()?;
        let has_delta = self.get_diff(false).is_some();

        if !is_empty && has_delta {
            let head = &self.0.head()?;
            let head_target = head.target().expect("Cannot get HEAD target");
            let tip = &self.0.find_commit(head_target)?;

            self.0
                .commit(Some("HEAD"), &sig, &sig, message, &tree, &[tip])
                .map_err(|err| anyhow!(err))
        } else if is_empty && has_delta {
            // First repo commit
            self.0
                .commit(Some("HEAD"), &sig, &sig, message, &tree, &[])
                .map_err(|err| anyhow!(err))
        } else {
            let err = self
                .get_branch_shorthand()
                .map(|branch| CocogittoError::NothingToCommitWithBranch { branch })
                .unwrap_or_else(|| CocogittoError::NothingToCommit);

            Err(anyhow!(err))
        }
    }

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

    pub(crate) fn get_head_commit_oid(&self) -> Result<Oid> {
        self.get_head_commit().map(|commit| commit.id())
    }

    pub(crate) fn get_head_commit(&self) -> Result<Git2Commit> {
        let head_ref = self.0.head();
        match head_ref {
            Ok(head) => head
                .peel_to_commit()
                .map_err(|err| anyhow!("Could not peel head to commit {}", err)),
            Err(err) => Err(anyhow!("Repo as not HEAD : {}", err)),
        }
    }

    pub(crate) fn resolve_lightweight_tag(&self, tag: &str) -> Result<Oid> {
        self.0
            .resolve_reference_from_short_name(tag)
            .map(|reference| reference.target().unwrap())
            .map_err(|err| anyhow!("Cannot resolve tag {} : {}", tag, err.message()))
    }

    pub(crate) fn get_latest_tag(&self) -> Result<String> {
        let tag_names = self.0.tag_names(None)?;
        let latest_tag = tag_names.iter().flatten().flat_map(Version::parse).max();

        match latest_tag {
            Some(tag) => Ok(tag.to_string()),
            None => Err(anyhow!("Unable to get any tag")),
        }
    }

    pub(crate) fn get_tag_commits(&self, target_tag: &str) -> Result<(OidOf, OidOf)> {
        let tag_names = self.0.tag_names(None)?;
        let oid_of_target_tag = OidOf::Tag(
            target_tag.to_string(),
            self.resolve_lightweight_tag(target_tag)?,
        );
        // Starting point is set to first commit
        let oid_of_first_commit = OidOf::Other(self.get_first_commit()?);

        let oid_of_previous_tag = tag_names
            .iter()
            .flatten()
            .sorted()
            .tuple_windows()
            .find(|&(_, cur)| cur == target_tag)
            .map(|(prev, _)| {
                OidOf::Tag(
                    prev.to_string(),
                    self.resolve_lightweight_tag(prev)
                        .expect("Unexpected tag parsing error"),
                )
            })
            .unwrap_or(oid_of_first_commit);

        Ok((oid_of_previous_tag, oid_of_target_tag))
    }

    pub(crate) fn get_latest_tag_oid(&self) -> Result<Oid> {
        self.get_latest_tag()
            .and_then(|tag| self.resolve_lightweight_tag(&tag))
            .map_err(|err| anyhow!("Could not resolve latest tag : {}", err))
    }

    pub(crate) fn get_latest_tag_oidof(&self) -> Result<OidOf> {
        self.get_latest_tag()
            .and_then(|tag| {
                self.resolve_lightweight_tag(&tag)
                    .map(|oid| OidOf::Tag(tag, oid))
            })
            .map_err(|err| anyhow!("Could not resolve latest tag : {}", err))
    }

    pub(crate) fn get_first_commit_oidof(&self) -> Result<OidOf> {
        self.get_first_commit().map(OidOf::Other)
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
        Repository::tree_to_treeish(&self.0, Some(&"HEAD".to_string()))
            .ok()
            .flatten()
    }

    pub(crate) fn get_branch_shorthand(&self) -> Option<String> {
        self.0
            .head()
            .ok()
            .and_then(|head| head.shorthand().map(|shorthand| shorthand.to_string()))
    }

    pub(crate) fn create_tag(&self, name: &str) -> Result<()> {
        ensure!(
            self.get_diff(true).is_none(),
            "{}{}",
            self.get_statuses()?,
            "Cannot create tag: changes need to be committed".red()
        );

        let head = self.get_head_commit().unwrap();
        self.0
            .tag_lightweight(name, &head.into_object(), false)
            .map(|_| ())
            .map_err(|err| {
                let cause_key = "cause:".red();
                anyhow!(Git {
                    level: "Git error".to_string(),
                    cause: format!("{} {}", cause_key, err.message())
                })
            })
    }

    pub(crate) fn stash_failed_version(&mut self, version: &str) -> Result<()> {
        let sig = self.0.signature()?;
        let message = &format!("cog_bump_{}", version);
        self.0
            .stash_save(&sig, message, None)
            .map(|_| ())
            .map_err(|err| anyhow!(err))
    }

    pub(crate) fn get_commit_range(&self, from: Oid, to: Oid) -> Result<Vec<Git2Commit>> {
        // Ensure commit exists
        self.0.find_commit(from)?;
        self.0.find_commit(to)?;

        let mut revwalk = self.0.revwalk()?;

        revwalk.push_range(&format!("{}..{}", from, to))?;

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

#[cfg(test)]
mod test {
    use std::ops::Not;
    use std::process::{Command, Stdio};

    use anyhow::Result;
    use tempfile::TempDir;

    use crate::OidOf;

    use super::Repository;

    #[test]
    fn init_repo() -> Result<()> {
        let tmp = TempDir::new()?;

        let repo = Repository::init(&tmp.path().join("test_repo"));

        assert!(repo.is_ok());
        Ok(())
    }

    #[test]
    fn create_commit_ok() -> Result<()> {
        let tmp = TempDir::new()?;

        let path = tmp.path().join("test_repo");
        let repo = Repository::init(&path)?;
        std::fs::write(&path.join("file"), "changes")?;
        repo.add_all()?;

        assert!(repo.commit("feat: a test commit").is_ok());
        Ok(())
    }

    #[test]
    fn not_create_empty_commit() -> Result<()> {
        let tmp = TempDir::new()?;

        let path = tmp.path().join("test_repo");
        let repo = Repository::init(&path)?;

        assert!(repo.commit("feat: a test commit").is_err());
        Ok(())
    }

    #[test]
    fn not_create_empty_commit_with_unstaged_changed() -> Result<()> {
        let tmp = TempDir::new()?;

        let path = tmp.path().join("test_repo");
        let repo = Repository::init(&path)?;
        std::fs::write(&path.join("file"), "changes")?;

        assert!(repo.commit("feat: a test commit").is_err());
        Ok(())
    }

    #[test]
    fn get_repo_working_dir_some() -> Result<()> {
        let tmp = TempDir::new()?;

        let path = tmp.path().join("test_repo");
        let repo = Repository::init(&path)?;
        let dir = &path.join("dir");
        std::fs::create_dir(dir)?;
        std::env::set_current_dir(dir)?;

        assert_eq!(repo.get_repo_dir(), Some(path.as_path()));
        Ok(())
    }

    #[test]
    fn get_diff_some() -> Result<()> {
        let tmp = TempDir::new()?;

        let path = tmp.path().join("test_repo");
        let repo = Repository::init(&path)?;
        std::fs::write(&path.join("file"), "changes")?;
        repo.add_all()?;

        assert!(repo.get_diff(false).is_some());
        Ok(())
    }

    #[test]
    fn get_diff_none() -> Result<()> {
        let tmp = TempDir::new()?;

        let path = tmp.path().join("test_repo");
        let repo = Repository::init(&path)?;
        std::fs::write(&path.join("file"), "changes")?;

        assert!(repo.get_diff(false).is_none());
        Ok(())
    }

    #[test]
    fn get_diff_include_untracked_some() -> Result<()> {
        let tmp = TempDir::new()?;

        let path = tmp.path().join("test_repo");
        let repo = Repository::init(&path)?;
        std::fs::write(&path.join("file"), "changes")?;

        assert!(repo.get_diff(true).is_some());
        Ok(())
    }

    #[test]
    fn get_diff_include_untracked_none() -> Result<()> {
        let tmp = TempDir::new()?;

        let path = tmp.path().join("test_repo");
        let repo = Repository::init(&path)?;

        assert!(repo.get_diff(true).is_none());
        Ok(())
    }

    // see: https://git-scm.com/book/en/v2/Git-on-the-Server-Getting-Git-on-a-Server
    #[test]
    fn open_bare_err() -> Result<()> {
        let tmp = TempDir::new()?;
        std::env::set_current_dir(&tmp)?;

        let tmp = &tmp.path().to_str().unwrap();

        Command::new("git")
            .arg("init")
            .arg("bare")
            .arg(tmp)
            .stdout(Stdio::inherit())
            .output()?;

        let repo = Repository::open(tmp);

        assert!(repo.is_err());
        Ok(())
    }

    #[test]
    fn get_repo_statuses_empty() -> Result<()> {
        let tmp = TempDir::new()?;

        let path = tmp.path().join("test_repo");
        let repo = Repository::init(&path)?;

        let statuses = repo.get_statuses()?;

        assert!(statuses.0.is_empty());
        Ok(())
    }

    #[test]
    fn get_repo_statuses_not_empty() -> Result<()> {
        let tmp = TempDir::new()?;

        let path = tmp.path().join("test_repo");
        let repo = Repository::init(&path)?;
        std::fs::write(&path.join("file"), "changes")?;

        let statuses = repo.get_statuses()?;

        assert!(statuses.0.is_empty().not());
        Ok(())
    }

    #[test]
    fn get_repo_head_oid_ok() -> Result<()> {
        let tmp = TempDir::new()?;

        let path = tmp.path().join("test_repo");
        let repo = Repository::init(&path)?;
        std::fs::write(&path.join("file"), "changes")?;
        repo.add_all()?;
        let commit_oid = repo.commit("first commit")?;

        let oid = repo.get_head_commit_oid();

        assert!(oid.is_ok());
        assert_eq!(oid.unwrap(), commit_oid);
        Ok(())
    }

    #[test]
    fn get_repo_head_oid_err() -> Result<()> {
        let tmp = TempDir::new()?;

        let path = tmp.path().join("test_repo");
        let repo = Repository::init(&path)?;

        let oid = repo.get_head_commit_oid();

        assert!(oid.is_err());
        Ok(())
    }

    #[test]
    fn get_repo_head_obj_ok() -> Result<()> {
        let tmp = TempDir::new()?;

        let path = tmp.path().join("test_repo");
        let repo = Repository::init(&path)?;
        std::fs::write(&path.join("file"), "changes")?;
        repo.add_all()?;
        let commit_oid = repo.commit("first commit")?;

        let head = repo.get_head_commit();

        assert!(head.is_ok());
        assert_eq!(head.unwrap().id(), commit_oid);
        Ok(())
    }

    #[test]
    fn get_repo_head_obj_err() -> Result<()> {
        let tmp = TempDir::new()?;

        let path = tmp.path().join("test_repo");
        let repo = Repository::init(&path)?;
        std::fs::write(&path.join("file"), "changes")?;
        repo.add_all()?;

        let head = repo.get_head_commit();

        assert!(head.is_err());
        Ok(())
    }

    #[test]
    fn resolve_lightweight_tag_ok() -> Result<()> {
        let tmp = TempDir::new()?;

        let path = tmp.path().join("test_repo");
        let repo = Repository::init(&path)?;
        std::fs::write(&path.join("file"), "changes")?;
        repo.add_all()?;
        repo.commit("first commit")?;
        repo.create_tag("the_tag")?;

        let tag = repo.resolve_lightweight_tag("the_tag");

        assert!(tag.is_ok());
        Ok(())
    }

    #[test]
    fn resolve_lightweight_tag_err() -> Result<()> {
        let tmp = TempDir::new()?;

        let path = tmp.path().join("test_repo");
        let repo = Repository::init(&path)?;
        std::fs::write(&path.join("file"), "changes")?;
        repo.add_all()?;
        repo.commit("first commit")?;
        repo.create_tag("the_tag")?;

        let tag = repo.resolve_lightweight_tag("the_taaaag");

        assert!(tag.is_err());
        Ok(())
    }

    #[test]
    fn get_latest_tag_ok() -> Result<()> {
        let tmp = TempDir::new()?;

        let path = tmp.path().join("test_repo");
        let repo = Repository::init(&path)?;
        std::fs::write(&path.join("file"), "changes")?;
        repo.add_all()?;
        repo.commit("first commit")?;
        repo.create_tag("0.1.0")?;

        std::fs::write(&path.join("file"), "changes2")?;
        repo.add_all()?;
        repo.commit("second commit")?;
        repo.create_tag("0.2.0")?;

        let tag = repo.get_latest_tag();

        assert!(tag.is_ok());
        assert_eq!(tag.unwrap(), "0.2.0");
        Ok(())
    }

    #[test]
    fn get_latest_tag_err() -> Result<()> {
        let tmp = TempDir::new()?;

        let path = tmp.path().join("test_repo");
        let repo = Repository::init(&path)?;
        std::fs::write(&path.join("file"), "changes")?;
        repo.add_all()?;
        repo.commit("first commit")?;

        let tag = repo.get_latest_tag();

        assert!(tag.is_err());
        Ok(())
    }

    #[test]
    fn get_latest_tag_oid_ok() -> Result<()> {
        let tmp = TempDir::new()?;

        let path = tmp.path().join("test_repo");
        let repo = Repository::init(&path)?;
        std::fs::write(&path.join("file"), "changes")?;
        repo.add_all()?;
        repo.commit("first commit")?;
        repo.create_tag("1.0.0")?;

        let tag = repo.get_latest_tag_oid();

        assert!(tag.is_ok());
        Ok(())
    }

    #[test]
    fn get_latest_tag_oid_err() -> Result<()> {
        let tmp = TempDir::new()?;

        let path = tmp.path().join("test_repo");
        let repo = Repository::init(&path)?;
        std::fs::write(&path.join("file"), "changes")?;
        repo.add_all()?;
        repo.commit("first commit")?;

        let tag = repo.get_latest_tag_oid();

        assert!(tag.is_err());
        Ok(())
    }

    #[test]
    fn get_head_some() -> Result<()> {
        let tmp = TempDir::new()?;

        let path = tmp.path().join("test_repo");
        let repo = Repository::init(&path)?;
        std::fs::write(&path.join("file"), "changes")?;
        repo.add_all()?;
        repo.commit("first commit")?;

        let tag = repo.get_head();

        assert!(tag.is_some());
        Ok(())
    }

    #[test]
    fn get_head_none() -> Result<()> {
        let tmp = TempDir::new()?;

        let path = tmp.path().join("test_repo");
        let repo = Repository::init(&path)?;
        std::fs::write(&path.join("file"), "changes")?;
        repo.add_all()?;

        let tag = repo.get_head();

        assert!(tag.is_none());
        Ok(())
    }

    #[test]
    fn get_tag_commits() -> Result<()> {
        let tmp = TempDir::new()?;

        let path = tmp.path().join("test_repo");
        let repo = Repository::init(&path)?;
        std::fs::write(&path.join("file"), "changes")?;
        repo.add_all()?;
        let start = repo.commit("chore: init")?;

        std::fs::write(&path.join("file2"), "changes")?;
        repo.add_all()?;
        let end = repo.commit("chore: 1.0.0")?;

        repo.create_tag("1.0.0")?;

        std::fs::write(&path.join("file3"), "changes")?;
        repo.add_all()?;
        repo.commit("feat: a commit")?;

        let commit_range = repo.get_tag_commits("1.0.0")?;

        assert_eq!(
            commit_range,
            (OidOf::Other(start), OidOf::Tag("1.0.0".to_string(), end))
        );
        Ok(())
    }

    #[test]
    fn get_branch_short_hand() -> Result<()> {
        let tmp = TempDir::new()?;

        let path = tmp.path().join("test_repo");
        let repo = Repository::init(&path)?;
        std::fs::write(&path.join("file"), "changes")?;
        repo.add_all()?;
        repo.commit("hello one")?;

        let shorthand = repo.get_branch_shorthand();

        assert_eq!(shorthand, Some("master".to_string()));
        Ok(())
    }

    #[test]
    fn should_stash_failed_bump() -> Result<()> {
        let tmp = TempDir::new()?;

        let path = tmp.path().join("test_repo");
        let mut repo = Repository::init(&path)?;
        std::fs::write(&path.join("file"), "changes")?;
        repo.add_all()?;
        repo.commit("Initial commit")?;

        let statuses = repo.get_statuses()?.0;
        assert!(statuses.is_empty());

        std::fs::write(&path.join("second_file"), "more changes")?;
        repo.add_all()?;
        let statuses = repo.get_statuses()?.0;
        assert_eq!(statuses.len(), 1);

        repo.stash_failed_version("1.0.0")?;

        let statuses = repo.get_statuses()?.0;
        assert!(statuses.is_empty());
        Ok(())
    }
}
