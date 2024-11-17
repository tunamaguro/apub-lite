use std::str::FromStr;

use rsa::{
    pkcs1::{DecodeRsaPrivateKey, DecodeRsaPublicKey, EncodeRsaPrivateKey, EncodeRsaPublicKey},
    pkcs1v15::{Signature, SigningKey, VerifyingKey},
    pkcs8::{DecodePrivateKey, DecodePublicKey, EncodePrivateKey, EncodePublicKey},
    sha2::Sha256,
    signature::{Keypair, RandomizedSigner, Verifier},
    RsaPrivateKey, RsaPublicKey,
};

pub struct RsaVerifyingKey {
    verifying_key: VerifyingKey<Sha256>,
}

impl RsaVerifyingKey {
    pub fn verify(&self, msg: &[u8], signature: &[u8]) -> anyhow::Result<()> {
        let signature = Signature::try_from(signature)?;
        self.verifying_key.verify(msg, &signature)?;

        Ok(())
    }

    pub fn from_pem(pem: &str) -> anyhow::Result<Self> {
        RsaVerifyingKey::from_pkcs1(pem).or_else(|_| RsaVerifyingKey::from_pkcs8(pem))
    }

    pub fn from_pkcs1(pem: &str) -> anyhow::Result<Self> {
        let public_key = RsaPublicKey::from_pkcs1_pem(pem)?;
        let verifying_key = VerifyingKey::<Sha256>::new(public_key);
        Ok(Self { verifying_key })
    }

    pub fn from_pkcs8(pem: &str) -> anyhow::Result<Self> {
        let public_key = RsaPublicKey::from_public_key_pem(pem)?;
        let verifying_key = VerifyingKey::<Sha256>::new(public_key);
        Ok(Self { verifying_key })
    }

    pub fn to_pkcs1(&self) -> anyhow::Result<String> {
        self.verifying_key
            .to_pkcs1_pem(rsa::pkcs8::LineEnding::LF)
            .map_err(|e| e.into())
    }

    pub fn to_pkcs8(&self) -> anyhow::Result<String> {
        self.verifying_key
            .to_public_key_pem(rsa::pkcs8::LineEnding::LF)
            .map_err(|e| e.into())
    }
}

impl FromStr for RsaVerifyingKey {
    type Err = anyhow::Error;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        RsaVerifyingKey::from_pem(s)
    }
}

impl std::fmt::Display for RsaVerifyingKey {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            self.to_pkcs1().unwrap_or("Encoding failed".to_string())
        )
    }
}

pub struct RsaSingingKey {
    signing_key: SigningKey<Sha256>,
}

impl RsaSingingKey {
    pub fn new() -> anyhow::Result<Self> {
        const KEY_BITS: usize = 4096;
        let mut rng = rand::thread_rng();
        let private_key = RsaPrivateKey::new(&mut rng, KEY_BITS)?;
        let signing_key = SigningKey::<Sha256>::new(private_key);
        Ok(Self { signing_key })
    }

    pub fn sign(&self, msg: &[u8]) -> Box<[u8]> {
        let mut rng = rand::thread_rng();
        let signature = self.signing_key.sign_with_rng(&mut rng, msg);
        signature.into()
    }

    pub fn from_pem(pem: &str) -> anyhow::Result<Self> {
        RsaSingingKey::from_pkcs1(pem).or_else(|_| RsaSingingKey::from_pkcs8(pem))
    }

    pub fn from_pkcs1(pem: &str) -> anyhow::Result<Self> {
        let private_key = RsaPrivateKey::from_pkcs1_pem(pem)?;
        let signing_key = SigningKey::<Sha256>::new(private_key);
        Ok(Self { signing_key })
    }

    pub fn from_pkcs8(pem: &str) -> anyhow::Result<Self> {
        let private_key = RsaPrivateKey::from_pkcs8_pem(pem)?;
        let signing_key = SigningKey::<Sha256>::new(private_key);
        Ok(Self { signing_key })
    }

    pub fn to_pkcs1(&self) -> anyhow::Result<String> {
        let s: &RsaPrivateKey = self.signing_key.as_ref();
        s.to_pkcs1_pem(rsa::pkcs8::LineEnding::LF)
            .map_err(|e| e.into())
            .map(|pem| pem.to_string())
    }

    pub fn to_pkcs8(&self) -> anyhow::Result<String> {
        let s: &RsaPrivateKey = self.signing_key.as_ref();
        s.to_pkcs8_pem(rsa::pkcs8::LineEnding::LF)
            .map_err(|e| e.into())
            .map(|pem| pem.to_string())
    }

    pub fn to_public_key(&self) -> RsaVerifyingKey {
        let verifying_key = self.signing_key.verifying_key();
        RsaVerifyingKey { verifying_key }
    }
}

impl FromStr for RsaSingingKey {
    type Err = anyhow::Error;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Self::from_pem(s)
    }
}
