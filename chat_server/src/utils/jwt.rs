use jwt_simple::prelude::*;
use std::ops::Deref;

use crate::{AppError, User};

#[warn(dead_code)]
const JWT_DURATION: u64 = 60 * 60 * 24 * 7;
const JWT_ISS: &str = "chat_server";
const JWT_AUD: &str = "chat_web";

pub struct EncodingKey(Ed25519KeyPair);
pub struct DecodingKey(Ed25519PublicKey);

impl EncodingKey {
    #[allow(unused)]
    pub fn load(key: &str) -> Result<Self, AppError> {
        Ok(Self(Ed25519KeyPair::from_pem(key)?))
    }

    #[allow(dead_code)]
    pub fn sign(&self, user: impl Into<User>) -> Result<String, AppError> {
        let claims = Claims::with_custom_claims(user.into(), Duration::from_secs(JWT_DURATION));
        let claims = claims.with_issuer(JWT_ISS).with_audience(JWT_AUD);
        Ok(self.0.sign(claims)?)
    }
}

impl DecodingKey {
    #[allow(unused)]
    pub fn load(key: &str) -> Result<Self, AppError> {
        Ok(Self(Ed25519PublicKey::from_pem(key)?))
    }

    #[allow(unused)]
    pub fn verify(&self, token: &str) -> Result<User, AppError> {
        let opts = VerificationOptions {
            allowed_issuers: Some(HashSet::from_strings(&[JWT_ISS])),
            allowed_audiences: Some(HashSet::from_strings(&[JWT_AUD])),
            ..Default::default()
        };
        let claims = self.0.verify_token::<User>(token, Some(opts))?;
        Ok(claims.custom)
    }
}

impl Deref for EncodingKey {
    type Target = Ed25519KeyPair;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl Deref for DecodingKey {
    type Target = Ed25519PublicKey;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use anyhow::Result;

    #[tokio::test]
    async fn jwt_sign_verify_should_work() -> Result<()> {
        let encoding_pem = include_str!("../../fixtures/encoding.pem");
        let decoding_pem = include_str!("../../fixtures/decoding.pem");
        let ek = EncodingKey::load(encoding_pem)?;
        let dk = DecodingKey::load(decoding_pem)?;

        let user = User::new(1, "MichaelScofield", "tchen@acme.org");

        let token = ek.sign(user.clone())?;
        let user2 = dk.verify(&token)?;

        assert_eq!(user, user2);
        Ok(())
    }
}
