use axum::{Json, extract::State, http::StatusCode, response};

use crate::{
    errors::AppError,
    models::{LoginRequest, LoginResponse},
    state::{self, AppState},
};

pub async fn login(
    State(state): State<AppState>,
    Json(payload): Json<LoginRequest>,
) -> Result<Json<LoginResponse>, AppError> {
    let response = state.auth_service.login(payload).await?;

    Ok(Json(response))
}
