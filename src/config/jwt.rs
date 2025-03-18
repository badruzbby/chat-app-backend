use anyhow::Result;
use chrono::{Duration, Utc};
use jsonwebtoken::{Algorithm, DecodingKey, EncodingKey, Header, Validation, decode, encode};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::config::get_jwt_expiration;
use crate::config::get_jwt_secret;

#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    pub sub: String,
    pub iat: i64,
    pub exp: i64,
    pub iss: String,
}

pub fn generate_token(user_id: Uuid) -> Result<String> {
    let now = Utc::now();
    let expiration = get_jwt_expiration();
    let claims = Claims {
        sub: user_id.to_string(),
        iat: now.timestamp(),
        exp: (now + Duration::seconds(expiration)).timestamp(),
        iss: "chat_app".to_string(),
    };

    let token = encode(
        &Header::default(),
        &claims,
        &EncodingKey::from_secret(get_jwt_secret().as_bytes()),
    )?;

    Ok(token)
}

pub fn validate_token(token: &str) -> Result<Claims> {
    let decoded = decode::<Claims>(
        token,
        &DecodingKey::from_secret(get_jwt_secret().as_bytes()),
        &Validation::new(Algorithm::HS256),
    )?;

    Ok(decoded.claims)
}
