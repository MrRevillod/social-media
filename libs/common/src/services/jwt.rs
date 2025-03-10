use chrono::Utc;
use jsonwebtoken::{
    errors::{Error as JwtError, ErrorKind as JwtErrorKind},
    DecodingKey, EncodingKey, Header,
};

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
    pub exp: i64,
}

impl Claims {
    pub fn new(user_id: String, session_id: String, exp: i64) -> Self {
        Self {
            user_id,
            session_id,
            exp,
        }
    }
}

#[derive(Debug)]
pub enum Secret {
    Key(String),
    Default,
}

pub fn build_secret(secret: Secret) -> Vec<u8> {
    match secret {
        Secret::Key(key) => key.as_bytes().to_vec(),
        Secret::Default => JWT_SECRET.as_bytes().to_vec(),
    }
}

/// ## Sign jsonwebtoken function
///
/// This function is used to sign the JWT token
/// if the key is not provided, it will use the default JWT_SECRET
/// Otherwise, it will use the provided key

pub fn sign(payload: Claims, secret: Secret) -> Result<String, JwtError> {
    jsonwebtoken::encode(
        &Header::default(),
        &payload,
        &EncodingKey::from_secret(&build_secret(secret)),
    )
}

/// ## Verify jsonwebtoken function
///
/// This function is used to verify the JWT token
/// if the key is not provided, it will use the default JWT_SECRET
///
/// ### Returns
///
/// It returns the JWT payload if the token is valid and not expired
/// Otherwise, it returns an error (JwtError)

pub fn verify(token: &String, secret: Secret) -> Result<Claims, JwtError> {
    let claims = jsonwebtoken::decode::<Claims>(
        &token,
        &DecodingKey::from_secret(&build_secret(secret)),
        &jsonwebtoken::Validation::default(),
    )
    .map(|data| data.claims)?;

    if claims.exp < Utc::now().timestamp() {
        return Err(JwtError::from(JwtErrorKind::ExpiredSignature));
    }

    Ok(claims)
}
