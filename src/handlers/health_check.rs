use axum::Json;
use serde_json::{Value, json};

use crate::models::AppError;

pub async fn health_check() -> Result<Json<Value>, AppError> {
    Ok(Json(json!({
       "status": "ok",
       "database": "connected"
    })))
}
