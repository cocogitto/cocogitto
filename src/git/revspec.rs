use anyhow::anyhow;
use anyhow::Result;
use git2::{Commit, Oid};

use crate::git::repository::Repository;
use crate::git::tag::Tag;
use crate::OidOf;

pub struct CommitRange<'repo> {
    pub from: OidOf,
    pub to: OidOf,
    pub commits: Vec<Commit<'repo>>,
}
impl Repository {
    /// Return a [`CommitRange`] containing all commit in the current repository
    pub fn all_commits(&self) -> Result<CommitRange> {
        let mut revwalk = self.0.revwalk()?;
        revwalk.push_head()?;
        let mut commits = vec![];

        for oid in revwalk {
            let commit = self.0.find_commit(oid?)?;
            commits.push(commit);
        }

        let from = commits
            .first()
            .map(|commit| commit.id())
            .map(OidOf::Other)
            .expect("No commit found");

        let to = commits
            .last()
            .map(|commit| commit.id())
            .map(OidOf::Head)
            .expect("No commit found");

        Ok(CommitRange { from, to, commits })
    }

    /// Return a commit range
    /// `from` : either a tag or an oid, latest tag if none, fallbacks to first commit
    /// `to`: HEAD if none
    pub fn get_commit_range(&self, from: Option<&str>, to: Option<&str>) -> Result<CommitRange> {
        // Is the given `to` arg a tag or an oid ?
        let maybe_to_tag = to.map(|to| self.resolve_tag(to).ok()).flatten();

        // get/validate the target oid
        let to = match to {
            None => self.get_head_commit_oid()?,
            Some(to) => self.0.revparse_single(to)?.id(),
        };

        // Either user input, latest tag since `to`, or first commit
        let from = match from {
            // No `from` arg provided get latest tag in `to` parents
            None => self
                .get_latest_tag_starting_from(to)
                .map(OidOf::Tag)
                // No tag in the tree, fallback to first commit
                .unwrap_or_else(|_| {
                    self.get_first_commit()
                        .map(OidOf::Other)
                        .expect("No commit found")
                }),
            // We might have a tag
            Some(from) => self
                .resolve_tag(from)
                .ok()
                .map(OidOf::Tag)
                // No tag found, this is an oid
                .unwrap_or_else(|| {
                    let oid = self
                        .0
                        .revparse_single(from)
                        .expect("Expected oid or tag")
                        .id();
                    OidOf::Other(oid)
                }),
        };

        // Resolve shorthands and tags
        let spec = format!("{}..{}", from, to);

        // Attempt to resolve tag names, fallback to oid
        let to = maybe_to_tag
            .map(OidOf::Tag)
            .unwrap_or_else(|| OidOf::Other(to));

        let commits = self.get_commit_range_from_spec(&spec)?;

        Ok(CommitRange { from, to, commits })
    }

    fn get_commit_range_from_spec(&self, spec: &str) -> Result<Vec<Commit>> {
        let mut revwalk = self.0.revwalk()?;

        revwalk.push_range(spec)?;

        let mut commits: Vec<Commit> = vec![];

        for oid in revwalk {
            let oid = oid?;
            let commit = self.0.find_commit(oid)?;
            commits.push(commit);
        }

        Ok(commits)
    }

    // Hide all commit after `starting_point` and get the closest tag
    fn get_latest_tag_starting_from(&self, starting_point: Oid) -> Result<Tag> {
        let starting_point = self.0.find_commit(starting_point)?;
        let starting_point = starting_point.parent(0)?;
        let first_commit = self.get_first_commit()?;
        let mut revwalk = self.0.revwalk()?;
        let range = format!("{}..{}", first_commit, starting_point.id());

        revwalk.push_range(&range)?;
        let mut range = vec![];
        for oid in revwalk {
            range.push(oid?);
        }

        let mut tags = vec![];
        self.0
            .tag_foreach(|oid, name| {
                let name = String::from_utf8_lossy(name);
                let name = name.as_ref().strip_prefix("refs/tags/").unwrap();
                if range.contains(&oid) {
                    if let Ok(tag) = Tag::new(name, oid) {
                        tags.push(tag);
                    };
                };
                true
            })
            .expect("Unable to walk tags");

        let latest_tag: Option<Tag> = tags.into_iter().max();

        match latest_tag {
            Some(tag) => Ok(tag),
            None => Err(anyhow!("Unable to get any tag")),
        }
    }
}

