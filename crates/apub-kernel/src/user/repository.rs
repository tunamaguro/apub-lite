use super::model::{CreateUser, User, UserId};

#[async_trait::async_trait]
pub trait UserRepository: Send + Sync {
    async fn find_by_name(&self, name: &str) -> anyhow::Result<User>;
    async fn find_by_id(&self, id: &UserId) -> anyhow::Result<User>;
    async fn create(&self, event: CreateUser) -> anyhow::Result<()>;
}
