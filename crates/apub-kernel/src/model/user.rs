use apub_shared::{config::AppConfig, model::resource_uri::ResourceUri};
use axum::http::uri;
use typed_builder::TypedBuilder;
use uuid::Uuid;

/// このアプリで扱うユーザの構造体
///
/// `ActivityPub`にするときは別途変換して使う
#[derive(Debug, Clone, PartialEq, TypedBuilder)]
pub struct User {
    pub id: Uuid,
    pub name: String,
}

impl User {
    /// `/users/{username}`
    pub fn users_uri(&self, config: &AppConfig) -> ResourceUri {
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
}

#[derive(Debug, Clone, PartialEq)]
pub struct CreateUser {
    pub name: String,
}

impl From<CreateUser> for User {
    fn from(value: CreateUser) -> Self {
        let CreateUser { name, .. } = value;
        let id = Uuid::now_v7();
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
            id: uuid!("0193351b-82ce-7c2d-9bb6-4a71b1b62c44"),
            name: "foo".to_string(),
        }
    }

    fn test_config() -> AppConfig {
        AppConfig::new("https://example.com")
    }

    #[test]
    fn test_user_uri() {
        let user = test_user().users_uri(&test_config());
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
