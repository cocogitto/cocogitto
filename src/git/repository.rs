use std::fmt::{Debug, Formatter};
use std::path::Path;

use crate::error::CocogittoError::{self, Git};
use crate::git::{status::Statuses, tag::Tag};
use crate::{OidOf, SETTINGS};

use anyhow::{anyhow, ensure, Result};
use colored::Colorize;
use git2::{
    Commit as Git2Commit, Diff, DiffOptions, IndexAddOption, Object, ObjectType, Oid,
    Repository as Git2Repository, StatusOptions,
};
use itertools::Itertools;

use git2::string_array::StringArray;

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

        let diff = match &self.get_head() {
            Some(head) => self
                .0
                .diff_tree_to_index(head.as_tree(), None, Some(&mut options)),
            None => self
                .0
                .diff_tree_to_workdir_with_index(None, Some(&mut options)),
        };

        match diff {
            Ok(diff) => {
                if diff.deltas().len() > 0 {
                    Some(diff)
                } else {
                    None
                }
            }
            Err(..) => None,
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
            Err(err) => Err(anyhow!("Repo as not HEAD:{}", err)),
        }
    }

    pub(crate) fn get_latest_tag(&self) -> Result<Tag> {
        let latest_tag: Option<Tag> = self
            .tags()?
            .iter()
            .flatten()
            .map(|tag| self.resolve_lightweight_tag(tag))
            .filter_map(Result::ok)
            .max();

        match latest_tag {
            Some(tag) => Ok(tag),
            None => Err(anyhow!("Unable to get any tag")),
        }
    }

    pub(crate) fn get_tag_commits(&self, target_tag: &str) -> Result<(OidOf, OidOf)> {
        let oid_of_target_tag = OidOf::Tag(self.resolve_lightweight_tag(target_tag)?);

        // Starting point is set to first commit
        let oid_of_first_commit = OidOf::Other(self.get_first_commit()?);

        let oid_of_previous_tag = self
            .tags()?
            .iter()
            .flatten()
            .sorted()
            .tuple_windows()
            .find(|&(_, cur)| cur == target_tag)
            .map(|(prev, _)| {
                OidOf::Tag(
                    self.resolve_lightweight_tag(prev)
                        .expect("Unexpected tag parsing error"),
                )
            })
            .unwrap_or(oid_of_first_commit);

        Ok((oid_of_previous_tag, oid_of_target_tag))
    }

    pub(crate) fn get_latest_tag_oid(&self) -> Result<Oid> {
        self.get_latest_tag()
            .map(|tag| tag.oid().to_owned())
            .map_err(|err| anyhow!("Could not resolve latest tag:{}", err))
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

    fn tags(&self) -> Result<StringArray> {
        let pattern = SETTINGS
            .tag_prefix
            .as_ref()
            .map(|prefix| format!("{}*", prefix));

        self.0
            .tag_names(pattern.as_deref())
            .map_err(|err| anyhow!("{}", err))
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

    use crate::git::repository::Repository;
    use crate::OidOf;

    use crate::test_helpers::run_test_with_context;
    use anyhow::Result;
    use speculoos::prelude::*;

    #[test]
    fn init_repo() -> Result<()> {
        run_test_with_context(|context| {
            let repo = Repository::init(&context.test_dir);

            assert_that!(repo).is_ok();
            Ok(())
        })
    }

    #[test]
    fn create_commit_ok() -> Result<()> {
        run_test_with_context(|context| {
            let repo = Repository::init(".")?;
            std::fs::write(context.test_dir.join("file"), "changes")?;
            repo.add_all()?;

            assert_that!(repo.commit("feat: a test commit")).is_ok();
            Ok(())
        })
    }

    #[test]
    fn not_create_empty_commit() -> Result<()> {
        run_test_with_context(|_| {
            let repo = Repository::init(".")?;

            assert_that!(repo.commit("feat: a test commit")).is_err();
            Ok(())
        })
    }

    #[test]
    fn not_create_empty_commit_with_unstaged_changed() -> Result<()> {
        run_test_with_context(|context| {
            let repo = Repository::init(".")?;
            std::fs::write(context.test_dir.join("file"), "changes")?;

            assert_that!(repo.commit("feat: a test commit")).is_err();
            Ok(())
        })
    }

    #[test]
    fn get_repo_working_dir_some() -> Result<()> {
        run_test_with_context(|context| {
            let repo = Repository::init(".")?;
            let dir = context.test_dir.join("dir");
            std::fs::create_dir(&dir)?;
            std::env::set_current_dir(&dir)?;

            assert_that!(repo.get_repo_dir()).is_equal_to(Some(context.test_dir.as_path()));
            Ok(())
        })
    }

    #[test]
    fn get_diff_some() -> Result<()> {
        run_test_with_context(|context| {
            let repo = Repository::init(".")?;
            std::fs::write(context.test_dir.join("file"), "changes")?;
            repo.add_all()?;

            assert!(repo.get_diff(false).is_some());
            Ok(())
        })
    }

    #[test]
    fn get_diff_none() -> Result<()> {
        run_test_with_context(|context| {
            let repo = Repository::init(".")?;
            std::fs::write(context.test_dir.join("file"), "changes")?;

            assert!(repo.get_diff(false).is_none());
            Ok(())
        })
    }

    #[test]
    fn get_diff_include_untracked_some() -> Result<()> {
        run_test_with_context(|context| {
            let repo = Repository::init(".")?;
            std::fs::write(context.test_dir.join("file"), "changes")?;

            assert!(repo.get_diff(true).is_some());
            Ok(())
        })
    }

    #[test]
    fn get_diff_include_untracked_none() -> Result<()> {
        run_test_with_context(|_| {
            let repo = Repository::init(".")?;

            assert!(repo.get_diff(true).is_none());
            Ok(())
        })
    }

    // see: https://git-scm.com/book/en/v2/Git-on-the-Server-Getting-Git-on-a-Server
    #[test]
    fn open_bare_err() -> Result<()> {
        run_test_with_context(|_| {
            Command::new("git")
                .arg("init")
                .arg("bare")
                .stdout(Stdio::inherit())
                .output()?;

            let repo = Repository::open(".");

            assert_that!(repo).is_err();
            Ok(())
        })
    }

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
    fn resolve_lightweight_tag_ok() -> Result<()> {
        run_test_with_context(|context| {
            let repo = Repository::init(&context.test_dir)?;
            std::fs::write(context.test_dir.join("file"), "changes")?;
            repo.add_all()?;
            repo.commit("first commit")?;
            repo.create_tag("the_tag")?;

            let tag = repo.resolve_lightweight_tag("the_tag");

            assert_that!(tag).is_ok();
            Ok(())
        })
    }

    #[test]
    fn resolve_lightweight_tag_err() -> Result<()> {
        run_test_with_context(|context| {
            let repo = Repository::init(&context.test_dir)?;
            std::fs::write(context.test_dir.join("file"), "changes")?;
            repo.add_all()?;
            repo.commit("first commit")?;
            repo.create_tag("the_tag")?;

            let tag = repo.resolve_lightweight_tag("the_taaaag");

            assert_that!(tag).is_err();
            Ok(())
        })
    }

    #[test]
    fn get_latest_tag_ok() -> Result<()> {
        run_test_with_context(|context| {
            let repo = Repository::init(&context.test_dir)?;
            std::fs::write(&context.test_dir.join("file"), "changes")?;
            repo.add_all()?;
            repo.commit("first commit")?;
            repo.create_tag("0.1.0")?;

            std::fs::write(&context.test_dir.join("file"), "changes2")?;
            repo.add_all()?;
            repo.commit("second commit")?;
            repo.create_tag("0.2.0")?;

            let tag = repo.get_latest_tag()?;

            assert_that!(tag.to_string_with_prefix()).is_equal_to("0.2.0".to_string());
            Ok(())
        })
    }

    #[test]
    fn get_latest_tag_err() -> Result<()> {
        run_test_with_context(|context| {
            let repo = Repository::init(&context.test_dir)?;
            std::fs::write(&context.test_dir.join("file"), "changes")?;
            repo.add_all()?;
            repo.commit("first commit")?;

            let tag = repo.get_latest_tag();

            assert_that!(tag).is_err();
            Ok(())
        })
    }

    #[test]
    fn get_latest_tag_oid_ok() -> Result<()> {
        run_test_with_context(|context| {
            let repo = Repository::init(&context.test_dir)?;
            std::fs::write(&context.test_dir.join("file"), "changes")?;
            repo.add_all()?;
            repo.commit("first commit")?;
            repo.create_tag("1.0.0")?;

            let tag = repo.get_latest_tag_oid();

            assert_that!(tag).is_ok();
            Ok(())
        })
    }

    #[test]
    fn get_latest_tag_oid_err() -> Result<()> {
        run_test_with_context(|context| {
            let repo = Repository::init(&context.test_dir)?;
            std::fs::write(&context.test_dir.join("file"), "changes")?;
            repo.add_all()?;
            repo.commit("first commit")?;

            let tag = repo.get_latest_tag_oid();

            assert_that!(tag).is_err();
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
    fn get_tag_commits() -> Result<()> {
        run_test_with_context(|context| {
            let repo = Repository::init(&context.test_dir)?;
            std::fs::write(&context.test_dir.join("file"), "changes")?;
            repo.add_all()?;
            let start = repo.commit("chore: init")?;

            std::fs::write(&context.test_dir.join("file2"), "changes")?;
            repo.add_all()?;
            let end = repo.commit("chore: 1.0.0")?;

            repo.create_tag("1.0.0")?;

            std::fs::write(&context.test_dir.join("file3"), "changes")?;
            repo.add_all()?;
            repo.commit("feat: a commit")?;

            let commit_range = repo.get_tag_commits("1.0.0")?;

            assert_that!(commit_range.0).is_equal_to(OidOf::Other(start));
            assert_that!(commit_range.1.get_oid()).is_equal_to(end);
            assert_that!(commit_range.1.to_string()).is_equal_to("1.0.0".to_string());
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

    #[test]
    fn should_stash_failed_bump() -> Result<()> {
        run_test_with_context(|context| {
            let mut repo = Repository::init(&context.test_dir)?;
            std::fs::write(&context.test_dir.join("file"), "changes")?;
            repo.add_all()?;
            repo.commit("Initial commit")?;

            let statuses = repo.get_statuses()?.0;
            assert_that!(statuses).is_empty();

            std::fs::write(&context.test_dir.join("second_file"), "more changes")?;
            repo.add_all()?;
            let statuses = repo.get_statuses()?.0;
            assert_that!(statuses).has_length(1);

            repo.stash_failed_version("1.0.0")?;

            let statuses = repo.get_statuses()?.0;
            assert_that!(statuses).is_empty();
            Ok(())
        })
    }
}
