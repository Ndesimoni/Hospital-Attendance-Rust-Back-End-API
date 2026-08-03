use crate::models::{UpdateUserPasswordAfterHash, UserRoleCreated, Users};
use async_trait::async_trait;

#[async_trait]
pub trait UserRoleRepository: Send + Sync {
    async fn get_all_users_trait(&self) -> Result<Vec<Users>, sqlx::Error>;
    async fn create_user_role_trait(&self, payload: UserRoleCreated) -> Result<Users, sqlx::Error>;
    async fn get_user_by_id_role_trait(&self, id: i32) -> Result<Option<Users>, sqlx::Error>;

    async fn update_user_password_trait(
        &self,
        id: i32,
        payload: UpdateUserPasswordAfterHash,
    ) -> Result<Users, sqlx::Error>;
}
