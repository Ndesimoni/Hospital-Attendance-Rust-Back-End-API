use std::ffi::CStr;

use axum::{
    extract::{Request, rejection::JsonSyntaxError},
    http::StatusCode,
    middleware::Next,
    response::Response,
};
use chrono::{Duration, Utc};
use jsonwebtoken::{DecodingKey, EncodingKey, Header, Validation, decode, encode};

use serde::{Deserialize, Serialize};

use crate::models::{Claims, Roles};

//////////////////////////////////////

//*create the jwt token */
pub fn create_token(
    user_id: i32,
    email: String,
    role: Roles,
    secret: &str,
) -> Result<String, jsonwebtoken::errors::Error> {
    let expiration = Utc::now()
        .checked_add_signed(Duration::hours(1))
        .unwrap()
        .timestamp() as usize;

    let claims = Claims {
        sub: user_id,
        email: email,
        role,
        exp: expiration,
    };

    let token = encode(
        &Header::default(),
        &claims,
        &EncodingKey::from_secret(secret.as_bytes()),
    )?;

    Ok(token)
}

//*verifying the the jwt token that was created*/
pub fn verify_token(token: &str, secret: &str) -> Result<Claims, jsonwebtoken::errors::Error> {
    let token_data = decode::<Claims>(
        token,
        &DecodingKey::from_secret(secret.as_bytes()),
        &Validation::default(),
    )?;

    Ok(token_data.claims)
}
