use crate::models::{CreateUsers, Users};

pub trait AuthRepository {
    pub async fn create_user_auth_trait(&self, payload: CreateUsers) -> Result<Users, sqlx::Error>;
    pub async fn find_user_by_email_trait(&self, email: &str)
    -> Result<Option<Users>, sqlx::Error>;
}
