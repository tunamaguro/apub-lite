use apub_shared::model::resource_uri::ResourceUri;
use serde::{Deserialize, Serialize};
use typed_builder::TypedBuilder;

use super::{actor::Actor, context::Context};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Default)]
pub enum PersonKind {
    #[default]
    Person,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, TypedBuilder)]
#[serde(rename_all = "camelCase")]
pub struct Person {
    #[serde(rename = "@context")]
    context: Context,
    id: ResourceUri,
    #[serde(rename = "type")]
    #[builder(default)]
    kind: PersonKind,
    #[serde()]
    preferred_username: String,
    inbox: ResourceUri,
}

impl Actor for Person {
    fn id(&self) -> &ResourceUri {
        &self.id
    }

    fn inbox(&self) -> &ResourceUri {
        &self.inbox
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_deserialize_person() {
        // https://www.w3.org/TR/activitystreams-vocabulary/#dfn-person
        let v = r#"
            {
                "@context": "https://www.w3.org/ns/activitystreams",
                "id": "https://example.com/user/foo",
                "type": "Person",
                "preferredUsername": "foo",
                "inbox": "https://example.com/user/foo/inbox"
            }
        "#;

        let _: Person = serde_json::from_str(v).unwrap();
    }
}
