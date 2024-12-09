use apub_kernel::{
    prelude::UserRepository,
    user::model::{CreateUser, User, UserId},
};

use crate::{model::user::UserRow, persistence::postgres::PostgresDb};

#[async_trait::async_trait]
impl UserRepository for PostgresDb {
    #[tracing::instrument(skip(self))]
    async fn find_by_name(&self, name: &str) -> anyhow::Result<User> {
        let row = sqlx::query_as!(
            UserRow,
            r#"
            SELECT
                user_id, name 
            FROM
                users 
            WHERE
                users.name = $1"#,
            name
        )
        .fetch_one(self.inner_ref())
        .await?;
        Ok(row.into())
    }

    #[tracing::instrument(skip(self))]
    async fn find_by_id(&self, id: &UserId) -> anyhow::Result<User> {
        let row = sqlx::query_as!(
            UserRow,
            r#"SELECT
                user_id, name 
            FROM 
                users 
            WHERE
                users.user_id = $1"#,
            **id
        )
        .fetch_one(self.inner_ref())
        .await?;
        Ok(row.into())
    }

    #[tracing::instrument(skip(self))]
    async fn create(&self, event: CreateUser) -> anyhow::Result<()> {
        let user = User::from(event);
        sqlx::query!(
            r#"
            INSERT INTO users 
                (user_id, name)
            VALUES
                ($1, $2)
        "#,
            *user.id,
            user.name
        )
        .execute(self.inner_ref())
        .await?;

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    use pretty_assertions::assert_eq;

    #[sqlx::test]
    async fn test_register_user(pool: sqlx::PgPool) -> anyhow::Result<()> {
        let repo = PostgresDb::new(pool);
        let user = CreateUser {
            name: "john".to_string(),
        };
        repo.create(user).await?;

        let res = repo.find_by_name("john").await?;

        assert_eq!(res.name, "john");

        Ok(())
    }
}
