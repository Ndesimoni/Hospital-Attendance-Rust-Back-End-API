use std::sync::Arc;

use axum::{http::StatusCode, middleware::Next};
use bcrypt::{DEFAULT_COST, hash, verify};

use crate::{
    models::{
        CreateUserRole, UpdateUserPasswordAfterHash, UpdateUserPasswordBeforeHash, UserRoleCreated,
        Users,
    },
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

    //* checking user email and password hashing */
    pub async fn update_user_password_service(
        &self,
        id: i32,
        payload: UpdateUserPasswordBeforeHash,
    ) -> Result<Users, String> {
        // 1. Check if user exists
        self.user_role_repository
            .get_user_by_id_role_trait(id)
            .await
            .map_err(|_| "Database error".to_string())?;

        // 2. Hash the NEW password
        let password_hash = hash(payload.password, DEFAULT_COST)
            .map_err(|_| "Password hashing failed".to_string())?;

        // 3. Create the payload that the repository expects
        let updated_user = UpdateUserPasswordAfterHash { password_hash };

        // 4. Call the repository trait and pass the values
        let updated_user = self
            .user_role_repository
            .update_user_password_trait(id, updated_user)
            .await
            .map_err(|_| "Database error".to_string())?;

        // 5. Return the updated user
        Ok(updated_user)
    }

    pub async fn get_all_user_role_services(&self) -> Result<Vec<Users>, sqlx::Error> {
        let users = self.user_role_repository.get_all_users_trait().await?;

        Ok(users)
    }
}
