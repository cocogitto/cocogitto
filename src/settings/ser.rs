pub mod commit_types_serde {
    use crate::conventional::commit::CommitConfig;
    use serde::de::{MapAccess, Visitor};
    use serde::ser::SerializeMap;
    use serde::{Deserialize, Deserializer, Serialize, Serializer};
    use std::collections::HashMap;
    use std::fmt;

    #[derive(Debug, Serialize, Deserialize, PartialEq, Eq, Clone)]
    #[serde(untagged)]
    enum CommitConfigOrNull {
        CommitConfig(CommitConfig),
        None {},
    }

    pub fn serialize<S>(
        map: &HashMap<String, Option<CommitConfig>>,
        serializer: S,
    ) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let nullable_map = map
            .iter()
            .map(|(k, v)| {
                (
                    k,
                    v.as_ref()
                        .map(|c| CommitConfigOrNull::CommitConfig(c.clone()))
                        .unwrap_or(CommitConfigOrNull::None {}),
                )
            })
            .collect::<HashMap<&String, CommitConfigOrNull>>();

        let mut map_ser = serializer.serialize_map(Some(nullable_map.len()))?;
        for (key, value) in map {
            map_ser.serialize_entry(key, value)?;
        }
        map_ser.end()
    }

    pub fn deserialize<'de, D>(
        deserializer: D,
    ) -> Result<HashMap<String, Option<CommitConfig>>, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct CustomMapVisitor;

        impl<'de> Visitor<'de> for CustomMapVisitor {
            type Value = HashMap<String, Option<CommitConfig>>;

            fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
                formatter.write_str("a map with string keys and values")
            }

            fn visit_map<A>(self, mut map: A) -> Result<Self::Value, A::Error>
            where
                A: MapAccess<'de>,
            {
                let mut result = HashMap::new();
                while let Some((key, value)) = map.next_entry::<String, CommitConfigOrNull>()? {
                    let value = match value {
                        CommitConfigOrNull::CommitConfig(c) => Some(c),
                        CommitConfigOrNull::None {} => None,
                    };
                    result.insert(key, value);
                }
                Ok(result)
            }
        }

        deserializer.deserialize_map(CustomMapVisitor)
    }
}
