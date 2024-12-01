use crate::user::model::UserId;

use super::model::{RsaSingingKey, RsaVerifyingKey};

#[async_trait::async_trait]
pub trait RsaKeyRepository: Send + Sync {
    /// 公開鍵をDBから探す
    async fn find_public_key(&self, user_id: &UserId) -> anyhow::Result<RsaVerifyingKey>;
    /// 秘密鍵をDBから探す
    async fn find_private_key(&self, user_id: &UserId) -> anyhow::Result<RsaSingingKey>;
    /// `UserId`用の鍵を生成し保存する
    async fn generate(&self, user_id: &UserId) -> anyhow::Result<(RsaSingingKey, RsaVerifyingKey)>;

    /// 公開鍵をDBから探す。なければ生成する
    async fn find_public_key_or_generate(
        &self,
        user_id: &UserId,
    ) -> anyhow::Result<RsaVerifyingKey> {
        let maybe_find_key = self.find_public_key(user_id).await;
        match maybe_find_key {
            Ok(pkey) => Ok(pkey),
            Err(_) => {
                let (_, pkey) = self.generate(user_id).await?;
                Ok(pkey)
            }
        }
    }

    /// 秘密をDBから探す。なければ生成する
    async fn find_private_key_or_generate(
        &self,
        user_id: &UserId,
    ) -> anyhow::Result<RsaSingingKey> {
        let maybe_find_key = self.find_private_key(user_id).await;
        match maybe_find_key {
            Ok(skey) => Ok(skey),
            Err(_) => {
                let (skey, _) = self.generate(user_id).await?;
                Ok(skey)
            }
        }
    }
}
