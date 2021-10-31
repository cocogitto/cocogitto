use std::fmt::{Debug, Formatter};
use std::path::Path;

use anyhow::{anyhow, Result};

use git2::{
    Commit as Git2Commit, IndexAddOption, Object, ObjectType, Oid, Repository as Git2Repository,
};

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

    pub(crate) fn add_all(&self) -> Result<()> {
        let mut index = self.0.index()?;
        index.add_all(["*"], IndexAddOption::DEFAULT, None)?;
        index.write().map_err(|err| anyhow!(err))
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
            Err(err) => Err(anyhow!("Repo as not HEAD:{}", err)),
        }
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

impl Debug for Repository {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "Repository {{ 0: {:?}}}", self.0.path())
    }
}

#[cfg(test)]
mod test {
    use std::process::{Command, Stdio};

    use anyhow::Result;
    use speculoos::prelude::*;

    use crate::git::repository::Repository;
    use crate::test_helpers::run_test_with_context;

    #[test]
    fn init_repo() -> Result<()> {
        run_test_with_context(|context| {
            let repo = Repository::init(&context.test_dir);

            assert_that!(repo).is_ok();
            Ok(())
        })
    }

    #[test]
    fn get_repo_working_dir_some() -> Result<()> {
        run_test_with_context(|context| {
            let repo = Repository::init(&context.test_dir)?;
            let dir = context.test_dir.join("dir");
            std::fs::create_dir(&dir)?;
            std::env::set_current_dir(&dir)?;

            assert_that!(repo.get_repo_dir()).is_equal_to(Some(context.test_dir.as_path()));
            Ok(())
        })
    }

    // see: https://git-scm.com/book/en/v2/Git-on-the-Server-Getting-Git-on-a-Server
    #[test]
    fn open_bare_err() -> Result<()> {
        run_test_with_context(|context| {
            Command::new("git")
                .arg("init")
                .arg(&context.test_dir)
                .arg("bare")
                .stdout(Stdio::inherit())
                .output()?;

            let repo = Repository::open(&context.test_dir);

            assert_that!(repo).is_err();
            Ok(())
        })
    }

    #[test]
    fn get_repo_head_oid_ok() -> Result<()> {
        run_test_with_context(|context| {
            let repo = Repository::init(&context.test_dir)?;
            std::fs::write(context.test_dir.join("file"), "changes")?;
            repo.add_all()?;
            let commit_oid = repo.commit("first commit")?;

            let oid = repo.get_head_commit_oid();

            assert_that!(oid).is_ok().is_equal_to(commit_oid);

            Ok(())
        })
    }

    #[test]
    fn get_repo_head_oid_err() -> Result<()> {
        run_test_with_context(|context| {
            let repo = Repository::init(&context.test_dir)?;

            let oid = repo.get_head_commit_oid();

            assert_that!(oid).is_err();
            Ok(())
        })
    }

    #[test]
    fn get_repo_head_obj_ok() -> Result<()> {
        run_test_with_context(|context| {
            let repo = Repository::init(&context.test_dir)?;
            std::fs::write(context.test_dir.join("file"), "changes")?;
            repo.add_all()?;
            let commit_oid = repo.commit("first commit")?;

            let head = repo.get_head_commit().map(|head| head.id());

            assert_that!(head).is_ok().is_equal_to(commit_oid);

            Ok(())
        })
    }

    #[test]
    fn get_repo_head_obj_err() -> Result<()> {
        run_test_with_context(|context| {
            let repo = Repository::init(&context.test_dir)?;
            std::fs::write(context.test_dir.join("file"), "changes")?;
            repo.add_all()?;

            let head = repo.get_head_commit();

            assert_that!(head).is_err();
            Ok(())
        })
    }

    #[test]
    fn get_head_some() -> Result<()> {
        run_test_with_context(|context| {
            let repo = Repository::init(&context.test_dir)?;
            std::fs::write(&context.test_dir.join("file"), "changes")?;
            repo.add_all()?;
            repo.commit("first commit")?;

            let tag = repo.get_head();

            assert_that!(tag).is_some();
            Ok(())
        })
    }

    #[test]
    fn get_head_none() -> Result<()> {
        run_test_with_context(|context| {
            let repo = Repository::init(&context.test_dir)?;
            std::fs::write(&context.test_dir.join("file"), "changes")?;
            repo.add_all()?;

            let tag = repo.get_head();

            assert_that!(tag).is_none();
            Ok(())
        })
    }

    #[test]
    fn get_branch_short_hand() -> Result<()> {
        run_test_with_context(|context| {
            let repo = Repository::init(&context.test_dir)?;
            std::fs::write(&context.test_dir.join("file"), "changes")?;
            repo.add_all()?;
            repo.commit("hello one")?;

            let shorthand = repo.get_branch_shorthand();

            assert_that!(shorthand).is_equal_to(Some("master".to_string()));
            Ok(())
        })
    }
}
