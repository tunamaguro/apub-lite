use apub_kernel::{
    follower::{model::Follower, repository::FollowerRepository},
    user::model::UserId,
};
use apub_shared::model::resource_url::ResourceUrl;

use crate::{
    model::follower::{FollowerCount, FollowerRow},
    persistence::postgres::PostgresDb,
};

#[async_trait::async_trait]
impl FollowerRepository for PostgresDb {
    async fn find(&self, user_id: &UserId, actor_url: &ResourceUrl) -> anyhow::Result<bool> {
        let r = sqlx::query_as!(
            FollowerCount,
            r#"
            SELECT COUNT(*) AS count
            FROM followers
            WHERE
                followers.user_id = $1 AND followers.actor_url = $2
        "#,
            user_id.as_ref(),
            actor_url.as_str()
        )
        .fetch_one(self.inner_ref())
        .await?;

        Ok(r.count() == 1)
    }

    async fn find_followee(&self, user_id: &UserId) -> anyhow::Result<Vec<Follower>> {
        let rows = sqlx::query_as!(
            FollowerRow,
            r#"
            SELECT user_id,actor_url
            FROM followers
            WHERE followers.user_id = $1
            "#,
            user_id.as_ref()
        )
        .fetch_all(self.inner_ref())
        .await?;

        let followers = rows
            .into_iter()
            .filter_map(|row| Follower::try_from(row).ok())
            .collect();

        Ok(followers)
    }

    async fn create(&self, event: &Follower) -> anyhow::Result<()> {
        let _count = sqlx::query!(
            r#"
            INSERT INTO followers (user_id, actor_url) 
            VALUES ($1,$2)
            "#,
            event.user_id.as_ref(),
            event.actor_url.as_str()
        )
        .execute(self.inner_ref())
        .await?;

        Ok(())
    }

    async fn delete(&self, user_id: &UserId, actor_url: &ResourceUrl) -> anyhow::Result<()> {
        let count = sqlx::query!(
            r#"
            DELETE FROM followers 
            WHERE 
                followers.user_id = $1 AND followers.actor_url = $2
        "#,
            user_id.as_ref(),
            actor_url.as_str()
        )
        .execute(self.inner_ref())
        .await?;

        if count.rows_affected() != 1 {
            Err(anyhow::anyhow!("No rows deleted"))
        } else {
            Ok(())
        }
    }
}
