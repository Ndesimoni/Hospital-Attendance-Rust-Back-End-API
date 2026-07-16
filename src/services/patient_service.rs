use axum::http::StatusCode;
use sqlx::PgPool;

use crate::{
    models::{CreatePatient, Patient, UpdatePatient, Visit},
    repository::patient_repository::PatientRepository,
};
//////////////////////////////////////

pub async fn get_all_patients_service(
    repository: &impl PatientRepository,
) -> Result<Vec<Patient>, sqlx::Error> {
    repository.get_all_patients_trait().await
}

pub async fn get_patients_by_id_service(
    repository: &impl PatientRepository,
    id: i32,
) -> Result<Option<Patient>, sqlx::Error> {
    repository.get_patients_by_id_trait(id).await
}

pub async fn create_patient_service(
    repository: &impl PatientRepository,
    payload: CreatePatient,
) -> Result<Patient, sqlx::Error> {
    repository.create_patients_trait(payload).await
}

pub async fn update_patient_details_service(
    repository: &impl PatientRepository,
    id: i32,
    payload: UpdatePatient,
) -> Result<Option<Patient>, sqlx::Error> {
    repository.update_patients_detail_trait(id, payload).await
}
