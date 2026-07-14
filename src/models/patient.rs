use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct Patient {
    pub id: u32,
    pub name: String,
    pub age: u8,
    pub gender: String,
}

//todo need to add the patient email and number fields later on
#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct CreatePatient {
    pub name: String,
    pub age: u8,
    pub gender: String,
}
