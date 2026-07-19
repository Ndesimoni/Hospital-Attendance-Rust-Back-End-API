// use axum::http::StatusCode;

// use crate::{
//     models::{CreateVisit, Patient, Visit},
//     services::diagnosis_services::diagnosis_services,
//     state::DB,
// };

// pub fn create_visit_service(
//     db: DB,
//     patient_id: u32,
//     payload: CreateVisit,
// ) -> Result<Visit, StatusCode> {
//     let mut state = db.lock().unwrap();

//     let patient_exist = state.patients.iter().any(|p| p.id == patient_id);

//     if !patient_exist {
//         return Err(StatusCode::NOT_FOUND);
//     }

//     let result = diagnosis_services(&payload.symptoms);

//     let visit = Visit {
//         id: state.visits.len() as u32 + 1,
//         patient_id,
//         symptoms: payload.symptoms.clone(),
//         diagnosis: result.diagnosis,
//         medication: result.medication,
//     };

//     state.visits.push(visit.clone());

//     Ok(visit)
// }

// pub fn get_all_visits(db: DB) -> Vec<Visit> {
//     let state = db.lock().unwrap();

//     state.visits.clone()
// }

// pub fn patient_visit_service(db: DB, patient_id: u32) -> Result<Vec<Visit>, StatusCode> {
//     let state = db.lock().unwrap();

//     if !state.patients.iter().any(|p| p.id == patient_id) {
//         return Err(StatusCode::NOT_FOUND);
//     }

//     let patients_visits = state
//         .visits
//         .iter()
//         .filter(|v| v.patient_id == patient_id)
//         .cloned()
//         .collect::<Vec<Visit>>();

//     Ok(patients_visits)
// }

use std::sync::Arc;

use crate::{
    models::{CreateVisit, NewVisit, Visit},
    repository::{self, patient_repository::PatientRepository, visit_repository::VisitRepository},
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
}
