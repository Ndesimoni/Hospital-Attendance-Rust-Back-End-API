use axum::{
    Json,
    extract::{Path, State},
    http::StatusCode,
};
use sqlx::PgPool;
use std::sync::Arc;

use crate::{
    models::{CreatePatient, Patient, UpdatePatient},
    repositories::{
        self, patient_repository::PatientRepository,
        postgres_patient_repository::PostgresPatientRepository,
    },
    services::patient_service::{
        self,
        PatientService,
        // create_patient_service,
        // get_all_patients_service,
        // get_patients_by_id_service, update_patient_details_service,
    },
    state::AppState,
};

////////////////////////////////////////////////////////
///

//*get all patients */
pub async fn get_all_patients(
    State(state): State<AppState>,
) -> Result<Json<Vec<Patient>>, StatusCode> {
    let patients = state
        .patient_service
        .get_all_patients_service()
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    Ok(Json(patients))
}

//*get patients by id*/
pub async fn get_patients_by_id(
    State(state): State<AppState>,
    Path(id): Path<i32>,
) -> Result<Json<Patient>, StatusCode> {
    println!("✅ get_patient_by_id handler called");

    let patient = state
        .patient_service
        .get_patients_by_id_service(id)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    match patient {
        Some(patient) => Ok(Json(patient)),

        None => Err(StatusCode::NOT_FOUND),
    }
}

//* create patients */
pub async fn create_patients(
    State(state): State<AppState>,
    Json(payload): Json<CreatePatient>,
) -> Result<Json<Patient>, StatusCode> {
    println!("hitting the created patient handler");

    let patient = state
        .patient_service
        .create_patients_service(payload)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    Ok(Json(patient))
}

//*update patients details*/
pub async fn update_patients_detail(
    State(state): State<AppState>,
    Path(id): Path<i32>,
    Json(payload): Json<UpdatePatient>,
) -> Result<Json<Patient>, StatusCode> {
    println!("api hitting the update route handler");

    let patient = state
        .patient_service
        .update_patient_details_service(id, payload)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    match patient {
        Some(p) => Ok(Json(p)),
        None => Err(StatusCode::NOT_MODIFIED),
    }
}
