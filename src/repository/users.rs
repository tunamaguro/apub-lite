use apub_kernel::model::user::{CreateUser, User};

#[async_trait::async_trait]
pub trait UserRepository: Send + Sync {
    async fn find_by_name(&self, name: &str) -> anyhow::Result<User>;
    async fn create(&self, event: CreateUser) -> anyhow::Result<()>;
}
