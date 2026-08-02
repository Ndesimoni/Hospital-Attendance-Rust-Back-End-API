use chrono::NaiveDateTime;

use crate::models::{AppError, PatientOtps};

#[async_trait::async_trait]
pub trait PatientOtpRepository: Send + Sync {
    async fn create_otp_trait(
        &self,
        patient_id: i32,
        otp: &str,
        expires_at: NaiveDateTime,
    ) -> Result<(), AppError>;

    async fn find_by_patient_id_trait(&self, patient_id: i32) -> Result<PatientOtps, AppError>;

    async fn delete_by_patient_id_trait(&self, patient_id: i32) -> Result<(), AppError>;
}
