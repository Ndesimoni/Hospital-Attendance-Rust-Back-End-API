use std::sync::Arc;

use bcrypt::{DEFAULT_COST, hash, verify};
use redis::ConnectionInfo;

use crate::{
    middleware::jwt::create_token,
    models::{AppError, CreateUser, LoginRequest, LoginResponse, RegisterRequest, Users},
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
    pub async fn register(&self, payload: RegisterRequest) -> Result<Users, AppError> {
        let existing_user = self
            .auth_repository
            .find_user_by_email_trait(&payload.email)
            .await?;

        if existing_user.is_some() {
            return Err(AppError::Conflict("Email already exists".to_string()));
        };

        let password_hash = hash(payload.password, DEFAULT_COST)?;

        let user_payload = CreateUser {
            email: payload.email,
            password_hash,
        };

        let user = self.auth_repository.create_user_trait(user_payload).await?;

        Ok(user)
    }

    //* user login */
    pub async fn login(&self, payload: LoginRequest) -> Result<LoginResponse, AppError> {
        let user = self
            .auth_repository
            .find_user_by_email_trait(&payload.email)
            .await?;

        let user = match user {
            Some(u) => u,
            None => return Err(AppError::Unauthorized),
        };

        let valid_password = match verify(payload.password, &user.password_hash) {
            Ok(valid) => valid,
            Err(err) => {
                eprintln!("bcrypt verification failed: {err:?}");
                return Err(AppError::Unauthorized);
            }
        };

        if !valid_password {
            return Err(AppError::Unauthorized);
        };

        let secret = std::env::var("JWT_SECRET").expect("JWT_SECRET must be set");

        let token = create_token(user.id, user.email.clone(), user.role, &secret)?;

        Ok(LoginResponse { token })
    }
}
