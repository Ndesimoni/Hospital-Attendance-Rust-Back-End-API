use axum::{Json, extract::State, http::StatusCode};
use sqlx::PgPool;

use crate::{
    models::{CreatePatient, Patient},
    services::patient_service::{create_patient_service, get_all_patients_service},
};

// //todo creating a new patient
pub async fn create_patient(
    State(pool): State<PgPool>,
    Json(payload): Json<CreatePatient>,
) -> Result<(StatusCode, Json<Patient>), StatusCode> {
    let patient = create_patient_service(&pool, payload).await.map_err(|e| {
        println!("Database error: {:?}", e);
        StatusCode::INTERNAL_SERVER_ERROR
    })?;

    Ok((StatusCode::CREATED, Json(patient)))
}

// //todo view all patient
pub async fn get_all_patients(
    State(pool): State<PgPool>,
) -> Result<Json<Vec<Patient>>, StatusCode> {
    let patients = get_all_patients_service(&pool)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    Ok(Json(patients))
}

// //todo fine patient by params id
// pub async fn find_patient(
//     State(db): State<DB>,
//     Path(id): Path<u32>,
// ) -> Result<Json<Patient>, StatusCode> {
//     let patient = find_patient_service(db, id)?;

//     Ok(Json(patient))
// }

// //todo put request
// pub async fn update_patient_details(
//     State(db): State<DB>,
//     Path(id): Path<u32>,
//     Json(payload): Json<CreatePatient>,
// ) -> Result<Json<Patient>, StatusCode> {
//     let patient = update_patient_service(db, id, payload)?;

//     Ok(Json(patient))
// }

// // delete a patient
// pub async fn delete_patient(State(db): State<DB>, Path(id): Path<u32>) -> StatusCode {
//     delete_patient_service(db, id)
// }
