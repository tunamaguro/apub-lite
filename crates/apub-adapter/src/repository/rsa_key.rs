use apub_kernel::rsa_key::{
    model::{RsaSingingKey, RsaVerifyingKey, SaveKeyPairEvent, SavePublicKeyEvent},
    repository::RsaKeyRepository,
};
use apub_kernel::user::model::UserId;

use crate::model::rsa_key::{UserPrivateRsaKeyRow, UserPublicRsaKeyRow};
use crate::persistence::postgres::PostgresDb;

#[async_trait::async_trait]
impl RsaKeyRepository for PostgresDb {
    #[tracing::instrument(skip(self))]
    async fn find_public_key(&self, user_id: &UserId) -> anyhow::Result<RsaVerifyingKey> {
        let row = sqlx::query_as!(
            UserPublicRsaKeyRow,
            r#"
            SELECT 
                actor_rsa_keys.public_key AS public_key 
            FROM 
                actors 
            LEFT JOIN 
                actor_rsa_keys
            ON
                actors.actor_id = actor_rsa_keys.actor_id
            WHERE 
                actors.local_user_id = $1
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
             SELECT 
                actor_rsa_keys.private_key AS private_key 
            FROM 
                actors 
            LEFT JOIN 
                actor_rsa_keys
            ON
                actors.actor_id = actor_rsa_keys.actor_id
            WHERE 
                actors.local_user_id = $1
            "#,
            user_id.as_ref()
        )
        .fetch_one(self.inner_ref())
        .await?;

        row.try_into()
    }
    #[tracing::instrument(skip(self))]
    async fn save_public_key(&self, event: SavePublicKeyEvent<'_>) -> anyhow::Result<()> {
        let actor_id = event.actor_id.as_ref();
        let key_url = event.key_url.as_str();
        let public_key = event.public_key.to_pkcs8()?;
        sqlx::query!(
            r#"
             INSERT INTO actor_rsa_keys
                (actor_id, key_url, public_key)
            VALUES
                ($1, $2, $3)
            "#,
            actor_id,
            key_url,
            &public_key
        )
        .execute(self.inner_ref())
        .await?;

        Ok(())
    }
    #[tracing::instrument(skip(self))]
    async fn save_key_pair(&self, event: SaveKeyPairEvent<'_>) -> anyhow::Result<()> {
        let actor_id = event.actor_id.as_ref();
        let key_url = event.key_url.as_str();
        let public_key = event.public_key.to_pkcs8()?;
        let private_key = event.private_key.to_pkcs8()?;
        sqlx::query!(
            r#"
             INSERT INTO actor_rsa_keys
                (actor_id, key_url, public_key, private_key)
            VALUES
                ($1, $2, $3, $4)
            "#,
            actor_id,
            key_url,
            &public_key,
            &private_key
        )
        .execute(self.inner_ref())
        .await?;

        Ok(())
    }
}
