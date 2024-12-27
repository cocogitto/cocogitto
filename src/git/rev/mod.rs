use git2::Commit;

use crate::git::oid::OidOf;

pub mod cache;
pub mod filters;
pub mod revspec;
pub mod revwalk;

#[derive(Debug)]
pub struct CommitIter<'repo>(Vec<(OidOf, Commit<'repo>)>);

impl CommitIter<'_> {
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
