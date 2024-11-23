use apub_activitypub::model::{
    context::Context,
    person::{Person, PersonUrl},
};
use apub_config::AppConfig;
use apub_shared::model::{id::Id, resource_url::ResourceUrl};
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
    pub fn user_uri(&self, config: &AppConfig) -> PersonUrl {
        let user_uri = config
            .host_uri()
            .clone()
            .set_path(&format!("/users/{}", self.name))
            .to_owned();
        user_uri.into()
    }

    /// `/users/{username}/inbox`
    pub fn inbox_uri(&self, config: &AppConfig) -> ResourceUrl {
        let inbox_uri = config
            .host_uri()
            .clone()
            .set_path(&format!("/users/{}/inbox", self.name))
            .to_owned();
        inbox_uri
    }

    pub fn user_key_uri<T>(&self, config: &AppConfig) -> ResourceUrl
    where
        T: KeyType,
    {
        let key_uri = config
            .host_uri()
            .clone()
            .set_path(&format!("/users/{}", self.name))
            .set_fragment(T::key_type())
            .to_owned();
        key_uri
    }

    /// Create Person actor
    pub fn to_person(&self, config: &AppConfig) -> Person {
        Person::builder()
            .id(self.user_uri(config))
            .preferred_username(self.name.clone())
            .inbox(self.inbox_uri(config))
            .context(Context::activity_context_url().clone().into())
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
