use apub_shared::{config::AppConfig, model::resource_uri::ResourceUri};
use axum::http::uri;

/// このアプリで扱うユーザの構造体
///
/// `ActivityPub`にするときは別途変換して使う
#[derive(Debug, Clone, PartialEq)]
pub struct User {
    pub name: String,
}

impl User {
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
}

#[derive(Debug, Clone, PartialEq)]
pub struct CreateUser {
    pub name: String,
}

impl From<CreateUser> for User {
    fn from(value: CreateUser) -> Self {
        let CreateUser { name, .. } = value;
        User { name }
    }
}
