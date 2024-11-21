use apub_activitypub::model::activity::CreateNote;
use apub_shared::{config::AppConfig, model::resource_uri::ResourceUri};
use axum::http::uri;

use crate::model::rsa_key::RsaSingingKey;

#[async_trait::async_trait]
pub trait ActivityRepository: Send + Sync {
    /// `Create``Note`を`inbox`に送信
    async fn post_note(
        &self,
        activity: &CreateNote,
        inbox: &ResourceUri,
        signer: &RsaSingingKey,
        key_uri: &ResourceUri,
    ) -> anyhow::Result<()>;
}

pub fn generate_note_uri(config: &AppConfig) -> ResourceUri {
    let id = uuid::Uuid::now_v7();
    let host_uri = config.host_uri();

    let note_uri = uri::Builder::new()
        .scheme(host_uri.scheme().clone())
        .authority(host_uri.host())
        .path_and_query(format!("/notes/{}", id))
        .build()
        .unwrap();

    ResourceUri::try_from(note_uri).unwrap()
}

pub fn generate_acitivity_uri(config: &AppConfig) -> ResourceUri {
    let id = uuid::Uuid::now_v7();
    let host_uri = config.host_uri();

    let note_uri = uri::Builder::new()
        .scheme(host_uri.scheme().clone())
        .authority(host_uri.host())
        .path_and_query(format!("/activities/{}", id))
        .build()
        .unwrap();

    ResourceUri::try_from(note_uri).unwrap()
}