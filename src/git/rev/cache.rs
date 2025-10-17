use crate::git::error::{Git2Error, TagError};
use crate::git::oid::OidOf;
use crate::git::repository::Repository;
use crate::git::tag::Tag;
use std::collections::BTreeMap;
use std::sync::{Mutex, MutexGuard};

static REPO_CACHE: Mutex<BTreeMap<String, Vec<OidOf>>> = Mutex::new(BTreeMap::new());

pub(crate) fn get_cache(repo: &Repository) -> MutexGuard<'_, BTreeMap<String, Vec<OidOf>>> {
    let mut cache = REPO_CACHE.lock().unwrap();
    if cache.is_empty() {
        build_cache(repo, &mut cache).expect("failed to construct tag cache");
    }
    cache
}

fn build_cache(
    repo: &Repository,
    cache: &mut BTreeMap<String, Vec<OidOf>>,
) -> Result<(), Git2Error> {
    if !cache.is_empty() {
        return Ok(());
    }

    let mut add_entry = |key: String, value: OidOf| {
        cache.entry(key).or_default().push(value);
    };

    // HEAD
    let head = repo.get_head_commit()?.id();
    add_entry(head.to_string(), OidOf::Head(head));

    // First Commit
    let first = OidOf::FirstCommit(repo.get_first_commit()?);
    add_entry(first.to_string(), first);

    // Tags
    let tags = repo.0.tag_names(None)?;
    let tags = tags
        .into_iter()
        .flatten()
        .filter_map(|tag| repo.resolve_tag(tag).ok());
    for tag in tags {
        if let Some(oid) = &tag.oid {
            add_entry(oid.to_string(), OidOf::Tag(tag.clone()));
        }
        add_entry(tag.to_string(), OidOf::Tag(tag))
    }

    Ok(())
}

impl Repository {
    fn resolve_tag(&self, tag: &str) -> Result<Tag, Git2Error> {
        let oid = self
            .0
            .resolve_reference_from_short_name(tag)
            .map_err(|err| TagError::not_found(tag, err))?
            .peel_to_commit()
            .map_err(|err| TagError::no_commit(tag, err))?
            .id();

        // check if tag is reachable from HEAD
        let head = self.get_head_commit_oid()?;
        if head != oid && !self.0.graph_descendant_of(head, oid)? {
            return Err(Git2Error::TagError(TagError::NotReachableFromHead {
                tag: tag.to_string(),
            }));
        }

        Tag::from_str(tag, Some(oid)).map_err(Git2Error::TagError)
    }
}

#[cfg(test)]
mod test {
    use crate::git::repository::Repository;
    use crate::git::rev::cache::get_cache;
    use crate::test_helpers::git_init_no_gpg;
    use cmd_lib::run_cmd;
    use sealed_test::prelude::*;
    use speculoos::prelude::*;

    #[test]
    fn init_cache_ok() -> anyhow::Result<()> {
        let repo = Repository::open(".")?;
        let cache = get_cache(&repo);
        assert_that!(cache.is_empty()).is_false();
        Ok(())
    }

    #[sealed_test]
    fn resolve_annotated_tag_ok() -> anyhow::Result<()> {
        // Arrange
        let repo = git_init_no_gpg()?;
        run_cmd!(
            git commit --allow-empty -m "first commit";
            git tag -a 1.0.0 -m "annotated tag";
        )?;

        // Act
        let tag = repo.resolve_tag("1.0.0");

        // Assert
        assert_that!(tag).is_ok();
        Ok(())
    }

    #[sealed_test]
    fn resolve_lightweight_tag_ok() -> anyhow::Result<()> {
        // Arrange
        let repo = git_init_no_gpg()?;
        run_cmd!(
            git commit --allow-empty -m "first commit";
            git tag 1.0.0;
        )?;

        // Act
        let tag = repo.resolve_tag("1.0.0");

        // Assert
        assert_that!(tag).is_ok();
        Ok(())
    }

    #[sealed_test]
    fn resolve_lightweight_tag_err() -> anyhow::Result<()> {
        // Arrange
        let repo = git_init_no_gpg()?;
        run_cmd!(
            git commit --allow-empty -m "first commit";
            git tag the_tag;
        )?;

        // Act
        let tag = repo.resolve_tag("the_taaaag");

        // Assert
        assert_that!(tag).is_err();
        Ok(())
    }

    #[sealed_test]
    fn dont_read_future_tag() -> anyhow::Result<()> {
        // Arrange
        let repo = git_init_no_gpg()?;
        run_cmd!(
            git commit --allow-empty -m "first commit";
            git commit --allow-empty -m "second commit";
            git tag 1.0.0;
            git reset HEAD^
        )?;

        // Act
        let tag = repo.resolve_tag("1.0.0");

        // Assert
        assert_that!(tag).is_err();
        Ok(())
    }
}
