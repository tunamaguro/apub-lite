use apub_shared::model::resource_uri::ResourceUrl;
use serde::{Deserialize, Serialize};
use typed_builder::TypedBuilder;

use super::{actor::Actor, context::Context, key::PublicKeyPem};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Default)]
pub enum PersonKind {
    #[default]
    Person,
}

/// Activity Person object  
///
/// See https://www.w3.org/ns/activitystreams#Person
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, TypedBuilder)]
#[serde(rename_all = "camelCase")]
pub struct Person {
    #[serde(rename = "@context")]
    context: Context,
    id: ResourceUrl,
    #[serde(rename = "type")]
    #[builder(default)]
    kind: PersonKind,
    preferred_username: String,
    inbox: ResourceUrl,
}

impl Actor for Person {
    fn id(&self) -> &ResourceUrl {
        &self.id
    }

    fn inbox(&self) -> &ResourceUrl {
        &self.inbox
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, TypedBuilder)]
#[serde(rename_all = "camelCase")]
pub struct SecurityPerson {
    #[serde(flatten)]
    person: Person,
    public_key: PublicKeyPem,
}

impl Actor for SecurityPerson {
    fn id(&self) -> &ResourceUrl {
        self.person.id()
    }

    fn inbox(&self) -> &ResourceUrl {
        self.person.inbox()
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
