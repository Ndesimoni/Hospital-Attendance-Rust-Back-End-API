use chrono::NaiveDateTime;

use crate::models::AppError;

#[async_trait::async_trait]
pub trait PatientOtpRepository: Send + Sync {
    async fn create_otp_trait(
        &self,
        patient_id: i32,
        otp: &str,
        expires_at: NaiveDateTime,
    ) -> Result<(), AppError>;
}
