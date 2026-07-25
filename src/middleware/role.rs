use axum::{extract::Request, http::StatusCode, middleware::Next, response::Response};

use crate::models::{Claims, Roles};


pub async fn require_doctor(request: Request, next: Next) -> Result<Response, StatusCode> {
    let claim = request
        .extensions()
        .get::<Claims>()
        .ok_or(StatusCode::UNAUTHORIZED)?;

    if claim.role != Roles::Doctor {
        return Err(StatusCode::FORBIDDEN);
    };

    Ok(next.run(request).await)
}

pub async fn require_receptionist(request: Request, next: Next) -> Result<Response, StatusCode> {
    let claim = request
        .extensions()
        .get::<Claims>()
        .ok_or(StatusCode::UNAUTHORIZED)?;

    if claim.role != Roles::Receptionist {
        return Err(StatusCode::UNAUTHORIZED);
    }

    Ok(next.run(request).await)
}

pub async fn require_patient(request: Request, next: Next) -> Result<Response, StatusCode> {
    let claim = request
        .extensions()
        .get::<Claims>()
        .ok_or(StatusCode::UNAUTHORIZED)?;

    if claim.role != Roles::Patient {
        return Err(StatusCode::UNAUTHORIZED);
    }

    Ok(next.run(request).await)
}

pub async fn require_admin(request: Request, next: Next) -> Result<Response, StatusCode> {
    let claim = request
        .extensions()
        .get::<Claims>()
        .ok_or(StatusCode::UNAUTHORIZED)?;

    println!("ADMIN CLAIM: {:?}", claim);

    if claim.role != Roles::Admin {
        return Err(StatusCode::FORBIDDEN);
    }

    Ok(next.run(request).await)
}
