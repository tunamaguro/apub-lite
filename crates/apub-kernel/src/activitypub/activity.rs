use apub_activitypub::model::activity::CreatePersonNote;
use apub_shared::model::resource_url::ResourceUrl;
use serde::{de::DeserializeOwned, Serialize};

use crate::rsa_key::model::RsaSingingKey;
use apub_config::AppConfig;

pub struct SendActivity<T> {
    pub activity: T,
    pub signer: RsaSingingKey,
    pub inbox: ResourceUrl,
    pub key_uri: ResourceUrl,
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
    /// reqにGetリクエストする
    async fn get_activity<T: DeserializeOwned>(&self, req: &ResourceUrl) -> anyhow::Result<T>;
    /// reqに署名してGetリクエストする
    async fn get_activity_with_sign<T: DeserializeOwned>(
        &self,
        req: &ResourceUrl,
        signer: &RsaSingingKey,
        key_uri: &ResourceUrl,
    ) -> anyhow::Result<T>;

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
