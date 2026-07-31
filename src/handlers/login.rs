use axum::{Json, extract::State, http::StatusCode, response};

use validator::Validate;

use crate::{
    models::{AppError, LoginRequest, LoginResponse},
    state::{self, AppState},
};

pub async fn login(
    State(state): State<AppState>,
    Json(payload): Json<LoginRequest>,
) -> Result<Json<LoginResponse>, AppError> {
    payload.validate()?;

    println!("this handler is working good");

    let response = state.auth_service.login(payload).await?;

    Ok(Json(response))
}


