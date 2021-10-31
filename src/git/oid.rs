use std::fmt::{Display, Formatter};

use git2::Oid;

use crate::git::tag::Tag;

/// A wrapper for git2 oid including tags and HEAD ref
#[derive(Debug, PartialEq, Eq)]
pub enum OidOf {
    Tag(Tag),
    Head(Oid),
    Other(Oid),
}

impl Display for OidOf {
    /// Print the oid according to it's type
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            OidOf::Tag(tag) => write!(f, "{}", tag),
            OidOf::Head(_) => write!(f, "HEAD"),
            OidOf::Other(oid) => write!(f, "{}", &oid.to_string()[0..6]),
        }
    }
}
