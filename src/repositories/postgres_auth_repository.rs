use async_trait::async_trait;
use sqlx::PgPool;

use crate::{
    models::{CreateUser, Users},
    repositories::auth_repository::AuthRepository,
};

pub struct PostgresAuthRepository {
    pool: PgPool,
}

impl PostgresAuthRepository {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }
}

#[async_trait]
impl AuthRepository for PostgresAuthRepository {
    async fn find_user_by_email_trait(&self, email: &str) -> Result<Option<Users>, sqlx::Error> {
        let user = sqlx::query_as!(
            Users,
            r#"
            SELECT
                id,
                email,
                password_hash,
                created_at
            FROM users
            WHERE email = $1
            "#,
            email
        )
        .fetch_optional(&self.pool)
        .await?;

        Ok(user)
    }

    async fn create_user_trait(&self, payload: CreateUser) -> Result<Users, sqlx::Error> {
        let user = sqlx::query_as!(
            Users,
            r#"
           INSERT INTO users(
               email,
               password_hash
           )

           VALUES ($1, $2)

           RETURNING 
             id,
             email,
             password_hash,
             created_at
        "#,
            payload.email,
            payload.password_hash
        )
        .fetch_one(&self.pool)
        .await?;

        Ok(user)
    }
}
