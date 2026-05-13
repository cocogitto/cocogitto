use crate::git::error::Git2Error;
use crate::git::oid::CommitInfo;
use crate::git::repository::Repository;
use crate::git::tag::Tag;
use git2::Oid;
use std::fmt;
use std::fmt::Formatter;

#[derive(Debug, Clone)]
pub enum RevSpecPattern2 {
    Range { from: CommitInfo, to: CommitInfo },
    AtTag { from: CommitInfo, to: CommitInfo },
}

impl RevSpecPattern2 {
    pub fn from(&self) -> Oid {
        match self {
            RevSpecPattern2::AtTag { from, .. } | RevSpecPattern2::Range { from, .. } => from.oid,
        }
    }

    pub fn to(&self) -> Oid {
        match self {
            RevSpecPattern2::AtTag { to, .. } | RevSpecPattern2::Range { to, .. } => to.oid,
        }
    }
}

impl Repository {
    pub(super) fn revspec_from_str(&self, s: &str) -> Result<RevSpecPattern2, Git2Error> {
        if let Some((from, to)) = s.split_once("..") {
            let from = if from.is_empty() {
                CommitInfo::new(self.get_first_commit()?)
            } else {
                self.get_commit_info(from)?
            };

            let to = if to.is_empty() {
                self.get_cache().head_commit_info()
            } else {
                self.get_commit_info(to)?
            };

            Ok(RevSpecPattern2::Range { from, to })
        } else if let Ok(tag) = Tag::from_str(s, None) {
            let previous = self.get_previous_tag(&tag)?.map(Into::into);

            let previous = match previous {
                None => CommitInfo::new(self.get_first_commit()?),
                Some(previous) => previous,
            };

            Ok(RevSpecPattern2::AtTag {
                from: previous,
                to: self.get_commit_info(s)?,
            })
        } else {
            Err(Git2Error::InvalidCommitRangePattern(s.to_string()))
        }
    }

    pub(super) fn get_commit_info(&self, from: &str) -> Result<CommitInfo, Git2Error> {
        let cache = self.get_cache();
        let oid = cache
            .refs
            .get(from)
            .copied()
            .or_else(|| Some(self.0.revparse_single(from).ok()?.id()))
            .ok_or(Git2Error::UnknownRevision(from.to_string()))?;

        Ok(cache
            .commits
            .get(&oid)
            .cloned()
            .unwrap_or(CommitInfo::new(oid)))
    }
}

impl fmt::Display for RevSpecPattern2 {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        match self {
            RevSpecPattern2::AtTag { from, to } | RevSpecPattern2::Range { from, to } => {
                write!(f, "{from}..{to}")
            }
        }
    }
}

#[cfg(test)]
mod test {
    use crate::git::oid::CommitInfo;
    use crate::git::rev::revspec::RevSpecPattern2;
    use crate::git::tag::Tag;
    use crate::test_helpers::{commit, git_init_no_gpg, git_tag};
    use anyhow::Result;
    use git2::Oid;
    use sealed_test::prelude::*;
    use semver::Version;
    use speculoos::prelude::*;

    #[sealed_test]
    fn should_resolve_tag_oid() -> Result<()> {
        let repo = git_init_no_gpg()?;
        commit("chore: first commit")?;
        let commit_oid = commit("chore: v1")?;
        git_tag("1.0.0")?;

        let oid = repo.get_commit_info("1.0.0")?;

        let mut info = CommitInfo::from(Tag {
            package: None,
            prefix: None,
            version: Version::new(1, 0, 0),
            oid: Some(Oid::from_str(&commit_oid)?),
        });
        info.head = true;
        assert_that!(oid).is_equal_to(info);

        Ok(())
    }

    #[sealed_test]
    fn should_resolve_head_oid() -> Result<()> {
        let repo = git_init_no_gpg()?;
        commit("chore: first commit")?;
        let commit_oid = commit("chore: other")?;

        let oid = repo.get_commit_info(&commit_oid)?;

        let mut info = CommitInfo::new(Oid::from_str(&commit_oid)?);
        info.head = true;
        assert_that!(oid).is_equal_to(info);

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
        commit("chore: v1")?;
        git_tag("1.0.0")?;

        let pattern = repo.revspec_from_str("..1.0.0")?;

        assert_that!(matches!(pattern, RevSpecPattern2::Range { .. })).is_true();
        Ok(())
    }

    #[sealed_test]
    fn convert_str_to_pattern_from() -> Result<()> {
        let repo = git_init_no_gpg()?;
        commit("chore: first commit")?;
        commit("chore: v1")?;
        git_tag("1.0.0")?;
        commit("chore: more commits")?;

        let pattern = repo.revspec_from_str("1.0.0..")?;

        assert_that!(matches!(pattern, RevSpecPattern2::Range { .. })).is_true();
        Ok(())
    }

    #[sealed_test]
    fn convert_empty_pattern() -> Result<()> {
        let repo = git_init_no_gpg()?;
        commit("chore: first commit")?;
        let pattern = repo.revspec_from_str("..")?;
        assert_that!(matches!(pattern, RevSpecPattern2::Range { .. })).is_true();
        Ok(())
    }

    #[sealed_test]
    fn error_invalid_pattern() -> Result<()> {
        let repo = git_init_no_gpg()?;
        commit("chore: first commit")?;
        commit("chore: v1")?;
        git_tag("1.0.0")?;
        let pattern = repo.revspec_from_str("1.0.0")?;

        assert_that!(matches!(pattern, RevSpecPattern2::AtTag { .. })).is_true();
        Ok(())
    }

    #[sealed_test]
    fn convert_full_pattern() -> Result<()> {
        let repo = git_init_no_gpg()?;
        commit("chore: first commit")?;
        commit("chore: first commit")?;
        commit("chore: v1")?;
        git_tag("1.0.0")?;
        commit("chore: more commits")?;
        git_tag("2.0.0")?;

        let pattern = repo.revspec_from_str("1.0.0..2.0.0")?;

        assert_that!(matches!(pattern, RevSpecPattern2::Range { .. })).is_true();
        Ok(())
    }
}
