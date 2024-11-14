use std::{
    collections::BTreeMap,
    sync::{Arc, Mutex},
};

use apub_kernel::{
    model::user::{CreateUser, User},
    repository::user::UserRepository,
};

use crate::persistence::postgres::PostgresDb;

pub struct UserRow {
    name: String,
}

impl From<UserRow> for User {
    fn from(value: UserRow) -> Self {
        let UserRow { name, .. } = value;
        User { name }
    }
}

#[async_trait::async_trait]
impl UserRepository for PostgresDb {
    #[tracing::instrument(skip(self))]
    async fn find_by_name(&self, name: &str) -> anyhow::Result<User> {
        let row = sqlx::query_as!(
            UserRow,
            r#"SELECT name FROM users WHERE users.name = $1"#,
            name
        )
        .fetch_one(self.inner_ref())
        .await?;
        Ok(row.into())
    }

    #[tracing::instrument(skip(self))]
    async fn create(&self, event: CreateUser) -> anyhow::Result<()> {
        sqlx::query!(
            r#"
            INSERT INTO users (name)
            VALUES ($1)
        "#,
            event.name
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
