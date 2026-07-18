use std::sync::Arc;

use crate::services::{patient_service::PatientService, visit_service::VisitService};

#[derive(Clone)]
pub struct AppState {
    pub patient_service: Arc<PatientService>,
    pub visit_service: Arc<VisitService>,
}
