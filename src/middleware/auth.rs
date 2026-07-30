use axum::{
    extract::{Request, State},
    http::StatusCode,
    middleware::Next,
    response::Response,
};

use crate::{middleware::jwt::verify_token, models::AppError, state::AppState};

//*using the token in the middleware */
pub async fn auth_middleware(
    State(state): State<AppState>,
    mut request: Request,
    next: Next,
) -> Result<Response, AppError> {
    let auth_header = request.headers().get(axum::http::header::AUTHORIZATION);

    let auth_token = match auth_header {
        Some(header_value) => header_value,
        None => return Err(AppError::Unauthorized),
    };

    let auth_token = auth_token.to_str().map_err(|_| AppError::Unauthorized)?;

    let token = auth_token
        .strip_prefix("Bearer ")
        .ok_or(AppError::Unauthorized)?;

    let claims = verify_token(token, &state.jwt_secret).map_err(|_| AppError::Unauthorized)?;

    request.extensions_mut().insert(claims);

    Ok(next.run(request).await)
}
