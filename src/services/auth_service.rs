use std::sync::Arc;

use bcrypt::{DEFAULT_COST, hash, verify};
use serde::{Deserialize, Serialize};

use crate::{
    middleware::jwt::create_token,
    models::{CreateUser, LoginRequest, LoginResponse, RegisterRequest, Users},
    repositories::auth_repository::AuthRepository,
};

pub struct AuthService {
    auth_repository: Arc<dyn AuthRepository>,
}

impl AuthService {
    pub fn new(auth_repository: Arc<dyn AuthRepository>) -> AuthService {
        AuthService { auth_repository }
    }

    //*registering/creating the user */
    pub async fn register(&self, payload: RegisterRequest) -> Result<Users, String> {
        let existing_user = self
            .auth_repository
            .find_user_by_email_trait(&payload.email)
            .await
            .map_err(|_| String::from("Database error"))?;

        if existing_user.is_some() {
            return Err("Email already exists".to_string());
        };

        let password_hash =
            hash(payload.password, DEFAULT_COST).map_err(|_| "Password hashing failed")?;

        let user_payload = CreateUser {
            email: payload.email,
            password_hash,
        };

        let user = self
            .auth_repository
            .create_user_trait(user_payload)
            .await
            .map_err(|_| "Database error")?;

        Ok(user)
    }

    //* user login */
    pub async fn login(&self, payload: LoginRequest) -> Result<LoginResponse, String> {
        let user = self
            .auth_repository
            .find_user_by_email_trait(&payload.email)
            .await
            .map_err(|_| String::from("Database Error"))?;

        let user = match user {
            Some(u) => u,
            None => return Err("Invalid email or password".to_string()),
        };

        let valid_password = verify(payload.password, &user.password_hash)
            .map_err(|_| "Password verification failed".to_string())?;

        if !valid_password {
            return Err("Invalid email or password".to_string());
        };

        let secret = std::env::var("JWT_SECRET").expect("JWT_SECRET must be set");

        let create_token = create_token(user.id, user.email.clone(), &secret)
            .map_err(|_| "Token creation failed".to_string())?;

        Ok(LoginResponse {
            token: create_token,
        })
    }
}
