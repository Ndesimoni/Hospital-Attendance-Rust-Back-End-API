use std::sync::Arc;

use crate::services::{
    auth_service::AuthService, patient_service::PatientService, visit_service::VisitService,
};

#[derive(Clone)]
pub struct AppState {
    pub patient_service: Arc<PatientService>,
    pub visit_service: Arc<VisitService>,
    pub auth_service: Arc<AuthService>,
}
