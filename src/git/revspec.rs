use std::fmt;
use std::fmt::Formatter;

use git2::{Commit, Oid};

use crate::conventional::changelog::release::Release;
use crate::git::error::Git2Error;
use crate::git::oid::OidOf;
use crate::git::repository::Repository;
use crate::git::tag::Tag;

#[derive(Debug)]
pub struct CommitRange<'repo> {
    pub from: OidOf,
    pub to: OidOf,
    pub commits: Vec<Commit<'repo>>,
}

#[derive(Debug, Default)]
pub struct RevspecPattern {
    from: Option<String>,
    to: Option<String>,
}

impl fmt::Display for RevspecPattern {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        let from = self.from.as_deref().unwrap_or("");
        let to = self.to.as_deref().unwrap_or("");
        write!(f, "{}..{}", from, to)
    }
}

impl From<&str> for RevspecPattern {
    fn from(value: &str) -> Self {
        if !value.contains("..") {
            panic!("Invalid commit range pattern: '{}'", value);
        }

        let split = value.split("..").collect::<Vec<&str>>();

        let from = if split[0].is_empty() {
            None
        } else {
            Some(split[0].to_string())
        };

        let to = if split[1].is_empty() {
            None
        } else {
            Some(split[1].to_string())
        };

        RevspecPattern { from, to }
    }
}

impl From<(&str, &str)> for RevspecPattern {
    fn from((from, to): (&str, &str)) -> Self {
        Self {
            from: Some(from.to_string()),
            to: Some(to.to_string()),
        }
    }
}

impl Repository {
    /// Return a [`CommitRange`] containing all commit in the current repository
    pub fn all_commits(&self) -> Result<CommitRange, Git2Error> {
        let mut revwalk = self.0.revwalk()?;
        revwalk.push_head()?;
        let mut commits = vec![];

        for oid in revwalk {
            let commit = self.0.find_commit(oid?)?;
            commits.push(commit);
        }

        let to = commits
            .first()
            .map(|commit| commit.id())
            .map(OidOf::Head)
            .expect("No commit found");

        let from = commits
            .last()
            .map(|commit| commit.id())
            .map(OidOf::Other)
            .expect("No commit found");

        Ok(CommitRange { from, to, commits })
    }

    pub(crate) fn get_release_range(&self, pattern: RevspecPattern) -> Result<Release, Git2Error> {
        let target = if let Some(target) = pattern.from {
            self.resolve_oid_of(&target)
        } else {
            OidOf::Other(self.get_first_commit()?)
        };

        let pattern = RevspecPattern {
            from: None,
            to: pattern.to,
        };

        let range = self.get_commit_range(&pattern)?;
        let release = Release::from(range);
        let release = self.populate_previous_release(release, target.oid())?;
        Ok(release)
    }

