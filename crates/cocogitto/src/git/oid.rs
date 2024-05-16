use std::fmt::{Display, Formatter};

use git2::Oid;

use cocogitto_tag::Tag;

/// A wrapper for git2 oid including tags and HEAD ref
#[derive(Debug, PartialEq, Eq, Clone)]
pub enum OidOf {
    Tag(Tag),
    Head(Oid),
    Other(Oid),
    FirstCommit(Oid),
}

impl OidOf {
    pub fn oid(&self) -> &Oid {
        match self {
            OidOf::Tag(t) => t.oid_unchecked(),
            OidOf::Head(o) | OidOf::Other(o) | OidOf::FirstCommit(o) => o,
        }
    }
}

impl Display for OidOf {
    /// Print the oid according to it's type
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            OidOf::Tag(tag) => write!(f, "{tag}"),
            OidOf::Head(_) => write!(f, "HEAD"),
            OidOf::Other(oid) => write!(f, "{}", &oid.to_string()),
            OidOf::FirstCommit(oid) => write!(f, "{}", &oid.to_string()),
        }
    }
}
