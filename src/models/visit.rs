use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct Visit {
    pub id: u32,
    pub patient_id: u32,
    pub symptoms: Vec<String>,
    pub diagnosis: String,
    pub medication: Vec<String>,
}

#[derive(Debug, Deserialize, Clone)]
pub struct CreateVisit {
    pub symptoms: Vec<String>,
}
