use axum::{Json, extract::State};

use validator::Validate;

use crate::{
    models::{AppError, LoginRequest, LoginResponse},
    state::AppState,
};

pub async fn login(
    State(state): State<AppState>,
    Json(payload): Json<LoginRequest>,
) -> Result<Json<LoginResponse>, AppError> {
    payload.validate()?;

    let response = state.auth_service.login(payload).await?;

    Ok(Json(response))
}
