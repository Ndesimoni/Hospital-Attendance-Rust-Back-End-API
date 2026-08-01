use axum::{Json, extract::State};

use crate::{
    models::{AppError, OtpResponse, PatientOtpRequest},
    state::AppState,
};

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
