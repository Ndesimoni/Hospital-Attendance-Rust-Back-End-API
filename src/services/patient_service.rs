use std::sync::Arc;

use crate::{
    errors::AppError,
    models::{CreatePatient, Patient, UpdatePatient},
    repositories::patient_repository::PatientRepository,
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

    pub async fn get_all_patients_service(&self) -> Result<Vec<Patient>, AppError> {
        self.repository.get_all_patients_trait().await
    }

    pub async fn create_patients_service(
        &self,
        payload: CreatePatient,
    ) -> Result<Patient, AppError> {
        self.repository.create_patients_trait(payload).await
    }

    pub async fn get_patients_by_id_service(&self, id: i32) -> Result<Option<Patient>, AppError> {
        self.repository.get_patients_by_id_trait(id).await
    }

    pub async fn update_patient_details_service(
        &self,
        id: i32,
        payload: UpdatePatient,
    ) -> Result<Option<Patient>, AppError> {
        self.repository
            .update_patients_detail_trait(id, payload)
            .await
    }
}
