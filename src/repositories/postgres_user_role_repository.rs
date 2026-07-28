use async_trait::async_trait;
use sqlx::PgPool;

use crate::{
    models::{CreateUserRole, UpdateUserPasswordAfterHash, UserRoleCreated, Users},
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
    //* create a user role trait repo
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

    //*get all users */
    async fn get_all_users_trait(&self) -> Result<Vec<Users>, sqlx::Error> {
        let users = sqlx::query_as!(
            Users,
            r#"
          SELECT
              id,
              email,
              password_hash,
              role,
              created_at

          FROM users
        "#
        )
        .fetch_all(&self.pool)
        .await?;

        Ok(users)
    }

    //* get user role by id trait repo
    async fn get_user_by_id_role_trait(&self, id: i32) -> Result<Option<Users>, sqlx::Error> {
        let user = sqlx::query_as!(
            Users,
            r#"
          SELECT
                id,
                email,
                password_hash,
                role,
                created_at
            FROM users
            WHERE id = $1
        "#,
            id
        )
        .fetch_optional(&self.pool)
        .await?;

        Ok(user)
    }

    //* update user role trait repo
    async fn update_user_password_trait(
        &self,
        id: i32,
        payload: UpdateUserPasswordAfterHash,
    ) -> Result<Users, sqlx::Error> {
        let user = sqlx::query_as!(
            Users,
            r#"
         
        UPDATE users
        SET
          
            password_hash = $1

        WHERE id = $2

        RETURNING
            id,
            email,
            password_hash,
            role,
            created_at
        "#,
            payload.password_hash,
            id
        )
        .fetch_one(&self.pool)
        .await?;

        Ok(user)
    }
}
