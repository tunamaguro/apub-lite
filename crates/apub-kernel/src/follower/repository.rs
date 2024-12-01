use apub_shared::model::resource_url::ResourceUrl;

use crate::user::model::UserId;

use super::model::Follower;

#[async_trait::async_trait]
pub trait FollowerRepository: Send + Sync {
    async fn find(&self, user_id: &UserId, actor_url: &ResourceUrl) -> anyhow::Result<bool>;
    async fn find_followee(&self, user_id: &UserId) -> anyhow::Result<Vec<Follower>>;
    async fn create(&self, user_id: &UserId, actor_url: &ResourceUrl) -> anyhow::Result<()>;
    async fn delete(&self, user_id: &UserId, actor_url: &ResourceUrl) -> anyhow::Result<()>;
}
