use apub_shared::model::{id::UrlId, resource_url::ResourceUrl};
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;
use typed_builder::TypedBuilder;

use crate::core::{actor::Actor, object::Object};

use super::{context::Context, key::PublicKeyPem};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum AnyActorKind {
    Application(ApplicationKind),
    Group(GroupKind),
    Organization(OrganizationKind),
    Person(PersonKind),
    Service(ServiceKind),
}

impl Default for AnyActorKind {
    fn default() -> Self {
        AnyActorKind::Person(PersonKind::Person)
    }
}

pub trait ActorKind: Serialize + for<'a> Deserialize<'a> + Default {
    fn actor_kind(&self) -> AnyActorKind;
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Default)]
pub enum ApplicationKind {
    #[default]
    Application,
}

impl ActorKind for ApplicationKind {
    fn actor_kind(&self) -> AnyActorKind {
        AnyActorKind::Application(self.clone())
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Default)]
pub enum GroupKind {
    #[default]
    Group,
}

impl ActorKind for GroupKind {
    fn actor_kind(&self) -> AnyActorKind {
        AnyActorKind::Group(self.clone())
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Default)]
pub enum OrganizationKind {
    #[default]
    Organization,
}

impl ActorKind for OrganizationKind {
    fn actor_kind(&self) -> AnyActorKind {
        AnyActorKind::Organization(self.clone())
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Default)]
pub enum PersonKind {
    #[default]
    Person,
}

impl ActorKind for PersonKind {
    fn actor_kind(&self) -> AnyActorKind {
        AnyActorKind::Person(self.clone())
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Default)]
pub enum ServiceKind {
    #[default]
    Service,
}

impl ActorKind for ServiceKind {
    fn actor_kind(&self) -> AnyActorKind {
        AnyActorKind::Service(self.clone())
    }
}

/// Activity Actor Object
///
/// See https://www.w3.org/ns/activitystreams#Person
#[skip_serializing_none]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, TypedBuilder)]
#[serde(rename_all = "camelCase", bound = "Kind:ActorKind")]
pub struct AnyActorImpl<Kind> {
    #[serde(rename = "@context")]
    context: Context,
    id: UrlId<AnyActorImpl<Kind>>,
    #[serde(rename = "type")]
    kind: Kind,
    preferred_username: String,
    inbox: ResourceUrl,
    #[builder(default, setter(strip_option))]
    shared_inbox: Option<ResourceUrl>,
    #[builder(default, setter(strip_option))]
    followers: Option<ResourceUrl>,
}

impl<Kind: ActorKind> Object for AnyActorImpl<Kind> {
    type Kind = Kind;

    fn kind() -> Self::Kind {
        Self::Kind::default()
    }
}

impl<Kind: ActorKind> Actor for AnyActorImpl<Kind> {
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

impl<Kind> AnyActorImpl<Kind> {
    pub fn username(&self) -> &str {
        &self.preferred_username
    }
}

pub type AnyActor = AnyActorImpl<AnyActorKind>;
pub type Application = AnyActorImpl<ApplicationKind>;
pub type Group = AnyActorImpl<GroupKind>;
pub type Organization = AnyActorImpl<OrganizationKind>;
pub type Person = AnyActorImpl<PersonKind>;
pub type Service = AnyActorImpl<ServiceKind>;

pub type AnyActorUrl = UrlId<AnyActor>;
pub type ApplicationUrl = UrlId<Application>;
pub type GroupUrl = UrlId<Group>;
pub type OrganizationUrl = UrlId<Organization>;
pub type PersonUrl = UrlId<Person>;
pub type ServiceUrl = UrlId<Service>;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, TypedBuilder)]
#[serde(rename_all = "camelCase")]
pub struct Security<T> {
    #[serde(flatten)]
    inner: T,
    public_key: PublicKeyPem,
}

impl<T: Object> Object for Security<T> {
    type Kind = <T as Object>::Kind;
}

impl<T: Actor> Actor for Security<T> {
    type Item = <T as Actor>::Item;
    fn id(&self) -> &UrlId<Self::Item> {
        self.inner.id()
    }
    fn inbox(&self) -> &ResourceUrl {
        self.inner.inbox()
    }
    fn outbox(&self) -> Option<&ResourceUrl> {
        self.inner.outbox()
    }
}

impl<T> std::ops::Deref for Security<T> {
    type Target = T;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

pub type SecurityAnyActor = Security<AnyActor>;
pub type SecurityApplication = Security<Application>;
pub type SecurityGroup = Security<Group>;
pub type SecurityOrganization = Security<Organization>;
pub type SecurityPerson = Security<Person>;
pub type SecurityService = Security<Service>;

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
