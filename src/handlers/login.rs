use axum::{Json, extract::State, http::StatusCode, response};

use crate::{
    models::{LoginRequest, LoginResponse},
    state::{self, AppState},
};

pub async fn login(
    State(state): State<AppState>,
    Json(payload): Json<LoginRequest>,
) -> Result<Json<LoginResponse>, StatusCode> {
    let response = state
        .auth_service
        .login(payload)
        .await
        .map_err(|_| StatusCode::UNAUTHORIZED)?;

    Ok(Json(response))
}
