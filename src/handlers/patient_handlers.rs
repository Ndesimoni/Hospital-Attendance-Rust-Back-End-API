use axum::{
    Json,
    extract::{Path, Query, State},
};

use validator::Validate;

use crate::{
    models::{
        AppError, CreatePatient, LoginResponse, OtpResponse, OtpVerification, Pagination, Patient,
        PatientOtpRequest, PatientResponse, UpdatePatient,
    },
    state::AppState,
};

////////////////////////////////////////////////////////

//*get all patients and invoke the pagination handler*/
pub async fn get_all_patients(
    State(state): State<AppState>,
    Query(pagination): Query<Pagination>,
) -> Result<Json<Vec<PatientResponse>>, AppError> {
    pagination.validation()?;

    let page = pagination.page();
    let limit = pagination.limit();

    let patients = state
        .patient_service
        .get_all_patients_service(page, limit)
        .await?;

    Ok(Json(patients))
}

//*get patients by id handler*/
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

//* create patients handler*/
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

//*update patients handler*/
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

//*patients login handler */
pub async fn patient_login(
    State(state): State<AppState>,
    Json(payload): Json<PatientOtpRequest>,
) -> Result<Json<OtpResponse>, AppError> {
    let otp_response = state
        .otp_service
        .request_patient_otp_service(payload)
        .await?;

    Ok(Json(otp_response))
}

//*patients opt handler */
pub async fn login_after_otp_verification(
    State(state): State<AppState>,
    Json(payload): Json<OtpVerification>,
) -> Result<Json<LoginResponse>, AppError> {
    let token = state.otp_service.verify_otp_service(payload).await?;

    Ok(Json(token))
}
