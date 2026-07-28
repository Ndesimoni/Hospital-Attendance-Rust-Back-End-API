use std::fmt;

use serde::{Deserialize, Serialize};
use sqlx::Type;

#[derive(Debug, Serialize, Deserialize, PartialEq, Eq, Clone)]
pub enum Roles {
    Doctor,
    Receptionist,
    Patient,
    Admin,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CreateUserRole {
    pub email: String,
    pub password: String,
    pub role: Roles,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UserRoleCreated {
    pub email: String,
    pub password_hash: String,
    pub role: Roles,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UpdateUserPasswordBeforeHash {
    pub password: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UpdateUserPasswordAfterHash {
    pub password_hash: String,
}
//*converting from enum to string so we can store in postgressql */
impl fmt::Display for Roles {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Roles::Doctor => write!(f, "Doctor"),
            Roles::Receptionist => write!(f, "Receptionist"),
            Roles::Patient => write!(f, "Patient"),
            Roles::Admin => write!(f, "Admin"),
        }
    }
}

//* converting back from sqlx strings values to Roles enum
impl From<String> for Roles {
    fn from(role: String) -> Self {
        match role.as_str() {
            "Doctor" => Roles::Doctor,
            "Receptionist" => Roles::Receptionist,
            "Admin" => Roles::Admin,
            _ => Roles::Patient,
        }
    }
}
