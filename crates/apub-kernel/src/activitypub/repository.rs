use apub_activitypub::model::activity::CreatePersonNote;
use apub_config::AppConfig;
use apub_shared::model::resource_url::ResourceUrl;
use serde::{de::DeserializeOwned, Serialize};

use crate::rsa_key::model::RsaSingingKey;

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

