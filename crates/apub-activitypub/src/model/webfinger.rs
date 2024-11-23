use serde::{Deserialize, Serialize};
use typed_builder::TypedBuilder;

use apub_shared::model::resource_uri::ResourceUrl;

/// ActivityPub WebFinger
///
/// See https://swicg.github.io/activitypub-webfinger
#[derive(Debug, Serialize, Deserialize, PartialEq, TypedBuilder)]
pub struct WebFinger {
    subject: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(setter(strip_option))]
    aliases: Option<Vec<ResourceUrl>>,
    links: Vec<WebFingerLink>,
}

/// WebFinger link item
#[derive(Debug, Serialize, Deserialize, PartialEq, TypedBuilder)]
pub struct WebFingerLink {
    rel: String,
    ///  Kind is optional because Mastodon may return object without `type`
    #[serde(rename = "type", skip_serializing_if = "Option::is_none")]
    #[builder(setter(strip_option))]
    kind: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(setter(strip_option))]
    href: Option<ResourceUrl>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    template: Option<ResourceUrl>,
}
