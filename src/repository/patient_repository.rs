use axum::http::StatusCode;

use crate::models::{CreatePatient, Patient, UpdatePatient};
use async_trait::async_trait;

///////////////////////////////////////////////

#[async_trait]
pub trait PatientRepository: Send + Sync {
    async fn get_all_patients_trait(&self) -> Result<Vec<Patient>, sqlx::Error>;

    async fn get_patients_by_id_trait(&self, id: i32) -> Result<Option<Patient>, sqlx::Error>;

    async fn create_patients_trait(&self, payload: CreatePatient) -> Result<Patient, sqlx::Error>;

    async fn update_patients_detail_trait(
        &self,
        id: i32,
        payload: UpdatePatient,
    ) -> Result<Option<Patient>, sqlx::Error>;
}