#[cfg(test)]
mod test {
    use anyhow::Result;
    use git2::Oid;
    use speculoos::prelude::*;

    use crate::git::repository::Repository;
    use crate::git::tag::Tag;
    use crate::test_helpers::run_test_with_context;
    use crate::OidOf;

    #[test]
    fn all_commits() -> Result<()> {
        run_test_with_context(|context| {
            // Arrange
            let repo = Repository::open(&context.current_dir)?;

            // Act
            let range = repo.all_commits()?;

            // Assert
            assert_that!(range.commits).is_not_empty();
            Ok(())
        })
    }
    #[test]
    fn from_tag_to_tag_ok() -> Result<()> {
        run_test_with_context(|context| {
            // Arrange
            let repo = Repository::open(&context.current_dir)?;
            let v1_0_0 = Oid::from_str("549070fa99986b059cbaa9457b6b6f065bbec46b")?;
            let v1_0_0 = OidOf::Tag(Tag::new("1.0.0", v1_0_0)?);
            let v3_0_0 = Oid::from_str("c6508e243e2816e2d2f58828ee0c6721502958dd")?;
            let v3_0_0 = OidOf::Tag(Tag::new("3.0.0", v3_0_0)?);

            // Act
            let range = repo.get_commit_range(Some("1.0.0"), Some("3.0.0"))?;

            // Assert
            assert_that!(range.from).is_equal_to(v1_0_0);
            assert_that!(range.to).is_equal_to(v3_0_0);

            Ok(())
        })
    }

    #[test]
    fn from_tag_to_head() -> Result<()> {
        run_test_with_context(|context| {
            // Arrange
            let repo = Repository::open(&context.current_dir)?;
            let head = repo.get_head_commit_oid()?;
            let head = OidOf::Other(head);
            let v1_0_0 = Oid::from_str("549070fa99986b059cbaa9457b6b6f065bbec46b")?;
            let v1_0_0 = OidOf::Tag(Tag::new("1.0.0", v1_0_0)?);

            // Act
            let range = repo.get_commit_range(Some("1.0.0"), None)?;

            // Assert
            assert_that!(range.from).is_equal_to(v1_0_0);
            assert_that!(range.to).is_equal_to(head);

            Ok(())
        })
    }

    #[test]
    fn from_latest_to_head() -> Result<()> {
        run_test_with_context(|context| {
            // Arrange
            let repo = Repository::open(&context.current_dir)?;
            let head = repo.get_head_commit_oid()?;
            let head = OidOf::Other(head);
            let latest = OidOf::Tag(repo.get_latest_tag()?);

            // Act
            let range = repo.get_commit_range(None, None)?;

            // Assert
            assert_that!(range.from).is_equal_to(latest);
            assert_that!(range.to).is_equal_to(head);

            Ok(())
        })
    }
    #[test]
    fn from_previous_to_tag() -> Result<()> {
        run_test_with_context(|context| {
            // Arrange
            let repo = Repository::open(&context.current_dir)?;
            let v2_1_1 = Oid::from_str("9dcf728d2eef6b5986633dd52ecbe9e416234898")?;
            let v2_1_1 = OidOf::Tag(Tag::new("2.1.1", v2_1_1)?);
            let v3_0_0 = Oid::from_str("c6508e243e2816e2d2f58828ee0c6721502958dd")?;
            let v3_0_0 = OidOf::Tag(Tag::new("3.0.0", v3_0_0)?);

            // Act
            let range = repo.get_commit_range(None, Some("3.0.0"))?;

            // Assert
            assert_that!(range.from).is_equal_to(v2_1_1);
            assert_that!(range.to).is_equal_to(v3_0_0);

            Ok(())
        })
    }
}
