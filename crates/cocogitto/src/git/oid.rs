use std::fmt::{Display, Formatter};

use git2::Oid;
use serde::{Serialize, Serializer};

use crate::git::tag::Tag;

/// A wrapper for git2 oid including tags and HEAD ref
#[derive(Debug, PartialEq, Eq, Clone)]
pub struct CommitInfo {
    pub oid: Oid,
    pub tags: Vec<Tag>,
    pub first: bool,
    pub head: bool,
}

impl CommitInfo {
    pub fn new(oid: Oid) -> Self {
        Self {
            oid,
            tags: Vec::new(),
            first: false,
            head: false,
        }
    }

    pub fn into_version(self, package: Option<&str>) -> ReleaseVersion {
        let tag = self
            .tags
            .into_iter()
            .find(|tag| tag.package.as_deref() == package);
        ReleaseVersion { id: self.oid, tag }
    }
}

impl From<Tag> for CommitInfo {
    fn from(value: Tag) -> Self {
        Self {
            oid: value.oid.unwrap_or(Oid::zero()),
            tags: vec![value],
            first: false,
            head: false,
        }
    }
}

impl Display for CommitInfo {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        if let Some(tag) = self.tags.first() {
            tag.fmt(f)
        } else if self.head {
            f.write_str("HEAD")
        } else {
            self.oid.fmt(f)
        }
    }
}

/// A lightweight wrapper for git2 oid
#[derive(Debug, Serialize, Clone, PartialEq, Eq)]
pub struct ReleaseVersion {
    #[serde(serialize_with = "serialize_oid")]
    pub id: Oid,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tag: Option<Tag>,
}

fn serialize_oid<S: Serializer>(oid: &Oid, s: S) -> Result<S::Ok, S::Error> {
    s.serialize_str(&oid.to_string())
}

impl ReleaseVersion {
    pub fn new(oid: Oid) -> Self {
        Self { id: oid, tag: None }
    }
}

impl From<Tag> for ReleaseVersion {
    fn from(value: Tag) -> Self {
        Self {
            id: value.oid.unwrap_or(Oid::zero()),
            tag: Some(value),
        }
    }
}

impl Display for ReleaseVersion {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        if let Some(tag) = &self.tag {
            tag.fmt(f)
        } else {
            self.id.fmt(f)
        }
    }
}
