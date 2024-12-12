use apub_activitypub::webfinger::AcctUri;
use apub_shared::model::resource_url::ResourceUrl;
use typed_builder::TypedBuilder;

use crate::user::model::UserId;

#[derive(Debug, Clone, PartialEq, TypedBuilder)]
pub struct Follower {
    pub user_id: UserId,
    pub acct: AcctUri,
    pub actor_url: ResourceUrl,
    pub actor_inbox: ResourceUrl,
}
