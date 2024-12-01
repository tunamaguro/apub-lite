use apub_shared::model::resource_url::ResourceUrl;

use crate::rsa_key::model::RsaSingingKey;



pub struct SendActivity<T> {
    pub activity: T,
    pub signer: RsaSingingKey,
    pub inbox: ResourceUrl,
    pub key_uri: ResourceUrl,
}
