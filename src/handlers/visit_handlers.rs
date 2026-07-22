use std::sync::Arc;

use axum::{
    Json,
    extract::{Path, State},
    http::StatusCode,
};

use crate::{
    models::{CreateVisit, UpdateVisit, Visit},
    services::visit_service::VisitService,
    state::AppState,
};

pub async fn create_visit(
    State(state): State<AppState>,
    Path(patient_id): Path<i32>,
    Json(payload): Json<CreateVisit>,
) -> Result<Json<Visit>, StatusCode> {
    println!("request hitting the create visit handler");

    let visit = state
        .visit_service
        .create_visit_services(patient_id, payload)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    Ok(Json(visit))
}

pub async fn get_all_visits(State(state): State<AppState>) -> Result<Json<Vec<Visit>>, StatusCode> {
    let all_visits = state
        .visit_service
        .get_all_visit_service()
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    Ok(Json(all_visits))
}

pub async fn get_patient_visit(
    State(state): State<AppState>,
    Path(id): Path<i32>,
) -> Result<Json<Vec<Visit>>, StatusCode> {
    let visit = state
        .visit_service
        .get_patient_visit_service(id)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    Ok(Json(visit))
}

pub async fn update_visit(
    State(state): State<AppState>,
    Path(id): Path<i32>,
    Json(payload): Json<UpdateVisit>,
) -> Result<Json<Visit>, StatusCode> {
    let visit = state
        .visit_service
        .update_visit_service(id, payload)
        .await
        .map_err(|_| StatusCode::NOT_FOUND)?;

    match visit {
        Some(v) => Ok(Json(v)),
        None => return Err(StatusCode::NOT_FOUND),
    }
}
