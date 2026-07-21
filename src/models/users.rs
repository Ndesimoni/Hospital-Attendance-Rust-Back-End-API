use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Users {
    pub id: i32,
    pub email: String,

    #[serde(skip_serializing)]
    pub password_hash: String,

    pub created_at: NaiveDateTime,
}
