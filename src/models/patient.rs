use serde::{Deserialize, Serialize};
use validator::Validate;

use crate::validation::validate_password;

#[derive(Debug, Deserialize, Serialize, Clone, sqlx::FromRow)]
pub struct Patient {
    pub id: i32,
    pub name: String,
    pub age: i32,
    pub gender: String,
    pub email: String,
    pub contact: String,
}

//todo need to add the patient email and number fields later on
#[derive(Debug, Deserialize, Serialize, Clone, Validate)]
pub struct CreatePatient {
    pub name: String,
    pub age: i32,
    pub gender: String,
    #[validate(email)]
    pub email: String,
    pub contact: String,
}

#[derive(Debug, Deserialize, Serialize, Clone, Validate)]
pub struct UpdatePatient {
    #[validate(email)]
    pub email: String,
    pub contact: String,
}

#[derive(Debug, Deserialize, Serialize, Clone, Validate)]
pub struct PatientsLogin {
    #[validate(email)]
    pub email: String,
    pub contact: String,
}
