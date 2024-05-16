use std::fmt::{Display, Formatter};

use git2::Oid;
use serde::ser::SerializeStruct;
use serde::{Serialize, Serializer};

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

impl Serialize for OidOf {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut oidof = serializer.serialize_struct("OidOf", 1)?;
        match self {
            OidOf::Tag(tag) => {
                oidof.serialize_field("tag", &tag.to_string())?;
                if let Some(oid) = tag.oid() {
                    oidof.serialize_field("id", &oid.to_string())?;
                }
            }
            OidOf::FirstCommit(oid) | OidOf::Head(oid) | OidOf::Other(oid) => {
                oidof.serialize_field("id", &oid.to_string())?
            }
        };
        oidof.end()
    }
}
