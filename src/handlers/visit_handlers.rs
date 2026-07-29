use axum::{
    Json,
    extract::{Path, State},
};

use crate::{
    errors::AppError,
    models::{CreateVisit, UpdateVisit, Visit},
    state::AppState,
};

pub async fn create_visit(
    State(state): State<AppState>,
    Path(patient_id): Path<i32>,
    Json(payload): Json<CreateVisit>,
) -> Result<Json<Visit>, AppError> {
    let visit = state
        .visit_service
        .create_visit_services(patient_id, payload)
        .await?;

    Ok(Json(visit))
}

pub async fn get_all_visits(State(state): State<AppState>) -> Result<Json<Vec<Visit>>, AppError> {
    let all_visits = state.visit_service.get_all_visit_service().await?;

    Ok(Json(all_visits))
}

pub async fn get_patient_visit(
    State(state): State<AppState>,
    Path(id): Path<i32>,
) -> Result<Json<Vec<Visit>>, AppError> {
    let visit = state.visit_service.get_patient_visit_service(id).await?;

    Ok(Json(visit))
}

pub async fn update_visit(
    State(state): State<AppState>,
    Path(id): Path<i32>,
    Json(payload): Json<UpdateVisit>,
) -> Result<Json<Visit>, AppError> {
    let visit = state
        .visit_service
        .update_visit_service(id, payload)
        .await?;

    Ok(Json(visit))
}
