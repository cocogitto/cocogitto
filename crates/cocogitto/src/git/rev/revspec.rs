use crate::git::error::Git2Error;
use crate::git::repository::Repository;
use crate::git::tag::Tag;
use git2::Oid;

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct RevSpecPattern2 {
    pub from: Option<Oid>,
    pub to: Option<Oid>,
}

impl RevSpecPattern2 {
    pub fn from(oid: Oid) -> Self {
        Self {
            from: Some(oid),
            to: None,
        }
    }

    pub fn to(oid: Oid) -> Self {
        Self {
            from: None,
            to: Some(oid),
        }
    }

    pub fn full() -> Self {
        Self {
            from: None,
            to: None,
        }
    }
}

impl Repository {
    pub fn revspec_from_str(&self, s: &str) -> Result<RevSpecPattern2, Git2Error> {
        if let Some((from, to)) = s.split_once("..") {
            let from = if from.is_empty() {
                None
            } else {
                Some(self.parse_rev(from)?)
            };

            let to = if to.is_empty() {
                None
            } else {
                Some(self.parse_rev(to)?)
            };

            Ok(RevSpecPattern2 { from, to })
        } else if let Ok(tag) = Tag::from_str(s, None) {
            let previous = self.get_previous_tag(&tag)?.and_then(|tag| tag.oid);

            Ok(RevSpecPattern2 {
                from: previous,
                to: Some(self.parse_rev(s)?),
            })
        } else {
            Err(Git2Error::InvalidCommitRangePattern(s.to_string()))
        }
    }

    pub fn parse_rev(&self, from: &str) -> Result<Oid, Git2Error> {
        self.get_cache()
            .refs
            .get(from)
            .copied()
            .or_else(|| Some(self.0.revparse_single(from).ok()?.id()))
            .ok_or(Git2Error::UnknownRevision(from.to_string()))
    }
}

#[cfg(test)]
mod test {
    use crate::git::error::Git2Error;
    use crate::git::oid::CommitInfo;
    use crate::git::repository::Repository;
    use crate::git::rev::revspec::RevSpecPattern2;
    use crate::git::tag::Tag;
    use crate::test_helpers::{commit, git_init_no_gpg, git_tag};
    use anyhow::Result;
    use git2::Oid;
    use sealed_test::prelude::*;
    use semver::Version;
    use speculoos::prelude::*;

    impl Repository {
        pub(super) fn get_commit_info(&self, from: &str) -> Result<CommitInfo, Git2Error> {
            let oid = self.parse_rev(from)?;

            Ok(self
                .get_cache()
                .commits
                .get(&oid)
                .cloned()
                .unwrap_or(CommitInfo::new(oid)))
        }
    }

    #[sealed_test]
    fn should_resolve_tag_oid() -> Result<()> {
        let repo = git_init_no_gpg()?;
        commit("chore: first commit")?;
        let commit_oid = commit("chore: v1")?;
        git_tag("1.0.0")?;

        let oid = repo.get_commit_info("1.0.0")?;

        assert_that!(oid).is_equal_to(CommitInfo::from(Tag {
            package: None,
            prefix: None,
            version: Version::new(1, 0, 0),
            oid: Some(Oid::from_str(&commit_oid)?),
        }));

        Ok(())
    }

    #[sealed_test]
    fn should_resolve_head_oid() -> Result<()> {
        let repo = git_init_no_gpg()?;
        commit("chore: first commit")?;
        let commit_oid = commit("chore: other")?;

        let oid = repo.get_commit_info(&commit_oid)?;

        assert_that!(oid).is_equal_to(CommitInfo::new(Oid::from_str(&commit_oid)?));

        Ok(())
    }

    #[sealed_test]
    fn should_resolve_commit_oid() -> Result<()> {
        let repo = git_init_no_gpg()?;
        commit("chore: first commit")?;
        let commit_oid = commit("chore: other")?;
        commit("chore: head")?;

        let oid = repo.get_commit_info(&commit_oid)?;

        assert_that!(oid).is_equal_to(CommitInfo::new(Oid::from_str(&commit_oid)?));

        Ok(())
    }

    #[sealed_test]
    fn should_error_on_unknown_rev() -> Result<()> {
        let repo = git_init_no_gpg()?;
        commit("chore: first commit")?;
        commit("chore: other")?;

        let oid = repo.get_commit_info("v32");

        assert_that!(oid).is_err();

        Ok(())
    }

    #[sealed_test]
    fn convert_str_to_pattern_to() -> Result<()> {
        let repo = git_init_no_gpg()?;
        commit("chore: first commit")?;
        let sha_1 = commit("chore: v1")?;
        git_tag("1.0.0")?;

        let pattern = repo.revspec_from_str("..1.0.0")?;

        assert_that!(pattern).is_equal_to(RevSpecPattern2 {
            from: None,
            to: Some(sha_1.parse().unwrap()),
        });
        Ok(())
    }

    #[sealed_test]
    fn convert_str_to_pattern_from() -> Result<()> {
        let repo = git_init_no_gpg()?;
        commit("chore: first commit")?;
        let sha_1 = commit("chore: v1")?;
        git_tag("1.0.0")?;
        commit("chore: more commits")?;

        let pattern = repo.revspec_from_str("1.0.0..")?;

        assert_that!(pattern).is_equal_to(RevSpecPattern2 {
            from: Some(sha_1.parse().unwrap()),
            to: None,
        });
        Ok(())
    }

    #[sealed_test]
    fn convert_empty_pattern() -> Result<()> {
        let repo = git_init_no_gpg()?;
        commit("chore: first commit")?;

        let pattern = repo.revspec_from_str("..")?;

        assert_that!(pattern).is_equal_to(RevSpecPattern2 {
            from: None,
            to: None,
        });
        Ok(())
    }

    #[sealed_test]
    fn error_invalid_pattern() -> Result<()> {
        let repo = git_init_no_gpg()?;
        commit("chore: first commit")?;
        let sha_1 = commit("chore: v1")?;
        git_tag("1.0.0")?;
        let pattern = repo.revspec_from_str("1.0.0")?;

        assert_that!(pattern).is_equal_to(RevSpecPattern2 {
            from: None,
            to: Some(sha_1.parse().unwrap()),
        });
        Ok(())
    }

    #[sealed_test]
    fn convert_full_pattern() -> Result<()> {
        let repo = git_init_no_gpg()?;
        commit("chore: first commit")?;
        commit("chore: first commit")?;
        let sha_1 = commit("chore: v1")?;
        git_tag("1.0.0")?;
        let sha_2 = commit("chore: more commits")?;
        git_tag("2.0.0")?;

        let pattern = repo.revspec_from_str("1.0.0..2.0.0")?;

        assert_that!(pattern).is_equal_to(RevSpecPattern2 {
            from: Some(sha_1.parse().unwrap()),
            to: Some(sha_2.parse().unwrap()),
        });
        Ok(())
    }
}
