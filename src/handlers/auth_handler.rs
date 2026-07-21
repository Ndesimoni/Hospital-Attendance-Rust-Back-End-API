use axum::{Json, extract::State, http::StatusCode};

use crate::{
    models::{RegisterRequest, Users, Visit},
    state::AppState,
};

pub async fn auth_handler(
    State(state): State<AppState>,
    Json(payload): Json<RegisterRequest>,
) -> Result<Json<Users>, StatusCode> {
    let user = state
        .auth_service
        .register(payload)
        .await
        .map_err(|_| StatusCode::IM_USED)?;

    Ok(Json(user))
}
