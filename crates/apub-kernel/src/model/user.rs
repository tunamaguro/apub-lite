use apub_activitypub::model::person::Person;
use apub_shared::{
    config::AppConfig,
    model::{id::Id, resource_uri::ResourceUri},
};
use axum::http::uri;
use typed_builder::TypedBuilder;

use super::rsa_key::KeyType;

pub type UserId = Id<User>;

/// このアプリで扱うユーザの構造体
///
/// `ActivityPub`にするときは別途変換して使う
#[derive(Debug, Clone, PartialEq, TypedBuilder)]
pub struct User {
    pub id: UserId,
    pub name: String,
}

impl User {
    /// `/users/{username}`
    pub fn user_uri(&self, config: &AppConfig) -> ResourceUri {
        let host_uri = config.host_uri();
        let user_uri = uri::Builder::new()
            .scheme(host_uri.scheme().clone())
            .authority(host_uri.host())
            .path_and_query(format!("/users/{}", self.name))
            .build()
            .unwrap();

        ResourceUri::try_from(user_uri).unwrap()
    }

    /// `/users/{username}/inbox`
    pub fn inbox_uri(&self, config: &AppConfig) -> ResourceUri {
        let host_uri = config.host_uri();
        let inbox_uri = uri::Builder::new()
            .scheme(host_uri.scheme().clone())
            .authority(host_uri.host())
            .path_and_query(format!("/users/{}/inbox", self.name))
            .build()
            .unwrap();

        ResourceUri::try_from(inbox_uri).unwrap()
    }

    pub fn user_key_uri<T>(&self, config: &AppConfig) -> ResourceUri
    where
        T: KeyType,
    {
        let host_uri = config.host_uri();
        let key_uri = uri::Builder::new()
            .scheme(host_uri.scheme().clone())
            .authority(host_uri.host())
            .path_and_query(format!("/users/{}#{}", self.name, T::key_type()))
            .build()
            .unwrap();
        ResourceUri::try_from(key_uri).unwrap()
    }

    /// Create Person actor
    pub fn to_person(&self, config: &AppConfig) -> Person {
        Person::builder()
            .id(self.user_uri(config))
            .preferred_username(self.name.clone())
            .inbox(self.inbox_uri(config))
            .context(vec!["https://www.w3.org/ns/activitystreams".parse().unwrap()].into())
            .build()
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct CreateUser {
    pub name: String,
}

impl From<CreateUser> for User {
    fn from(value: CreateUser) -> Self {
        let CreateUser { name, .. } = value;
        let id = Id::new();
        User { id, name }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use pretty_assertions::assert_eq;
    use uuid::uuid;

    fn test_user() -> User {
        User {
            id: uuid!("0193351b-82ce-7c2d-9bb6-4a71b1b62c44").into(),
            name: "foo".to_string(),
        }
    }

    fn test_config() -> AppConfig {
        AppConfig::new("https://example.com")
    }

    #[test]
    fn test_user_uri() {
        let user = test_user().user_uri(&test_config());
        assert_eq!(user, "https://example.com/users/foo".parse().unwrap())
    }

    #[test]
    fn test_user_inbox() {
        let inbox = test_user().inbox_uri(&test_config());
        assert_eq!(
            inbox,
            "https://example.com/users/foo/inbox".parse().unwrap()
        )
    }
}
