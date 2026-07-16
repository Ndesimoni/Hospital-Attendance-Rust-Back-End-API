use serde::{Deserialize, Serialize};

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
#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct CreatePatient {
    pub name: String,
    pub age: i32,
    pub gender: String,
    pub email: String,
    pub contact: String,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct UpdatePatient {
    pub email: String,
    pub contact: String,
}
