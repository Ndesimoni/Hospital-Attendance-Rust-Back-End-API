use axum::extract::rejection::JsonSyntaxError;
use chrono::{Duration, Utc};
use jsonwebtoken::{EncodingKey, Header, encode};

use serde::{Deserialize, Serialize};

//////////////////////////////////////

#[derive(Serialize, Deserialize)]
pub struct Claims {
    pub sub: i32,
    pub email: String,
    pub exp: usize,
}

pub fn create_token(
    user_id: i32,
    email: String,
    secret: &str,
) -> Result<String, jsonwebtoken::errors::Error> {
    let expiration = Utc::now()
        .checked_add_signed(Duration::hours(1))
        .unwrap()
        .timestamp() as usize;

    let claims = Claims {
        sub: user_id,
        email: email,
        exp: expiration,
    };

    let token = encode(
        &Header::default(),
        &claims,
        &EncodingKey::from_secret(secret.as_bytes()),
    )?;

    Ok(token)
}
