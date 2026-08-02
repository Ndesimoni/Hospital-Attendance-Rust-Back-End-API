use serde::{Deserialize, Serialize};
use validator::Validate;

//////////////////////////////////////////////////
#[derive(Debug, Deserialize, Serialize, Clone, sqlx::FromRow)]
pub struct Patient {
    pub id: i32,
    pub name: String,
    pub age: i32,
    pub gender: String,
    pub email: String,
    pub contact: String,
}

#[derive(Debug, Serialize, Clone, sqlx::FromRow)]
pub struct PatientResponse {
    pub id: i32,
    pub name: String,
    pub age: i32,
    pub gender: String,
    pub email: String,
    pub contact: String,
    pub role: String,
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

//* this is for updating patient details struct */
#[derive(Debug, Deserialize, Serialize, Clone, Validate)]
pub struct UpdatePatient {
    #[validate(email)]
    pub email: String,
    pub contact: String,
}
