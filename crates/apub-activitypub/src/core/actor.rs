use apub_shared::model::{id::UrlId, resource_url::ResourceUrl};

use super::object::Object;

/// 自身がActivity Stream2.0の`Actor`を継承していることを表明する
///
/// See https://www.w3.org/TR/activitystreams-core/#actors
pub trait Actor: Object {
    type Item;
    /// 自身を示す一意なURL
    fn id(&self) -> &UrlId<Self::Item>;
    /// `Actor`がアクティビティを受け取るエンドポイントのURL
    fn inbox(&self) -> &ResourceUrl;
    /// `Actor`が送信したアクティビティを公開するエンドポイントのURL
    fn outbox(&self) -> Option<&ResourceUrl>;
}
