use crate::oid::OidOf;
use crate::tag::Tag;
use serde::ser::SerializeStruct;
use serde::{Serialize, Serializer};

impl Serialize for Tag {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_str(&self.to_string())
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

#[cfg(test)]
mod test {
    use crate::tag::Tag;
    use git2::Oid;
    use speculoos::prelude::*;

    #[test]
    fn should_serialize_tag() {
        let oid = Oid::from_str("1234567890").unwrap();
        let tag = Tag::from_str("1.0.0", Some(oid)).unwrap();

        let result = serde_json::to_string(&tag);

        assert_that!(result)
            .is_ok()
            .is_equal_to("\"1.0.0\"".to_string())
    }
}
