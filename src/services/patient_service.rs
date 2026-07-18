use std::sync::Arc;

use axum::{extract::State, http::StatusCode};
use sqlx::PgPool;

use crate::{
    models::{CreatePatient, Patient, UpdatePatient, Visit},
    repository::{
        patient_repository::PatientRepository,
        postgres_patient_repository::PostgresPatientRepository,
    },
};
//////////////////////////////////////
///

pub struct PatientService {
    repository: Arc<dyn PatientRepository>,
}

impl PatientService {
    pub fn new(repository: Arc<dyn PatientRepository>) -> Self {
        Self { repository }
    }

    pub async fn get_all_patients_service(&self) -> Result<Vec<Patient>, sqlx::Error> {
        self.repository.get_all_patients_trait().await
    }

    pub async fn create_patients_service(
        &self,
        payload: CreatePatient,
    ) -> Result<Patient, sqlx::Error> {
        self.repository.create_patients_trait(payload).await
    }

    pub async fn get_patients_by_id_service(
        &self,
        id: i32,
    ) -> Result<Option<Patient>, sqlx::Error> {
        self.repository.get_patients_by_id_trait(id).await
    }

    pub async fn update_patient_details_service(
        &self,
        id: i32,
        payload: UpdatePatient,
    ) -> Result<Option<Patient>, sqlx::Error> {
     self.repository.update_patients_detail_trait(id, payload).await
    }
}
