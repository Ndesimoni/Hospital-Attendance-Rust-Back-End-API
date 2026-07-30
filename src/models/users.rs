use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};
use validator::Validate;

use crate::{models::Roles, validation::validate_password};

#[derive(Debug, Serialize, Deserialize)]
pub struct Users {
    pub id: i32,
    pub email: String,

    #[serde(skip_serializing)]
    pub password_hash: String,
    pub role: Roles,
    pub created_at: NaiveDateTime,
}

#[derive(Debug, Serialize, Deserialize, Validate)]
pub struct CreateUser {
    #[validate(email)]
    pub email: String,

    #[validate(length(min = 8), custom(function = "validate_password"))]
    pub password_hash: String,
}
