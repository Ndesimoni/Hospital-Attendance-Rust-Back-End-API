use axum::{extract::State, response::IntoResponse};

use crate::state::AppState;

pub async fn redis_health(State(state): State<AppState>) -> impl IntoResponse {
    let value = state.redis_service.health_check().await.unwrap();

    value
}
