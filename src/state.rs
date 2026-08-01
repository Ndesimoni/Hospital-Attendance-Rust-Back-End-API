use std::sync::Arc;

use crate::services::{
    auth_service::AuthService, patient_opt_service::PatientOtpService,
    patient_service::PatientService, user_role_service::UserRoleServices,
    visit_service::VisitService,
};

#[derive(Clone)]
pub struct AppState {
    pub patient_service: Arc<PatientService>,
    pub visit_service: Arc<VisitService>,
    pub auth_service: Arc<AuthService>,
    pub jwt_secret: String,
    pub role_service: Arc<UserRoleServices>,
    pub otp_service: Arc<PatientOtpService>,
}
