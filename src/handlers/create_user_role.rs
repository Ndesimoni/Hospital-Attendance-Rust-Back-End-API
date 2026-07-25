use axum::{Json, extract::State, http::StatusCode};

use crate::{
    models::{CreateUserRole, Users},
    state::AppState,
};

pub async fn create_user(
    State(state): State<AppState>,
    Json(payload): Json<CreateUserRole>,
) -> Result<Json<Users>, StatusCode> {
    let user = state
        .role_service
        .create_user_role_service(payload)
        .await
        .map_err(|_| StatusCode::BAD_REQUEST)?;

    Ok(Json(user))
}
