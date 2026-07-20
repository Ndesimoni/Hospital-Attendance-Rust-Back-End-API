use std::sync::Arc;

use crate::{
    models::{CreateVisit, NewVisit, UpdateVisit, Visit},
    repositories::{self, patient_repository::PatientRepository, visit_repository::VisitRepository},
    services::diagnosis_services::diagnosis_services,
};

pub struct VisitService {
    visit_repository: Arc<dyn VisitRepository>,
    patient_repository: Arc<dyn PatientRepository>,
}

impl VisitService {
    pub fn new(
        visit_repository: Arc<dyn VisitRepository>,
        patient_repository: Arc<dyn PatientRepository>,
    ) -> Self {
        Self {
            visit_repository,
            patient_repository,
        }
    }

    //* create a visits */
    pub async fn create_visit_services(
        &self,
        patient_id: i32,
        payload: CreateVisit,
    ) -> Result<Visit, sqlx::Error> {
        //todo check it there is already a patient that exist

        let patient_exist = self
            .patient_repository
            .get_patients_by_id_trait(patient_id)
            .await?;

        match patient_exist {
            Some(patient) => patient,
            None => return Err(sqlx::Error::RowNotFound),
        };

        let diagnosis = diagnosis_services(&payload.symptoms);

        //todo we still have to convert this CreateVisit to Visit

        let payload = NewVisit {
            patient_id,
            symptoms: payload.symptoms,
            diagnosis: diagnosis.diagnosis,
            medication: diagnosis.medication,
        };

        self.visit_repository.create_visit_trait(payload).await
    }

    //* get all visits */
    pub async fn get_all_visit_service(&self) -> Result<Vec<Visit>, sqlx::Error> {
        self.visit_repository.get_all_visits_trait().await
    }

    //* get a patient visits */
    pub async fn get_patient_visit_service(&self, id: i32) -> Result<Vec<Visit>, sqlx::Error> {
        let patient = self
            .patient_repository
            .get_patients_by_id_trait(id)
            .await?
            .ok_or(sqlx::Error::RowNotFound);

        let visits = self.visit_repository.get_a_patient_visits_trait(id).await?;

        Ok(visits)
    }

    pub async fn update_visit_service(
        &self,
        id: i32,
        payload: UpdateVisit,
    ) -> Result<Option<Visit>, sqlx::Error> {
        //todo check if visit exist first before updating it

        let updated_visit = self
            .visit_repository
            .update_a_visit_trait(id, payload)
            .await?;

        Ok(updated_visit)
    }
}