    fn populate_previous_release<'a>(
        &'a self,
        mut release: Release<'a>,
        target: &Oid,
    ) -> Result<Release<'a>, Git2Error> {
        let pattern = format!("..{}", release.from);
        let pattern = RevspecPattern::from(pattern.as_str());
        let range = self.get_commit_range(&pattern)?;

        // Target tag or commit reached
        if range.to.oid() == target {
            // We are not on first commit
            if range.from != range.to {
                let previous = Release::from(range);
                release.previous = Some(Box::new(previous));
            }
            return Ok(release);
        }

        let previous = Release::from(range);
        let previous = self.populate_previous_release(previous, target)?;
        release.previous = Some(Box::new(previous));

        Ok(release)
    }

    /// Return a commit range
    /// `from` : either a tag or an oid, latest tag if none, fallbacks to first commit
    /// `to`: HEAD if none
    pub fn get_commit_range(&self, pattern: &RevspecPattern) -> Result<CommitRange, Git2Error> {
        let from = pattern.from.as_deref();
        let to = pattern.to.as_deref();

        // Is the given `to` arg a tag or an oid ?
        let maybe_to_tag = match to {
            // No target tag provided, check if HEAD is tagged
            None => {
                let head = self.get_head_commit_oid()?;
                self.get_latest_tag()
                    .ok()
                    .filter(|tag| *tag.oid_unchecked() == head)
            }
            // Try to resolve a tag from the provided range, ex: ..1.0.0
            Some(to) => self.resolve_tag(to).ok(),
        };

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
            Some(from) => self.resolve_oid_of(from),
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

    fn resolve_oid_of(&self, from: &str) -> OidOf {
        // either we have a tag name
        self.resolve_tag(from)
            .map(OidOf::Tag)
            // Or an oid
            .unwrap_or_else(|_| {
                let object = self.0.revparse_single(from).expect("Expected oid or tag");

                // Is the oid pointing to a tag ?
                let tag = self
                    .all_tags()
                    .expect("Error trying to get repository tags")
                    .into_iter()
                    .find(|tag| *tag.oid_unchecked() == object.id());

                match tag {
                    None => OidOf::Other(object.id()),
                    Some(tag) => OidOf::Tag(tag),
                }
            })
    }

    fn get_commit_range_from_spec(&self, spec: &str) -> Result<Vec<Commit>, Git2Error> {
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
    fn get_latest_tag_starting_from(&self, starting_point: Oid) -> Result<Tag, Git2Error> {
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
            .tag_foreach(|mut oid, name| {
                let name = String::from_utf8_lossy(name);
                let name = name.as_ref().strip_prefix("refs/tags/").unwrap();

                // If this is an annotated tag, find the first parent commit
                if self.0.revparse_single(name).unwrap().as_commit().is_none() {
                    if let Some(commit) = self
                        .0
                        .revparse_single([name, "^{}"].concat().as_str())
                        .unwrap()
                        .as_commit()
                    {
                        oid = commit.id();
                    }
                };

                if range.contains(&oid) {
                    if let Ok(tag) = Tag::new(name, Some(oid)) {
                        tags.push(tag);
                    };
                };
                true
            })
            .expect("Unable to walk tags");

        let latest_tag: Option<Tag> = tags.into_iter().max();

        latest_tag.ok_or(Git2Error::NoTagFound)
    }
}

#[cfg(test)]
mod test {
    use crate::conventional::changelog::release::Release;
    use anyhow::Result;
    use cmd_lib::run_cmd;
    use git2::Oid;
    use sealed_test::prelude::*;
    use speculoos::prelude::*;

    use crate::git::oid::OidOf;
    use crate::git::repository::Repository;
    use crate::git::revspec::RevspecPattern;
    use crate::git::tag::Tag;

    const COCOGITTO_REPOSITORY: &str = env!("CARGO_MANIFEST_DIR");

    #[test]
    fn convert_str_to_pattern_to() {
        let pattern = RevspecPattern::from("..1.0.0");

        assert_that!(pattern.from).is_none();
        assert_that!(pattern.to)
            .is_some()
            .is_equal_to("1.0.0".to_string());
    }

    #[test]
    fn convert_str_to_pattern_from() {
        let pattern = RevspecPattern::from("1.0.0..");

        assert_that!(pattern.from)
            .is_some()
            .is_equal_to("1.0.0".to_string());
        assert_that!(pattern.to).is_none()
    }

    #[test]
    fn convert_empty_pattern() {
        let pattern = RevspecPattern::from("..");

        assert_that!(pattern.from).is_none();
        assert_that!(pattern.to).is_none()
    }

    #[test]
    #[should_panic(expected = "Invalid commit range pattern: '1.0.0'")]
    fn panic_invalid_pattern() {
        let _ = RevspecPattern::from("1.0.0");
    }

    #[test]
    fn convert_full_pattern() {
        let pattern = RevspecPattern::from("1.0.0..2.0.0");

        assert_that!(pattern.from)
            .is_some()
            .is_equal_to("1.0.0".to_string());
        assert_that!(pattern.to)
            .is_some()
            .is_equal_to("2.0.0".to_string());
    }

    #[test]
    fn all_commits() -> Result<()> {
        // Arrange
        let repo = Repository::open(COCOGITTO_REPOSITORY)?;

        // Act
        let range = repo.all_commits()?;

        // Assert
        assert_that!(range.commits).is_not_empty();
        Ok(())
    }

    #[test]
    fn get_release_range_integration_test() -> Result<()> {
        // Arrange
        let repo = Repository::open(COCOGITTO_REPOSITORY)?;
        let format_version = |release: &Release| format!("{}", release.version);

        // Act
        let release = repo.get_release_range(RevspecPattern::from("0.32.1..0.32.3"))?;

        // Assert
        assert_that!(format_version(&release)).is_equal_to("0.32.3".to_string());

        let release = *release.previous.unwrap();
        assert_that!(format_version(&release)).is_equal_to("0.32.2".to_string());

        let release = *release.previous.unwrap();
        assert_that!(format_version(&release)).is_equal_to("0.32.1".to_string());

        assert_that!(release.previous).is_none();
        Ok(())
    }

    #[sealed_test]
    fn get_annotated_tag_commits() -> Result<()> {
        // Arrange
        let repo = Repository::init(".")?;
        run_cmd!(
            git init;
            echo changes > file;
            git add .;
        )?;

        let start = repo.commit("chore: init", false)?;

        run_cmd!(
            git init;
            echo changes > file2;
            git add .;
        )?;

        let _end = repo.commit("chore: 1.0.0", false)?;

        // Create an annotated tag
        let head = repo.get_head_commit().unwrap();
        let sig = git2::Signature::now("Author", "email@example.com")?;
        repo.0
            .tag("1.0.0", &head.into_object(), &sig, "the_msg", false)?;

        run_cmd!(
            git init;
            echo changes > file3;
            git add .;
        )?;

        repo.commit("feat: a commit", false)?;

        let commit_range = repo.get_commit_range(&RevspecPattern::from("..1.0.0"))?;

        assert_that!(commit_range.from).is_equal_to(OidOf::Other(start));
        assert_that!(commit_range.to.to_string()).is_equal_to("1.0.0".to_string());
        assert_that!(commit_range.commits).has_length(1);
        Ok(())
    }

    #[sealed_test]
    fn get_tag_commits() -> Result<()> {
        // Arrange
        let repo = Repository::init(".")?;

        run_cmd!(
            git init;
            echo changes > file;
            git add .;
        )?;

        let start = repo.commit("chore: init", false)?;

        run_cmd!(
            git commit --allow-empty -m "chore: 1.0.0";
            git tag 1.0.0;
            git commit --allow-empty -m "feat: a commit";
        )?;

        // Act
        let commit_range = repo.get_commit_range(&RevspecPattern::from("..1.0.0"))?;

        // Assert
        assert_that!(commit_range.from).is_equal_to(OidOf::Other(start));
        assert_that!(commit_range.to.to_string()).is_equal_to("1.0.0".to_string());
        assert_that!(commit_range.commits).has_length(1);
        Ok(())
    }

    #[test]
    fn from_tag_to_tag_ok() -> Result<()> {
        // Arrange
        let repo = Repository::open(COCOGITTO_REPOSITORY)?;
        let v1_0_0 = Oid::from_str("549070fa99986b059cbaa9457b6b6f065bbec46b")?;
        let v1_0_0 = OidOf::Tag(Tag::new("1.0.0", Some(v1_0_0))?);
        let v3_0_0 = Oid::from_str("c6508e243e2816e2d2f58828ee0c6721502958dd")?;
        let v3_0_0 = OidOf::Tag(Tag::new("3.0.0", Some(v3_0_0))?);

        // Act
        let range = repo.get_commit_range(&RevspecPattern::from("1.0.0..3.0.0"))?;

        // Assert
        assert_that!(range.from).is_equal_to(v1_0_0);
        assert_that!(range.to).is_equal_to(v3_0_0);

        Ok(())
    }

    #[test]
    fn from_tag_to_head() -> Result<()> {
        // Arrange
        let repo = Repository::open(COCOGITTO_REPOSITORY)?;
        let head = repo.get_head_commit_oid()?;
        let head = OidOf::Other(head);
        let tag = repo.get_latest_tag()?;

        // Cover the case when we release a version and run the test in the CI right after that
        let head = if tag.oid() == Some(head.oid()) {
            OidOf::Tag(tag)
        } else {
            head
        };

        let v1_0_0 = Oid::from_str("549070fa99986b059cbaa9457b6b6f065bbec46b")?;
        let v1_0_0 = OidOf::Tag(Tag::new("1.0.0", Some(v1_0_0))?);

        // Act
        let range = repo.get_commit_range(&RevspecPattern::from("1.0.0.."))?;

        // Assert
        assert_that!(range.from).is_equal_to(v1_0_0);
        assert_that!(range.to).is_equal_to(head);

        Ok(())
    }

    #[test]
    fn from_latest_to_head() -> Result<()> {
        // Arrange
        let repo = Repository::open(COCOGITTO_REPOSITORY)?;
        let head = repo.get_head_commit_oid()?;
        let head = OidOf::Other(head);
        let mut tags = repo.all_tags()?;
        tags.sort();
        let mut latest = tags.last().unwrap();

        if latest.oid().unwrap() == head.oid() {
            latest = &tags[tags.len() - 2];
        }

        let latest = OidOf::Tag(latest.clone());

        // Act
        let range = repo.get_commit_range(&RevspecPattern::default())?;

        // Assert
        assert_that!(range.from).is_equal_to(latest);
        assert_that!(range.to.oid()).is_equal_to(head.oid());

        Ok(())
    }

    #[test]
    fn from_previous_to_tag() -> Result<()> {
        // Arrange
        let repo = Repository::open(COCOGITTO_REPOSITORY)?;
        let v2_1_1 = Oid::from_str("9dcf728d2eef6b5986633dd52ecbe9e416234898")?;
        let v2_1_1 = OidOf::Tag(Tag::new("2.1.1", Some(v2_1_1))?);
        let v3_0_0 = Oid::from_str("c6508e243e2816e2d2f58828ee0c6721502958dd")?;
        let v3_0_0 = OidOf::Tag(Tag::new("3.0.0", Some(v3_0_0))?);

        // Act
        let range = repo.get_commit_range(&RevspecPattern::from("..3.0.0"))?;

        // Assert
        assert_that!(range.from).is_equal_to(v2_1_1);
        assert_that!(range.to).is_equal_to(v3_0_0);

        Ok(())
    }

    #[test]
    fn recursive_from_origin_to_head() -> Result<()> {
        // Arrange
        let repo = Repository::open(COCOGITTO_REPOSITORY)?;
        let mut tag_count = repo.0.tag_names(None)?.len();
        let head = repo.get_head_commit_oid()?;
        let latest = repo.get_latest_tag()?;
        let latest = latest.oid();
        if latest == Some(&head) {
            tag_count -= 1;
        };

        // Act
        let mut release = repo.get_release_range(RevspecPattern::from(".."))?;
        let mut count = 0;

        while let Some(previous) = release.previous {
            release = *previous;
            count += 1;
        }

        // Assert
        assert_that!(count).is_equal_to(tag_count);

        Ok(())
    }
}
