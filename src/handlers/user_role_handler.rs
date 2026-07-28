use axum::{
    Json,
    extract::{Path, State},
    http::StatusCode,
};

use crate::{
    models::{CreateUserRole, UpdateUserPasswordBeforeHash, Users},
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

pub async fn update_user(
    State(state): State<AppState>,
    Path(id): Path<i32>,
    Json(payload): Json<UpdateUserPasswordBeforeHash>,
) -> Result<Json<Users>, StatusCode> {
    // println!("this is the update route");

    let update_user_password = state
        .role_service
        .update_user_password_service(id, payload)
        .await
        .map_err(|_| StatusCode::NOT_FOUND)?;

    Ok(Json(update_user_password))
}

pub async fn get_all_user(State(state): State<AppState>) -> Result<Json<Vec<Users>>, StatusCode> {
    let users = state
        .role_service
        .get_all_user_role_services()
        .await
        .map_err(|_| StatusCode::NO_CONTENT)?;

    Ok(Json(users))
}
