use apub_shared::model::{id::UrlId, resource_url::ResourceUrl};
use serde::{Deserialize, Serialize};
use typed_builder::TypedBuilder;

use super::person::Person;

/// https://docs.joinmastodon.org/spec/security/#http
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, TypedBuilder)]
#[serde(rename_all = "camelCase")]
pub struct PublicKeyPem {
    id: ResourceUrl,
    owner: UrlId<Person>,
    public_key_pem: String,
}
