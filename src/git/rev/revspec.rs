use crate::git::error::Git2Error;
use crate::git::oid::OidOf;
use crate::git::repository::Repository;
use crate::git::rev::cache::get_cache;
use crate::git::tag::Tag;
use git2::Oid;
use std::fmt;
use std::fmt::Formatter;

#[derive(Debug, Clone)]
pub enum RevSpecPattern2 {
    Range { from: OidOf, to: OidOf },
    AtTag { from: OidOf, to: OidOf },
}

impl RevSpecPattern2 {
    pub fn from(&self) -> &Oid {
        match self {
            RevSpecPattern2::AtTag { from, .. } | RevSpecPattern2::Range { from, .. } => from.oid(),
        }
    }

    pub fn to(&self) -> &Oid {
        match self {
            RevSpecPattern2::AtTag { to, .. } | RevSpecPattern2::Range { to, .. } => to.oid(),
        }
    }
}

impl Repository {
    pub(super) fn revspec_from_str(&self, s: &str) -> Result<RevSpecPattern2, Git2Error> {
        if let Some((from, to)) = s.split_once("..") {
            let from = if from.is_empty() {
                OidOf::Other(self.get_first_commit()?)
            } else {
                self.resolve_oid_of(from)?
            };

            let to = if to.is_empty() {
                OidOf::Head(self.get_head_commit_oid()?)
            } else {
                self.resolve_oid_of(to)?
            };

            Ok(RevSpecPattern2::Range { from, to })
        } else if let Ok(tag) = Tag::from_str(s, None, None) {
            let previous = self.get_previous_tag(&tag)?.map(OidOf::Tag);

            let previous = match previous {
                None => OidOf::Other(self.get_first_commit()?),
                Some(previous) => previous,
            };

            Ok(RevSpecPattern2::AtTag {
                from: previous,
                to: self.resolve_oid_of(s)?,
            })
        } else {
            Err(Git2Error::InvalidCommitRangePattern(s.to_string()))
        }
    }

    pub(super) fn resolve_oid_of(&self, from: &str) -> Result<OidOf, Git2Error> {
        let cache = get_cache(self);

        println!("oid to resolve is {}", from);

        let oids = cache
            .iter()
            .find(|(k, _)| k.starts_with(from))
            .map(|(_, v)| v);

        for o in oids.unwrap_or(&Vec::new()) {
            print!("oid is {} ", o.to_string());
            match o {
                OidOf::FirstCommit(_) => {
                    print!("type is first FirstCommit ");
                }
                &OidOf::Head(_) => {
                    print!("type is first Head ");
                }
                OidOf::Other(_) => {
                    print!("type is first Other ");
                }
                &OidOf::Tag(_) => {
                    print!("type is first Tag ");
                }
            }
        }
        println!("");

        match oids {
            None => {
                let object = self
                    .0
                    .revparse_single(from)
                    .map_err(|_| Git2Error::UnknownRevision(from.to_string()))?;
                Ok(OidOf::Other(object.id()))
            }
            Some(oids) => Ok(oids.first().unwrap().clone()),
        }
    }

    pub(super) fn resolve_oid_of_for_package(
        &self,
        from: &str,
        package: &str,
    ) -> Result<OidOf, Git2Error> {
        let cache = get_cache(self);

        println!("pattern to look for is {}", from);

        let oids = cache
            .iter()
            .find(|(k, _)| k.starts_with(from))
            .map(|(_, v)| v.clone());

        /*
        println!("len is {}", &oid.len());

        for o in &oid {
            print!("oid is {} ", o.to_string());
        }

        println!("");
        */

        match oids {
            None => {
                let object = self
                    .0
                    .revparse_single(from)
                    .map_err(|_| Git2Error::UnknownRevision(from.to_string()))?;
                Ok(OidOf::Other(object.id()))
            }
            Some(oids) => Ok(oids
                .iter()
                .find(|oid| oid.to_string().starts_with(&package))
                .unwrap_or(oids.first().unwrap())
                .clone()),
        }
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
    use crate::git::oid::OidOf;
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

        let oid = repo.resolve_oid_of("1.0.0")?;

        assert_that!(oid).is_equal_to(OidOf::Tag(Tag {
            package: None,
            prefix: None,
            version: Version::new(1, 0, 0),
            oid: Some(Oid::from_str(&commit_oid)?),
            target: None,
        }));

        Ok(())
    }

    #[sealed_test]
    fn should_resolve_head_oid() -> Result<()> {
        let repo = git_init_no_gpg()?;
        commit("chore: first commit")?;
        let commit_oid = commit("chore: other")?;

        let oid = repo.resolve_oid_of(&commit_oid)?;

        assert_that!(oid).is_equal_to(OidOf::Head(Oid::from_str(&commit_oid)?));

        Ok(())
    }

    #[sealed_test]
    fn should_resolve_commit_oid() -> Result<()> {
        let repo = git_init_no_gpg()?;
        commit("chore: first commit")?;
        let commit_oid = commit("chore: other")?;
        commit("chore: head")?;

        let oid = repo.resolve_oid_of(&commit_oid)?;

        assert_that!(oid).is_equal_to(OidOf::Other(Oid::from_str(&commit_oid)?));

        Ok(())
    }

    #[sealed_test]
    fn should_error_on_unknown_rev() -> Result<()> {
        let repo = git_init_no_gpg()?;
        commit("chore: first commit")?;
        commit("chore: other")?;

        let oid = repo.resolve_oid_of("v32");

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
