use apub_shared::model::{id::UrlId, resource_url::ResourceUrl};
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;
use typed_builder::TypedBuilder;

use crate::core::{actor::Actor, object::Object};

use super::{context::Context, key::PublicKeyPem};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Default)]
pub enum PersonKind {
    #[default]
    Person,
}

pub type PersonUrl = UrlId<Person>;

/// Activity Person object  
///
/// See https://www.w3.org/ns/activitystreams#Person
#[skip_serializing_none]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, TypedBuilder)]
#[serde(rename_all = "camelCase")]
pub struct Person {
    #[serde(rename = "@context")]
    context: Context,
    id: UrlId<Person>,
    #[serde(rename = "type")]
    #[builder(default)]
    kind: PersonKind,
    preferred_username: String,
    inbox: ResourceUrl,
    #[builder(default,setter(strip_option))]
    shared_inbox: Option<ResourceUrl>,
}

impl Object for Person {
    type Kind = PersonKind;
}

impl Actor for Person {
    type Item = Self;
    fn id(&self) -> &UrlId<Self> {
        &self.id
    }
    fn inbox(&self) -> &ResourceUrl {
        &self.inbox
    }
    fn outbox(&self) -> Option<&ResourceUrl> {
        None
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, TypedBuilder)]
#[serde(rename_all = "camelCase")]
pub struct SecurityPerson {
    #[serde(flatten)]
    person: Person,
    public_key: PublicKeyPem,
}

impl std::ops::Deref for SecurityPerson {
    type Target = Person;
    fn deref(&self) -> &Self::Target {
        &self.person
    }
}

impl Object for SecurityPerson {
    type Kind = PersonKind;
}

impl Actor for SecurityPerson {
    type Item = Person;
    fn id(&self) -> &UrlId<Person> {
        self.person.id()
    }

    fn inbox(&self) -> &ResourceUrl {
        self.person.inbox()
    }
    fn outbox(&self) -> Option<&ResourceUrl> {
        self.person.outbox()
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
