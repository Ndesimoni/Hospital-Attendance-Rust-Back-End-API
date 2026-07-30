use axum::{
    Json,
    extract::{Path, State},
};
use validator::Validate;

use crate::{
    models::{AppError, CreateUserRole, UpdateUserPasswordBeforeHash, Users},
    state::AppState,
};

pub async fn create_user(
    State(state): State<AppState>,
    Json(payload): Json<CreateUserRole>,
) -> Result<Json<Users>, AppError> {
    payload.validate()?;

    let user = state.role_service.create_user_role_service(payload).await?;

    Ok(Json(user))
}

pub async fn update_user(
    State(state): State<AppState>,
    Path(id): Path<i32>,
    Json(payload): Json<UpdateUserPasswordBeforeHash>,
) -> Result<Json<Users>, AppError> {
    payload.validate()?;

    let update_user_password = state
        .role_service
        .update_user_password_service(id, payload)
        .await?;

    Ok(Json(update_user_password))
}

pub async fn get_all_user(State(state): State<AppState>) -> Result<Json<Vec<Users>>, AppError> {
    let users = state.role_service.get_all_user_role_services().await?;

    Ok(Json(users))
}
