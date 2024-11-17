use apub_kernel::model::rsa_key::{RsaSingingKey, RsaVerifyingKey};
use sqlx::types::Uuid;

pub struct UserPublicRsaKeyRow {
    pub user_id: Uuid,
    pub public_key: String,
}

impl TryFrom<UserPublicRsaKeyRow> for RsaVerifyingKey {
    type Error = anyhow::Error;
    fn try_from(row: UserPublicRsaKeyRow) -> Result<Self, Self::Error> {
        RsaVerifyingKey::from_pem(&row.public_key)
    }
}

pub struct UserPrivateRsaKeyRow {
    pub user_id: Uuid,
    pub private_key: String,
}

impl TryFrom<UserPrivateRsaKeyRow> for RsaSingingKey {
    type Error = anyhow::Error;
    fn try_from(row: UserPrivateRsaKeyRow) -> Result<Self, Self::Error> {
        RsaSingingKey::from_pem(&row.private_key)
    }
}
