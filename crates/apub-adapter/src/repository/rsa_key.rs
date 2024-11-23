use apub_kernel::{
    model::{
        rsa_key::{RsaSingingKey, RsaVerifyingKey},
        user::UserId,
    },
    repository::rsa_key::RsaKeyRepository,
};

use crate::model::rsa_key::{UserPrivateRsaKeyRow, UserPublicRsaKeyRow};
use crate::persistence::postgres::PostgresDb;

#[async_trait::async_trait]
impl RsaKeyRepository for PostgresDb {
    #[tracing::instrument(skip(self))]
    async fn find_public_key(&self, user_id: &UserId) -> anyhow::Result<RsaVerifyingKey> {
        let row = sqlx::query_as!(
            UserPublicRsaKeyRow,
            r#"
            SELECT user_id, public_key  FROM user_rsa_keys WHERE user_rsa_keys.user_id = $1
        "#,
            user_id.as_ref()
        )
        .fetch_one(self.inner_ref())
        .await?;

        row.try_into()
    }
    #[tracing::instrument(skip(self))]
    async fn find_private_key(&self, user_id: &UserId) -> anyhow::Result<RsaSingingKey> {
        let row = sqlx::query_as!(
            UserPrivateRsaKeyRow,
            r#"
            SELECT user_id, private_key  FROM user_rsa_keys WHERE user_rsa_keys.user_id = $1
        "#,
            user_id.as_ref()
        )
        .fetch_one(self.inner_ref())
        .await?;

        row.try_into()
    }
    #[tracing::instrument(skip(self))]
    async fn generate(&self, user_id: &UserId) -> anyhow::Result<(RsaSingingKey, RsaVerifyingKey)> {
        let skey = RsaSingingKey::new()?;
        let pkey = skey.to_public_key();

        let skey_pkcs8 = skey.to_pkcs8()?;
        let pkey_pkcs8 = pkey.to_pkcs8()?;

        sqlx::query!(
            r#"
            INSERT INTO user_rsa_keys (user_id, private_key, public_key) 
            VALUES ($1, $2, $3)
        "#,
            user_id.as_ref(),
            skey_pkcs8,
            pkey_pkcs8
        )
        .execute(self.inner_ref())
        .await?;
        Ok((skey, pkey))
    }
}
