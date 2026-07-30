use axum::{
    Json,
    extract::{Path, State},
};

use validator::Validate;

use crate::{
    errors::AppError,
    models::{CreatePatient, Patient, UpdatePatient},
    state::AppState,
};

////////////////////////////////////////////////////////

//*get all patients */
pub async fn get_all_patients(
    State(state): State<AppState>,
) -> Result<Json<Vec<Patient>>, AppError> {
    let patients = state.patient_service.get_all_patients_service().await?;

    Ok(Json(patients))
}

//*get patients by id*/
pub async fn get_patients_by_id(
    State(state): State<AppState>,
    Path(id): Path<i32>,
) -> Result<Json<Patient>, AppError> {
    let patient = state.patient_service.get_patients_by_id_service(id).await?;

    match patient {
        Some(patient) => Ok(Json(patient)),
        None => Err(AppError::NotFound),
    }
}

//* create patients */
pub async fn create_patients(
    State(state): State<AppState>,
    Json(payload): Json<CreatePatient>,
) -> Result<Json<Patient>, AppError> {
    payload.validate()?;

    let patient = state
        .patient_service
        .create_patients_service(payload)
        .await?;

    Ok(Json(patient))
}

//*update patients details*/
pub async fn update_patients_detail(
    State(state): State<AppState>,
    Path(id): Path<i32>,
    Json(payload): Json<UpdatePatient>,
) -> Result<Json<Patient>, AppError> {
    payload.validate()?;

    let patient = state
        .patient_service
        .update_patient_details_service(id, payload)
        .await?;

    match patient {
        Some(p) => Ok(Json(p)),
        None => Err(AppError::NotFound),
    }
}
