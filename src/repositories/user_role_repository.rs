use crate::models::{UserRoleCreated, Users};
use async_trait::async_trait;
use axum::http::StatusCode;

#[async_trait]
pub trait UserRoleRepository: Send + Sync {
    async fn create_user_role_trait(&self, payload: UserRoleCreated) -> Result<Users, sqlx::Error>;
}
