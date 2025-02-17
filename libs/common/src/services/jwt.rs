use jsonwebtoken::{errors::Error, Algorithm, DecodingKey, EncodingKey, Header};
use serde::{Deserialize, Serialize};

use crate::constants::JWT_SECRET;

#[derive(Debug, Serialize, Deserialize)]
struct JwtPayload {
    pub id: String,
    pub exp: i64,
}

pub fn build_secret(key: Option<String>) -> Vec<u8> {
    key.unwrap_or(JWT_SECRET.to_string()).as_bytes().to_vec()
}

pub fn sign(payload: JwtPayload, key: Option<String>) -> Result<String, Error> {
    jsonwebtoken::encode(
        &Header::new(Algorithm::RS512),
        &payload,
        &EncodingKey::from_secret(&build_secret(key)),
    )
}

pub fn verify(token: &String, key: Option<String>) -> Result<JwtPayload, Error> {
    jsonwebtoken::decode::<JwtPayload>(
        &token,
        &DecodingKey::from_secret(&build_secret(key)),
        &jsonwebtoken::Validation::new(Algorithm::RS512),
    )
    .map(|data| data.claims)
}
