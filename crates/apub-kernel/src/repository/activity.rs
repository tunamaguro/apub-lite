use apub_activitypub::model::activity::CreatePersonNote;
use apub_config::AppConfig;
use apub_shared::model::resource_url::ResourceUrl;
use serde::Serialize;

use crate::model::rsa_key::RsaSingingKey;

#[async_trait::async_trait]
pub trait ActivityRepository: Send + Sync {
    /// Activityに署名して`inbox`に`post`する
    async fn post_activity<T: Serialize + Sync>(
        &self,
        activity: &T,
        inbox: &ResourceUrl,
        signer: &RsaSingingKey,
        key_uri: &ResourceUrl,
    ) -> anyhow::Result<()>;
    /// `Create``Note`を`inbox`に送信
    async fn post_note(
        &self,
        activity: &CreatePersonNote,
        inbox: &ResourceUrl,
        signer: &RsaSingingKey,
        key_uri: &ResourceUrl,
    ) -> anyhow::Result<()> {
        self.post_activity(activity, inbox, signer, key_uri).await
    }
}

pub fn generate_note_uri(config: &AppConfig) -> ResourceUrl {
    let id = uuid::Uuid::now_v7();
    let note_uri = config
        .host_uri()
        .clone()
        .set_path(&format!("/notes/{}", id))
        .to_owned();

    note_uri
}

pub fn generate_activity_uri(config: &AppConfig) -> ResourceUrl {
    let id = uuid::Uuid::now_v7();
    let activity_uri = config
        .host_uri()
        .clone()
        .set_path(&format!("/activities/{}", id))
        .to_owned();

    activity_uri
}
