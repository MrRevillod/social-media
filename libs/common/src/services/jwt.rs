use jsonwebtoken::{errors::Error as JwtError, DecodingKey, EncodingKey, Header};
use serde::{Deserialize, Serialize};

use crate::constants::JWT_SECRET;

/// ## JWT Payload struct
///
/// This struct is used to create the JWT token payload
///
/// ### Fields
/// uuid: session user id
/// session_id: session id for the user (shared between access and refresh token)
/// exp: token expiration time

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Claims {
    pub user_id: String,
    pub session_id: String,
    pub exp: u64,
}

impl Claims {
    pub fn new(user_id: String, session_id: String, exp: u64) -> Self {
        Self {
            user_id,
            session_id,
            exp,
        }
    }
}

pub fn build_secret(key: Option<String>) -> Vec<u8> {
    key.unwrap_or(JWT_SECRET.to_string()).as_bytes().to_vec()
}

/// ## Sign jsonwebtoken function
///
/// This function is used to sign the JWT token
/// if the key is not provided, it will use the default JWT_SECRET
/// Otherwise, it will use the provided key

pub fn sign(payload: Claims, key: Option<String>) -> Result<String, JwtError> {
    jsonwebtoken::encode(
        &Header::default(),
        &payload,
        &EncodingKey::from_secret(&build_secret(key)),
    )
}

/// ## Verify jsonwebtoken function
///
/// This function is used to verify the JWT token
/// if the key is not provided, it will use the default JWT_SECRET
///
/// ### Returns
///
/// It returns the JWT payload if the token is valid
/// Otherwise, it returns an error (JwtError)

pub fn verify(token: &String, key: Option<String>) -> Result<Claims, JwtError> {
    jsonwebtoken::decode::<Claims>(
        &token,
        &DecodingKey::from_secret(&build_secret(key)),
        &jsonwebtoken::Validation::default(),
    )
    .map(|data| data.claims)
}
