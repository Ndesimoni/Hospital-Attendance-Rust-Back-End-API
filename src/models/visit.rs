use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct Visit {
    pub id: i32,
    pub patient_id: i32,
    pub symptoms: Vec<String>,
    pub diagnosis: String,
    pub medication: Vec<String>,
}

#[derive(Debug, Deserialize, Clone)]
pub struct CreateVisit {
    pub symptoms: Vec<String>,
}

#[derive(Debug, Clone)]
pub struct NewVisit {
    pub patient_id: i32,
    pub symptoms: Vec<String>,
    pub diagnosis: String,
    pub medication: Vec<String>,
}
