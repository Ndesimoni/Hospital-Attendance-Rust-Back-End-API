use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};

use crate::models::Roles;

#[derive(Debug, Serialize, Deserialize)]
pub struct Users {
    pub id: i32,
    pub email: String,

    #[serde(skip_serializing)]
    pub password_hash: String,
    pub role: Roles,
    pub created_at: NaiveDateTime,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CreateUser {
    pub email: String,
    pub password_hash: String,
}
