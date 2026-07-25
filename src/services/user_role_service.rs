use std::sync::Arc;

use axum::middleware::Next;
use bcrypt::{DEFAULT_COST, hash, verify};

use crate::{
    models::{CreateUserRole, UserRoleCreated, Users},
    repositories::{
        auth_repository::AuthRepository,
        user_role_repository::{self, UserRoleRepository},
    },
};

#[derive(Clone)]
pub struct UserRoleServices {
    auth_repository: Arc<dyn AuthRepository>,
    user_role_repository: Arc<dyn UserRoleRepository>,
}

impl UserRoleServices {
    pub fn new(
        auth_repository: Arc<dyn AuthRepository>,
        user_role_repository: Arc<dyn UserRoleRepository>,
    ) -> Self {
        Self {
            auth_repository,
            user_role_repository,
        }
    }

    //* checking user email and password hashing */
    pub async fn create_user_role_service(&self, payload: CreateUserRole) -> Result<Users, String> {
        let user_exist = self
            .auth_repository
            .find_user_by_email_trait(&payload.email)
            .await
            .map_err(|_| String::from("DataBase Error"))?;

        if user_exist.is_some() {
            return Err("Email already Exist".to_string());
        };

        let password_hash = hash(payload.password, DEFAULT_COST)
            .map_err(|_| String::from("Password hashing failed"))?;

        let user_payload = UserRoleCreated {
            email: payload.email,
            password_hash,
            role: payload.role,
        };

        let user_role = self
            .user_role_repository
            .create_user_role_trait(user_payload)
            .await
            .map_err(|_| "Database error")?;

        Ok(user_role)
    }
}
