use redis::AsyncCommands;

use crate::models::AppError;

use super::RedisService;

impl RedisService {
    pub async fn check_rate_limit(
        &self,
        key: &str,
        limit: i32,
        seconds: i64,
    ) -> Result<bool, AppError> {
        let mut conn = self.connection.clone();

        let count: i32 = conn
            .incr(key, 1)
            .await
            .map_err(|_| AppError::InternalServerError)?;

        if count == 1 {
            let _: bool = conn
                .expire(key, seconds)
                .await
                .map_err(|_| AppError::InternalServerError)?;
        }

        Ok(count <= limit)
    }
}
