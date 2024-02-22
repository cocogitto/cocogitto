use git2::Commit;

use std::collections::BTreeMap;
use std::sync::OnceLock;

use crate::git::oid::OidOf;
use crate::git::repository::Repository;

pub mod filters;
pub mod revspec;
pub mod revwalk;

pub static REPO_CACHE: OnceLock<BTreeMap<String, OidOf>> = OnceLock::new();

fn init_cache(repository: &Repository) -> &BTreeMap<String, OidOf> {
    REPO_CACHE.get_or_init(|| {
        let mut cache = BTreeMap::new();
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
    })
}

#[derive(Debug)]
pub struct CommitIter<'repo>(Vec<(OidOf, Commit<'repo>)>);

impl<'repo> CommitIter<'repo> {
    pub fn from_oid(&self) -> OidOf {
        self.0.last().expect("non empty commit range").0.clone()
    }

    pub fn to_oid(&self) -> OidOf {
        self.0.first().expect("non empty commit range").0.clone()
    }

    pub fn iter_commits(&self) -> impl Iterator<Item = &Commit> {
        self.0.iter().map(|(_, commit)| commit)
    }
}

impl<'repo> IntoIterator for CommitIter<'repo> {
    type Item = (OidOf, Commit<'repo>);
    type IntoIter = std::vec::IntoIter<Self::Item>;

    fn into_iter(self) -> Self::IntoIter {
        self.0.into_iter()
    }
}

#[cfg(test)]
mod test {
    use crate::git::repository::Repository;
    use crate::git::rev::{init_cache, REPO_CACHE};

    #[test]
    fn init_cache_ok() -> anyhow::Result<()> {
        let repo = Repository::open(".")?;
        init_cache(&repo);
        let x = REPO_CACHE.get().unwrap();
        for tag in x {
            println!("{:?}", tag);
        }
        Ok(())
    }
}
