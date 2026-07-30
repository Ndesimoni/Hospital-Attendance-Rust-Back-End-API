use axum::http::StatusCode;

use crate::models::{AppError, CreatePatient, Patient, UpdatePatient};
use async_trait::async_trait;

///////////////////////////////////////////////

#[async_trait]
pub trait PatientRepository: Send + Sync {
    async fn get_all_patients_trait(
        &self,
        limit: u32,
        offset: u32,
    ) -> Result<Vec<Patient>, AppError>;

    async fn get_patients_by_id_trait(&self, id: i32) -> Result<Option<Patient>, AppError>;

    async fn create_patients_trait(&self, payload: CreatePatient) -> Result<Patient, AppError>;

    async fn update_patients_detail_trait(
        &self,
        id: i32,
        payload: UpdatePatient,
    ) -> Result<Option<Patient>, AppError>;
}
