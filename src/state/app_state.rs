use serde::{Deserialize, Serialize};
use std::sync::{Arc, Mutex};

use crate::models::{Patient, Visit};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AppState {
    pub patients: Vec<Patient>,
    pub visits: Vec<Visit>,
}

pub type DB = Arc<Mutex<AppState>>;
