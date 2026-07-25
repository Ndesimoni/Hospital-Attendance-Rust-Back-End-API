use async_trait::async_trait;
use sqlx::PgPool;

use crate::{
    models::{CreateUserRole, UserRoleCreated, Users},
    repositories::user_role_repository::UserRoleRepository,
};

pub struct PostgresUserRoleRepository {
    pool: PgPool,
}

impl PostgresUserRoleRepository {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }
}

#[async_trait]
impl UserRoleRepository for PostgresUserRoleRepository {
    async fn create_user_role_trait(&self, payload: UserRoleCreated) -> Result<Users, sqlx::Error> {
        let user_role = sqlx::query_as!(
            Users,
            r#"
            INSERT INTO users (
            email,
            password_hash,
            role
          )

            VALUES($1, $2, $3)
          
            RETURNING 
             id,
             email,
             password_hash,
             role,
             created_at
          "#,
            payload.email,
            payload.password_hash,
            payload.role.to_string()
        )
        .fetch_one(&self.pool)
        .await?;

        Ok(user_role)
    }
}
