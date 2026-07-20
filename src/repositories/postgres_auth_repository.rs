use sqlx::PgPool;

use crate::repositories::auth_repository::AuthRepository;

pub struct PostgresAuthRepository {
    pool: PgPool,
}

impl PostgresAuthRepository {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }
}

impl AuthRepository for PostgresAuthRepository {}
