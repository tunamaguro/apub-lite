use apub_shared::model::{id::UriId, resource_uri::ResourceUri};
use serde::{Deserialize, Serialize};
use typed_builder::TypedBuilder;

use super::person::Person;

/// https://docs.joinmastodon.org/spec/security/#http
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, TypedBuilder)]
#[serde(rename_all = "camelCase")]
pub struct PublicKeyPem {
    id: ResourceUri,
    owner: UriId<Person>,
    public_key_pem: String,
}
