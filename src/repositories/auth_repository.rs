use crate::{
    errors::AppError,
    models::{CreateUser, RegisterRequest, Users},
};

#[async_trait::async_trait]
pub trait AuthRepository: Send + Sync {
    async fn create_user_trait(&self, payload: CreateUser) -> Result<Users, AppError>;
    async fn find_user_by_email_trait(&self, email: &str) -> Result<Option<Users>, AppError>;
}
