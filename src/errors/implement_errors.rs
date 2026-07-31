use serde_json::json;

use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
};

use validator::ValidationErrors;

use crate::models::AppError;

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

            AppError::Validation(errors) => {
                let fields = errors
                    .field_errors()
                    .into_iter()
                    .map(|(field, errors)| {
                        let message = errors
                            .first()
                            .and_then(|error| error.message.as_ref())
                            .map(|message| message.to_string())
                            .unwrap_or_else(|| "Invalid value".to_string());

                        (field.to_string(), message)
                    })
                    .collect::<std::collections::HashMap<_, _>>();

                let body = json!({
                    "error": "Validation failed",
                    "fields": fields
                });

                return (StatusCode::BAD_REQUEST, axum::Json(body)).into_response();
            }
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

impl From<ValidationErrors> for AppError {
    fn from(error: ValidationErrors) -> Self {
        AppError::Validation(error)
    }
}
