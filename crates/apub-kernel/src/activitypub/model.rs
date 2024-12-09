use apub_config::AppConfig;
use apub_shared::model::resource_url::ResourceUrl;

use crate::rsa_key::model::RsaSingingKey;

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
