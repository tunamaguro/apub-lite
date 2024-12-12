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
    #[tracing::instrument(skip(self))]
    async fn find(&self, user_id: &UserId, actor_url: &ResourceUrl) -> anyhow::Result<bool> {
        let r = sqlx::query_as!(
            FollowerCount,
            r#"
            SELECT 
                COUNT(*) AS count
            FROM 
                actor_follows
            LEFT JOIN
                actors
            ON
                actor_follows.follower_actor_id = actors.actor_id
            WHERE
                actor_follows.followed_user_id = $1 AND actors.actor_url = $2
        "#,
            user_id.as_ref(),
            actor_url.as_str()
        )
        .fetch_one(self.inner_ref())
        .await?;

        Ok(r.count() == 1)
    }

    #[tracing::instrument(skip(self))]
    async fn find_followee(&self, user_id: &UserId) -> anyhow::Result<Vec<Follower>> {
        let user_id: &sqlx::types::Uuid = user_id.as_ref();
        let rows = sqlx::query_as!(
            FollowerRow,
            r#"
            SELECT
                actor_follows.followed_user_id AS user_id,
                actors.actor_url AS follower_url,
                actors.host AS host,
                actors.preferred_username AS preferred_username,
                actors.inbox_url AS inbox_url
            FROM
                actor_follows
            LEFT JOIN
                actors
            ON 
                actor_follows.follower_actor_id = actors.actor_id
            WHERE
                actor_follows.followed_user_id = $1
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

    #[tracing::instrument(skip(self))]
    async fn create(&self, user_id: &UserId, actor_url: &ResourceUrl) -> anyhow::Result<()> {
        let count = sqlx::query!(
            r#"
            INSERT INTO actor_follows 
                (followed_user_id, follower_actor_id) 
            VALUES
                (
                $1,
                (SELECT actor_id FROM actors WHERE actor_url = $2)
             )
            "#,
            user_id.as_ref(),
            actor_url.as_str()
        )
        .execute(self.inner_ref())
        .await
        .inspect_err(|e| tracing::error!(%e))?;

        if count.rows_affected() != 1 {
            return Err(anyhow::anyhow!("follower is not added"));
        }

        Ok(())
    }

    #[tracing::instrument(skip(self))]
    async fn delete(&self, user_id: &UserId, actor_url: &ResourceUrl) -> anyhow::Result<()> {
        let count = sqlx::query!(
            r#"
            DELETE FROM 
                actor_follows 
            WHERE 
                actor_follows.followed_user_id = $1 
                AND actor_follows.follower_actor_id in (SELECT actor_id FROM actors WHERE actor_url = $2)
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

