use sqlx::{postgres::PgPoolOptions, PgPool};

/// `Pool`は内部で`Arc`を使っているのでここで包む必要はない
#[derive(Clone, Debug)]
pub struct PostgresDb(PgPool);

impl PostgresDb {
    pub fn new(pool: PgPool) -> Self {
        Self(pool)
    }

    pub async fn connect(database_url: &str) -> anyhow::Result<Self> {
        let pool: sqlx::Pool<sqlx::Postgres> = PgPoolOptions::new().connect(database_url).await?;

        Ok(Self(pool))
    }

    pub(crate) fn inner_ref(&self) -> &PgPool {
        &self.0
    }
}

impl AsRef<PgPool> for PostgresDb {
    fn as_ref(&self) -> &PgPool {
        self.inner_ref()
    }
}
