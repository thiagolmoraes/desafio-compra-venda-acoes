use jsonwebtoken::{DecodingKey, EncodingKey};
use anyhow::Result;

pub struct Keys {
    pub encoding: EncodingKey,
    pub decoding: DecodingKey,
}

impl Keys {
    pub fn new(secret: &[u8]) -> Result<Self> {
        Ok(
            Self {
                encoding: EncodingKey::from_secret(secret),
                decoding: DecodingKey::from_secret(secret),
            }
        )
    }
}