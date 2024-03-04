use crate::git::error::TagError;
use crate::git::oid::OidOf;
use crate::git::repository::Repository;
use crate::git::tag::Tag;
use once_cell::sync::Lazy;
use std::collections::BTreeMap;
use std::sync::{Arc, Mutex, MutexGuard};

// TODO: we need to handle the case where multiple tags point to the same commit Oid
static REPO_CACHE: Lazy<Arc<Mutex<BTreeMap<String, OidOf>>>> =
    Lazy::new(|| Arc::new(Mutex::new(BTreeMap::new())));

pub(crate) fn init(repository: &Repository) -> MutexGuard<'_, BTreeMap<String, OidOf>> {
    let mut cache = REPO_CACHE.lock().unwrap();
    let head = repository.get_head_commit().expect("HEAD");
    let first = repository.get_first_commit().expect("first commit");

    cache.insert(head.id().to_string(), OidOf::Head(head.id()));
    cache.insert(first.to_string(), OidOf::FirstCommit(first));

    let tag_iter = repository.0.tag_names(None).expect("tags");

    let tag_iter = tag_iter
        .into_iter()
        .flatten()
        .filter_map(|tag| repository.resolve_tag(tag).ok());

    for tag in tag_iter {
        if let Some(target) = tag.target.as_ref() {
            let target = target.to_string();
            cache.insert(target.clone(), OidOf::Tag(tag.clone()));
            cache.insert(target[0..7].to_string(), OidOf::Tag(tag.clone()));
        }

        if let Some(oid) = tag.oid.as_ref() {
            let string = oid.to_string();
            cache.insert(string.clone(), OidOf::Tag(tag.clone()));
            cache.insert(string[0..7].to_string(), OidOf::Tag(tag.clone()));
        }

        cache.insert(tag.to_string(), OidOf::Tag(tag));
    }

    cache
}

impl Repository {
    fn resolve_tag(&self, tag: &str) -> Result<Tag, TagError> {
        self.0
            .resolve_reference_from_short_name(tag)
            .map_err(|err| TagError::not_found(tag, err))
            .map(|reference| reference.target().unwrap())
            .map(|commit_oid| {
                if let Ok(annotated_tag) = self.0.find_tag(commit_oid) {
                    Tag::from_str(tag, Some(commit_oid), Some(annotated_tag.target_id()))
                } else {
                    Tag::from_str(tag, Some(commit_oid), None)
                }
            })?
    }
}

#[cfg(test)]
mod test {
    use crate::git::repository::Repository;
    use crate::git::rev::cache::init;
    use crate::test_helpers::git_init_no_gpg;
    use cmd_lib::run_cmd;
    use sealed_test::prelude::*;
    use speculoos::prelude::*;

    #[test]
    fn init_cache_ok() -> anyhow::Result<()> {
        let repo = Repository::open(".")?;
        let cache = init(&repo);
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
}
