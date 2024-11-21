use apub_shared::model::resource_uri::ResourceUri;

use super::rsa_key::RsaSingingKey;

pub struct SendActivity<T> {
    pub activity: T,
    pub signer: RsaSingingKey,
    pub inbox: ResourceUri,
    pub key_uri: ResourceUri,
}
