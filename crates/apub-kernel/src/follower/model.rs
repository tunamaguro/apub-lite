use apub_shared::model::resource_url::ResourceUrl;

use crate::user::model::UserId;

#[derive(Debug, Clone, PartialEq)]
pub struct Follower {
    pub user_id: UserId,
    pub actor_url: ResourceUrl,
}
