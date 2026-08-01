use chrono::NaiveDateTime;
use sqlx::PgPool;

use crate::{models::AppError, repositories::patient_otp_repository::PatientOtpRepository};

pub struct PostgresPatientOtpRepository {
    pool: PgPool,
}

impl PostgresPatientOtpRepository {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }
}

#[async_trait::async_trait]
impl PatientOtpRepository for PostgresPatientOtpRepository {
    async fn create_otp_trait(
        &self,
        patient_id: i32,
        otp: &str,
        expires_at: NaiveDateTime,
    ) -> Result<(), AppError> {
        sqlx::query!(
            r#"
            INSERT INTO patients_otp (
                patient_id,
                otp,
                expires_at
            )
            VALUES ($1, $2, $3)
            "#,
            patient_id,
            otp,
            expires_at
        )
        .execute(&self.pool)
        .await?;

        Ok(())
    }
}
