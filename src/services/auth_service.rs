use std::sync::Arc;

use bcrypt::{DEFAULT_COST, hash};
use serde::{Deserialize, Serialize};

use crate::{
    models::{CreateUser, RegisterRequest, Users},
    repositories::auth_repository::AuthRepository,
};

pub struct AuthService {
    auth_repository: Arc<dyn AuthRepository>,
}

impl AuthService {
    pub fn new(auth_repository: Arc<dyn AuthRepository>) -> AuthService {
        AuthService { auth_repository }
    }

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
}
