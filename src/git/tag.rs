use crate::SETTINGS;
use anyhow::anyhow;
use anyhow::Result;
use git2::Oid;
use semver::Version;
use std::cmp::Ordering;
use std::fmt;
use std::fmt::Formatter;

#[derive(Debug, PartialEq, Eq)]
pub(crate) struct Tag {
    tag: String,
    oid: Oid,
}

impl Tag {
    pub(crate) fn oid(&self) -> &Oid {
        &self.oid
    }

    pub(crate) fn new(name: &str, oid: Oid) -> Result<Tag> {
        let tag = match SETTINGS.tag_prefix.as_ref() {
            None => Ok(name),
            Some(prefix) => name
                .strip_prefix(prefix)
                .ok_or_else(|| anyhow!("Expected a tag with prefix {}, got {}", prefix, name)),
        }?
        .to_string();

        Ok(Tag { tag, oid })
    }

    pub(crate) fn to_version(&self) -> Result<Version> {
        Version::parse(&self.tag).map_err(|err| anyhow!("{}", err))
    }

    pub(crate) fn to_string_with_prefix(&self) -> String {
        match SETTINGS.tag_prefix.as_ref() {
            None => self.tag.to_string(),
            Some(prefix) => format!("{}{}", prefix, self.tag),
        }
    }
}

impl fmt::Display for Tag {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.to_string_with_prefix())
    }
}

impl Ord for Tag {
    fn cmp(&self, other: &Self) -> Ordering {
        self.partial_cmp(other).unwrap()
    }
}

impl PartialOrd<Tag> for Tag {
    fn partial_cmp(&self, other: &Tag) -> Option<Ordering> {
        Some(self.to_version().ok().cmp(&other.to_version().ok()))
    }
}
