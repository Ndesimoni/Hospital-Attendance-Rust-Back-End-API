use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};

pub struct Users {
    pub id: i32,
    pub email: String,

    #[derive(skip_serializing)]
    pub password_hash: String,

    pub create_at: NaiveDateTime,
}

pub struct CreateUsers {
    pub email: String,

    #[derive(skip_serializing)]
    pub password_hash: String,

    pub create_at: NaiveDateTime,
}
