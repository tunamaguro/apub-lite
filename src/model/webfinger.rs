use serde::{Deserialize, Serialize};
use typed_builder::TypedBuilder;

use super::resource_uri::ResourceUri;

/// ActivityPub WebFinger  
/// See https://swicg.github.io/activitypub-webfinger
#[derive(Debug, Serialize, Deserialize, PartialEq, TypedBuilder)]
pub struct WebFinger {
    subject: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(setter(strip_option))]
    aliases: Option<Vec<ResourceUri>>,
    links: Vec<WebFingerLink>,
}

#[derive(Debug, Serialize, Deserialize, PartialEq, TypedBuilder)]
pub struct WebFingerLink {
    rel: String,
    ///  Kind is optional because Mastodon may return object without `type`
    #[serde(rename = "type", skip_serializing_if = "Option::is_none")]
    #[builder(setter(strip_option))]
    kind: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(setter(strip_option))]
    href: Option<ResourceUri>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    template: Option<ResourceUri>,
}
