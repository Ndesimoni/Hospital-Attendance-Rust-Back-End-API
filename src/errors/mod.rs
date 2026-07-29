use std::{eprint, eprintln};

use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
};

use serde_json::json;

pub enum AppError {
    NotFound,
    Unauthorized,
    Forbidden,
    BadRequest(String),
    Conflict(String),
    InternalServerError,
}

impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        let (status, message) = match self {
            AppError::NotFound => (StatusCode::NOT_FOUND, "Resource not found".to_string()),

            AppError::Unauthorized => (StatusCode::UNAUTHORIZED, "user not Authorize".to_string()),

            AppError::Forbidden => (StatusCode::FORBIDDEN, "forbidden".to_string()),

            AppError::BadRequest(message) => (StatusCode::BAD_REQUEST, message),

            AppError::Conflict(message) => (StatusCode::CONFLICT, message),

            AppError::InternalServerError => (
                StatusCode::INTERNAL_SERVER_ERROR,
                "Internal server error".to_string(),
            ),
        };

        let body = json!({
            "error":message
        });

        (status, axum::Json(body)).into_response()
    }
}

impl From<sqlx::Error> for AppError {
    fn from(error: sqlx::Error) -> Self {
        match error {
            sqlx::Error::RowNotFound => AppError::NotFound,

            sqlx::Error::Database(data_error) => {
                if data_error.is_unique_violation() {
                    AppError::Conflict(String::from("Resource already exists"))
                } else {
                    eprintln!("internal error: {:?}", data_error);
                    AppError::InternalServerError
                }
            }

            error => {
                eprintln!("Database error: {:?}", error);
                AppError::InternalServerError
            }
        }
    }
}

impl From<bcrypt::BcryptError> for AppError {
    fn from(error: bcrypt::BcryptError) -> Self {
        eprintln!("Bcrypt error {:?}", error);
        AppError::InternalServerError
    }
}

impl From<jsonwebtoken::errors::Error> for AppError {
    fn from(error: jsonwebtoken::errors::Error) -> Self {
        eprintln!("Jwt error {:?}", error);
        AppError::InternalServerError
    }
}
