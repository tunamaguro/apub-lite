use crate::{
    model::follower::{FollowerCount, FollowerRow},
    persistence::postgres::PostgresDb,
};
use apub_kernel::{
    follower::{model::Follower, repository::FollowerRepository},
    user::model::UserId,
};
use apub_shared::model::resource_url::ResourceUrl;

#[async_trait::async_trait]
impl FollowerRepository for PostgresDb {
    async fn find(&self, user_id: &UserId, actor_url: &ResourceUrl) -> anyhow::Result<bool> {
        let r = sqlx::query_as!(
            FollowerCount,
            r#"
            SELECT 
                COUNT(*) AS count
            FROM 
                actors
            LEFT JOIN
                actor_follows
            ON
                actors.local_user_id = actor_follows.followed_actor_id
            WHERE
                actors.local_user_id = $1 AND actor_follows.follower_actor_url = $2
        "#,
            user_id.as_ref(),
            actor_url.as_str()
        )
        .fetch_one(self.inner_ref())
        .await?;

        Ok(r.count() == 1)
    }

    async fn find_followee(&self, user_id: &UserId) -> anyhow::Result<Vec<Follower>> {
        let user_id: &sqlx::types::Uuid = user_id.as_ref();
        let rows = sqlx::query_as!(
            FollowerRow,
            r#"
            SELECT
                actors.local_user_id AS user_id, actor_url AS follower_url
            FROM
                actors
            LEFT JOIN
                actor_follows
            ON 
                actors.actor_id = actor_follows.followed_actor_id            
            WHERE
                actors.local_user_id = $1
            "#,
            user_id
        )
        .fetch_all(self.inner_ref())
        .await?;

        let followers = rows
            .into_iter()
            .filter_map(|row| Follower::try_from(row).ok())
            .collect();

        Ok(followers)
    }

    async fn create(&self, user_id: &UserId, actor_url: &ResourceUrl) -> anyhow::Result<()> {
        let _count = sqlx::query!(
            r#"
            INSERT INTO actor_follows 
                (followed_actor_id, follower_actor_url) 
            VALUES 
                ($1,$2)
            "#,
            user_id.as_ref(),
            actor_url.as_str()
        )
        .execute(self.inner_ref())
        .await?;

        Ok(())
    }

    async fn delete(&self, user_id: &UserId, actor_url: &ResourceUrl) -> anyhow::Result<()> {
        let count = sqlx::query!(
            r#"
            DELETE FROM 
                actor_follows 
            WHERE 
                actor_follows.followed_actor_id = $1 AND actor_follows.follower_actor_url = $2
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
