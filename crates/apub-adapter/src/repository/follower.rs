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

#[cfg(test)]
mod tests {
    use std::sync::LazyLock;

    use super::*;
    use apub_kernel::user::model::User;
    use apub_shared::model::id::Id;
    use pretty_assertions::assert_eq;

    static USER_ID: LazyLock<Id<User>> =
        LazyLock::new(|| "ce68da5d-692e-4c9c-ab36-3322dd6bf214".parse::<_>().unwrap());
    static ALICE_URL: LazyLock<ResourceUrl> =
        LazyLock::new(|| "https://example.com/users/alice".parse::<_>().unwrap());
    static BOB_URL: LazyLock<ResourceUrl> =
        LazyLock::new(|| "https://sub1.example.com/users/bob".parse::<_>().unwrap());
    static CHARLIE_URL: LazyLock<ResourceUrl> = LazyLock::new(|| {
        "https://sub2.example.com/users/charlie"
            .parse::<_>()
            .unwrap()
    });
    #[sqlx::test(fixtures(path = "fixtures", scripts("users", "actors")))]
    async fn test_add_followers(pool: sqlx::PgPool) {
        let repo = PostgresDb::new(pool);

        // Add follower
        repo.create(&USER_ID, &ALICE_URL).await.unwrap();
        repo.create(&USER_ID, &BOB_URL).await.unwrap();

        // find follower
        assert!(repo.find(&USER_ID, &ALICE_URL).await.unwrap());
        assert!(repo.find(&USER_ID, &BOB_URL).await.unwrap());
        assert!(!repo.find(&USER_ID, &CHARLIE_URL).await.unwrap());

        let list = repo.find_followee(&USER_ID).await.unwrap();
        assert_eq!(list.len(), 2)
    }

    #[sqlx::test(fixtures(path = "fixtures", scripts("users", "actors")))]
    async fn test_delete_followers(pool: sqlx::PgPool) {
        let repo = PostgresDb::new(pool);

        // Add follower
        repo.create(&USER_ID, &ALICE_URL).await.unwrap();
        repo.create(&USER_ID, &BOB_URL).await.unwrap();

        let list = repo.find_followee(&USER_ID).await.unwrap();
        assert_eq!(list.len(), 2);

        repo.delete(&USER_ID, &ALICE_URL).await.unwrap();

        let list = repo.find_followee(&USER_ID).await.unwrap();
        assert_eq!(list.len(), 1);
        let bob = list.first().unwrap();
        assert_eq!(
            bob.inbox.as_str(),
            "https://sub1.example.com/users/bob/inbox"
        )
    }
}
