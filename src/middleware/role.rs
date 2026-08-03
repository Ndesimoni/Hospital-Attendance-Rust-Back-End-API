use axum::{
    extract::{Request, State},
    http::header,
    middleware::Next,
    response::Response,
};

use crate::{
    middleware::jwt::verify_token,
    models::{AppError, Claims, Roles},
    state::AppState,
};

pub async fn require_doctor(request: Request, next: Next) -> Result<Response, AppError> {
    let claim = request
        .extensions()
        .get::<Claims>()
        .ok_or(AppError::Unauthorized)?;

    if claim.role != Roles::Doctor {
        return Err(AppError::Forbidden);
    }

    Ok(next.run(request).await)
}

pub async fn require_receptionist(request: Request, next: Next) -> Result<Response, AppError> {
    let claim = request
        .extensions()
        .get::<Claims>()
        .ok_or(AppError::Unauthorized)?;

    if claim.role != Roles::Receptionist {
        return Err(AppError::Forbidden);
    }

    Ok(next.run(request).await)
}

pub async fn require_patient(request: Request, next: Next) -> Result<Response, AppError> {
    let claim = request
        .extensions()
        .get::<Claims>()
        .ok_or(AppError::Unauthorized)?;

    if claim.role != Roles::Patient {
        return Err(AppError::Forbidden);
    }

    Ok(next.run(request).await)
}

pub async fn require_admin(request: Request, next: Next) -> Result<Response, AppError> {
    let claim = request
        .extensions()
        .get::<Claims>()
        .ok_or(AppError::Unauthorized)?;

    if claim.role != Roles::Admin {
        return Err(AppError::Forbidden);
    }

    Ok(next.run(request).await)
}

pub async fn require_me_patient(
    State(state): State<AppState>,
    mut req: Request,
    next: Next,
) -> Result<Response, AppError> {
    let auth_header = req
        .headers()
        .get(header::AUTHORIZATION)
        .and_then(|h| h.to_str().ok())
        .ok_or(AppError::Unauthorized)?;

    let token = auth_header
        .strip_prefix("Bearer ")
        .ok_or(AppError::Unauthorized)?;

    let claims = verify_token(token, &state.jwt_secret)?;

    if claims.role != Roles::Patient {
        return Err(AppError::Forbidden);
    }

    req.extensions_mut().insert(claims);

    Ok(next.run(req).await)
}
