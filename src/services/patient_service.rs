use std::sync::Arc;

use crate::{
    models::{AppError, CreatePatient, Patient, PatientOtpRequest, PatientResponse, UpdatePatient},
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

    pub async fn get_all_patients_service(
        &self,
        page: u32,
        limit: u32,
    ) -> Result<Vec<PatientResponse>, AppError> {
        let offset = (page - 1) * limit;

        let patients = self
            .repository
            .get_all_patients_trait(limit, offset)
            .await?;

        Ok(patients)
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

    //*requesting the opt */
    pub async fn request_patient_otp_service(
        &self,
        payload: PatientOtpRequest,
    ) -> Result<Patient, AppError> {
        match (payload.email, payload.contact) {
            (Some(email), None) => {
                let patient = self.repository.get_patients_by_email_trait(&email).await?;

                patient.ok_or(AppError::NotFound)
            }

            (None, Some(contact)) => {
                let patient = self
                    .repository
                    .get_patients_by_contact_trait(&contact)
                    .await?;

                patient.ok_or(AppError::NotFound)
            }

            _ => Err(AppError::BadRequest(String::from(
                "Provide exactly one of email or contact number",
            ))),
        }
    }
}
