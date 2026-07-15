// use axum::{
//     Json,
//     extract::{Path, State},
//     http::StatusCode,
// };

// use crate::{
//     models::{CreateVisit, Visit},
//     services::{
//         diagnosis_services::diagnosis_services,
//         visit_service::{create_visit_service, get_all_visits, patient_visit_service},
//     },
// };

// pub async fn create_visit(
//     State(db): State<DB>,
//     Path(patient_id): Path<u32>,
//     Json(payload): Json<CreateVisit>,
// ) -> Result<Json<Visit>, StatusCode> {
//     let visit = create_visit_service(db, patient_id, payload)?;

//     Ok(Json(visit))
// }

// pub async fn visits(State(db): State<DB>) -> Json<Vec<Visit>> {
//     let visits = get_all_visits(db);
//     Json(visits)
// }

// pub async fn patients_visit(
//     State(db): State<DB>,
//     Path(patient_id): Path<u32>,
// ) -> Result<Json<Vec<Visit>>, StatusCode> {
//     let visit = patient_visit_service(db, patient_id)?;

//     Ok(Json(visit))
// }
