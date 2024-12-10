use apub_kernel::rsa_key::model::{RsaSingingKey, RsaVerifyingKey};

pub struct UserPublicRsaKeyRow {
    pub public_key: String,
}

impl TryFrom<UserPublicRsaKeyRow> for RsaVerifyingKey {
    type Error = anyhow::Error;
    fn try_from(row: UserPublicRsaKeyRow) -> Result<Self, Self::Error> {
        RsaVerifyingKey::from_pem(&row.public_key)
    }
}

pub struct UserPrivateRsaKeyRow {
    pub private_key: Option<String>,
}

impl TryFrom<UserPrivateRsaKeyRow> for RsaSingingKey {
    type Error = anyhow::Error;
    fn try_from(row: UserPrivateRsaKeyRow) -> Result<Self, Self::Error> {
        match row.private_key {
            Some(ref k) => RsaSingingKey::from_pem(k),
            None => Err(anyhow::anyhow!("private key is not found")),
        }
    }
}
