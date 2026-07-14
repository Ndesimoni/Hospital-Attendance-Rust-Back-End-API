use axum::{
    Json,
    extract::{Path, State},
    http::StatusCode,
};

use crate::{
    models::{CreatePatient, Patient},
    services::patient_service::{
        create_patient_service, delete_patient_service, find_patient_service,
        get_all_patients_service, update_patient_service,
    },
    state::DB,
};

pub async fn hello() -> &'static str {
    "this is the first handler working"
}

//todo creating a new patient
pub async fn create_patient(
    State(db): State<DB>,
    Json(payload): Json<CreatePatient>,
) -> Result<(StatusCode, Json<Patient>), StatusCode> {
    let patient = create_patient_service(db, payload)?;

    Ok((StatusCode::CREATED, Json(patient)))
}

//todo view all patient
pub async fn get_all_patients(State(db): State<DB>) -> Json<Vec<Patient>> {
    let state = get_all_patients_service(db);

    Json(state)
}

//todo fine patient by params id
pub async fn find_patient(
    State(db): State<DB>,
    Path(id): Path<u32>,
) -> Result<Json<Patient>, StatusCode> {
    let patient = find_patient_service(db, id)?;

    Ok(Json(patient))
}

//todo put request
pub async fn update_patient_details(
    State(db): State<DB>,
    Path(id): Path<u32>,
    Json(payload): Json<CreatePatient>,
) -> Result<Json<Patient>, StatusCode> {
    let patient = update_patient_service(db, id, payload)?;

    Ok(Json(patient))
}

// delete a patient
pub async fn delete_patient(State(db): State<DB>, Path(id): Path<u32>) -> StatusCode {
    delete_patient_service(db, id)
}
