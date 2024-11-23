use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;
use typed_builder::TypedBuilder;

use apub_shared::model::resource_url::ResourceUrl;

/// ActivityPub WebFinger
///
/// See https://swicg.github.io/activitypub-webfinger
#[skip_serializing_none]
#[derive(Debug, Serialize, Deserialize, PartialEq, TypedBuilder)]
pub struct WebFinger {
    /// リソースの識別子(e.g. `acct:username@example.com`)
    ///
    /// See https://datatracker.ietf.org/doc/html/rfc7033#section-4.4.1
    subject: String,
    /// リソースの別名リスト
    ///
    /// See https://datatracker.ietf.org/doc/html/rfc7033#section-4.4.2
    #[builder(setter(strip_option))]
    aliases: Option<Vec<ResourceUrl>>,
    /// リソースに関連するリンクのリスト
    ///
    /// See https://datatracker.ietf.org/doc/html/rfc7033#section-4.4.4
    links: Vec<WebFingerLink>,
}

/// WebFinger link item
#[skip_serializing_none]
#[derive(Debug, Serialize, Deserialize, PartialEq, TypedBuilder)]
pub struct WebFingerLink {
    /// リンクの関係性を示す識別子(e.g. `self`, `http://webfinger.net/rel/profile-page`)
    ///
    /// See
    /// - https://datatracker.ietf.org/doc/html/rfc7033#section-4.4.4.1
    /// - https://webfinger.net/rel/#profile-page
    rel: String,
    /// リンク先のメディアタイプ(e.g. `text/html`、`application/activity+json`)  
    /// Mastodonがつけてこないことがあるので、`Option`
    ///
    /// See
    /// - https://datatracker.ietf.org/doc/html/rfc7033#section-4.4.4.2
    /// - https://docs.joinmastodon.org/spec/webfinger/
    #[serde(rename = "type")]
    #[builder(setter(strip_option))]
    kind: Option<String>,
    /// リンク先のURL
    ///
    /// See https://datatracker.ietf.org/doc/html/rfc7033#section-4.4.4.3
    #[builder(setter(strip_option))]
    href: Option<ResourceUrl>,
    /// 何らかのプレースホルダを持つURL(e.g. `https://mastodon.social/authorize_interaction?uri={uri}`)
    ///
    /// See https://docs.joinmastodon.org/spec/webfinger/#example
    #[builder(default)]
    template: Option<ResourceUrl>,
}
