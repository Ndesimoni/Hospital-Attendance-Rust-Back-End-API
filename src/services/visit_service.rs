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
    repository::{self, visit_repository::VisitRepository},
};

pub struct VisitService {
    repository: Arc<dyn VisitRepository>,
}

impl VisitService {
    pub fn new(repository: Arc<dyn VisitRepository>) -> Self {
        Self { repository }
    }

    pub async fn create_visit_services(
        &self,
        patient_id: i32,
        payload: CreateVisit,
    ) -> Result<Visit, sqlx::Error> {
        //todo check it there is already a patient that exist
        //todo we still have to convert this CreateVisit to Visit

        let payload = NewVisit {
            patient_id,
            symptoms: payload.symptoms,
            diagnosis: String::from("fever"),
            medication: vec!["panadol".to_string()],
        };

        self.repository.create_visit_trait(payload).await
    }
}
