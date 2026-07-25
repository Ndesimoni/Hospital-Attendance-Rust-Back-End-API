use serde::{Deserialize, Serialize};

use crate::models::Roles;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Claims {
    pub sub: i32,
    pub email: String,
    pub role: Roles,
    pub exp: usize,
}
