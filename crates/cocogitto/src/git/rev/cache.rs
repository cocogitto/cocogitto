use crate::git::oid::OidOf;
use crate::git::repository::Repository;
use cocogitto_config::SETTINGS;
use cocogitto_tag::error::TagError;
use cocogitto_tag::Tag;
use once_cell::sync::{Lazy, OnceCell};
use std::collections::BTreeMap;
use std::sync::{Arc, Mutex, MutexGuard};

static REPO_CACHE: Lazy<Arc<Mutex<BTreeMap<String, OidOf>>>> =
    Lazy::new(|| Arc::new(Mutex::new(BTreeMap::new())));

static FIRST_COMMIT: OnceCell<OidOf> = OnceCell::new();

pub(crate) fn get_cache(repository: &Repository) -> MutexGuard<'_, BTreeMap<String, OidOf>> {
    let mut cache = REPO_CACHE.lock().unwrap();
    if cache.is_empty() {
        let head = repository.get_head_commit().expect("HEAD");
        let first = FIRST_COMMIT.get_or_init(|| {
            OidOf::FirstCommit(repository.get_first_commit().expect("first commit"))
        });

        cache.insert(head.id().to_string(), OidOf::Head(head.id()));
        cache.insert(
            first.to_string(),
            FIRST_COMMIT.get().expect("first commit").clone(),
        );

        let tag_iter = repository.0.tag_names(None).expect("tags");

        let tag_iter = tag_iter
            .into_iter()
            .flatten()
            .filter_map(|tag| repository.resolve_tag(tag).ok());

        for tag in tag_iter {
            if let Some(target) = tag.target.as_ref() {
                let target = target.to_string();
                cache.insert(target, OidOf::Tag(tag.clone()));
            }

            if let Some(oid) = tag.oid.as_ref() {
                let oid = oid.to_string();
                cache.insert(oid, OidOf::Tag(tag.clone()));
            }

            cache.insert(tag.to_string(), OidOf::Tag(tag));
        }
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
                    Tag::from_str(
                        tag,
                        Some(commit_oid),
                        Some(annotated_tag.target_id()),
                        SETTINGS.tag_prefix(),
                        SETTINGS.monorepo_separator(),
                        SETTINGS.package_names(),
                    )
                } else {
                    Tag::from_str(
                        tag,
                        Some(commit_oid),
                        None,
                        SETTINGS.tag_prefix(),
                        SETTINGS.monorepo_separator(),
                        SETTINGS.package_names(),
                    )
                }
            })?
    }
}

#[cfg(test)]
mod test {

    use crate::git::repository::Repository;
    use crate::git::rev::cache::get_cache;
    use crate::test_helpers::git_init_no_gpg;
    use cargo_metadata::MetadataCommand;
    use cmd_lib::run_cmd;
    use sealed_test::prelude::*;
    use speculoos::prelude::*;

    #[test]
    fn init_cache_ok() -> anyhow::Result<()> {
        let metadata = MetadataCommand::new()
            .exec()
            .expect("Failed to get cargo metadata");
        let workspace = metadata.workspace_root;

        let repo = Repository::open(&workspace)?;
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
}
