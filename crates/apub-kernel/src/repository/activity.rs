use apub_activitypub::model::activity::CreateNote;
use apub_shared::{config::AppConfig, model::resource_uri::ResourceUri};

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
    let note_uri = config
        .host_uri()
        .clone()
        .set_path(&format!("/notes/{}", id))
        .to_owned();

    note_uri
}

pub fn generate_activity_uri(config: &AppConfig) -> ResourceUri {
    let id = uuid::Uuid::now_v7();
    let activity_uri = config
        .host_uri()
        .clone()
        .set_path(&format!("/activities/{}", id))
        .to_owned();

    activity_uri
}
