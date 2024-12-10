use crate::user::model::UserId;

use super::model::{RsaSingingKey, RsaVerifyingKey, SaveKeyPairEvent, SavePublicKeyEvent};

#[async_trait::async_trait]
pub trait RsaKeyRepository: Send + Sync {
    /// 公開鍵をDBから探す
    async fn find_public_key(&self, user_id: &UserId) -> anyhow::Result<RsaVerifyingKey>;
    /// 秘密鍵をDBから探す
    async fn find_private_key(&self, user_id: &UserId) -> anyhow::Result<RsaSingingKey>;
    /// 公開鍵をDBに保存する
    async fn save_public_key(&self, event: SavePublicKeyEvent<'_>) -> anyhow::Result<()>;
    /// ユーザのキーペアをDBに保存する
    async fn save_key_pair(&self, event: SaveKeyPairEvent<'_>) -> anyhow::Result<()>;
}
