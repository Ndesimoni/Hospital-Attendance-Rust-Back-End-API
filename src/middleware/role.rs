use axum::{extract::Request, middleware::Next, response::Response};

use crate::models::{AppError, Claims, Roles};

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
