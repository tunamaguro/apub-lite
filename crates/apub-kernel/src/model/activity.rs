use apub_shared::model::resource_url::ResourceUrl;

use super::rsa_key::RsaSingingKey;

pub struct SendActivity<T> {
    pub activity: T,
    pub signer: RsaSingingKey,
    pub inbox: ResourceUrl,
    pub key_uri: ResourceUrl,
}
