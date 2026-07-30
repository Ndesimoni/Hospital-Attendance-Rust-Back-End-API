use axum::{Json, extract::State, http::StatusCode};
use validator::Validate;

use crate::{
    errors::AppError,
    models::{RegisterRequest, Users, Visit},
    state::AppState,
};

pub async fn auth_handler(
    State(state): State<AppState>,
    Json(payload): Json<RegisterRequest>,
) -> Result<Json<Users>, AppError> {
    payload.validate()?;

    let user = state.auth_service.register(payload).await?;

    Ok(Json(user))
}
