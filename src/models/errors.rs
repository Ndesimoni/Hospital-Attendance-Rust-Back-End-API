use validator::ValidationErrors;

pub enum AppError {
    NotFound,
    Unauthorized,
    Forbidden,
    BadRequest(String),
    Conflict(String),
    Validation(ValidationErrors),
    InternalServerError,
}
