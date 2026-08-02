use chrono::NaiveDateTime;
use sqlx::PgPool;

use crate::{
    models::{AppError, PatientOtps},
    repositories::patient_otp_repository::PatientOtpRepository,
};

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

    async fn find_by_patient_id_trait(&self, patient_id: i32) -> Result<PatientOtps, AppError> {
        let otp = sqlx::query_as!(
            PatientOtps,
            r#"
    SELECT
        id,
        patient_id,
        otp,
        expires_at,
        used,
        created_at
    FROM patients_otp
    WHERE patient_id = $1
    "#,
            patient_id
        )
        .fetch_one(&self.pool)
        .await?;

        Ok(otp)
    }

    // async fn delete_by_patient_id_trait(&self, patient_id: i32) -> Result<(), AppError> {
    //     sqlx::query!(
    //         r#"
    //     DELETE FROM patients_otp
    //     WHERE patient_id = $1
    //     "#,
    //         patient_id
    //     )
    //     .execute(&self.pool)
    //     .await?;

    //     Ok(())
    // }

    async fn delete_by_patient_id_trait(&self, patient_id: i32) -> Result<(), AppError> {
        sqlx::query!(
            r#"
        DELETE FROM patients_otp
        WHERE patient_id = $1
        "#,
            patient_id
        )
        .execute(&self.pool)
        .await?;

        Ok(())
    }
}
