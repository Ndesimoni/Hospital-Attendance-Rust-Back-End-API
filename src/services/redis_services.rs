use redis::{AsyncCommands, aio::MultiplexedConnection};

use crate::models::AppError;

pub struct RedisService {
    pub connection: MultiplexedConnection,
}

impl RedisService {
    pub fn new(connection: MultiplexedConnection) -> Self {
        Self { connection }
    }

    pub async fn health_check(&self) -> Result<String, AppError> {
        let mut conn = self.connection.clone();

        let _: () = conn
            .set("hospital:test", "redis is working")
            .await
            .map_err(|_| AppError::InternalServerError)?;

        let value: String = conn
            .get("hospital:test")
            .await
            .map_err(|_| AppError::InternalServerError)?;

        Ok(value)
    }

    pub async fn store_otp(&self, patient_id: i32, otp: &str) -> Result<(), AppError> {
        let mut conn = self.connection.clone();

        let key = format!("otp:{}", patient_id);

        conn.set_ex::<_, _, ()>(key, otp, 300)
            .await
            .map_err(|_| AppError::InternalServerError)?;

        Ok(())
    }

    pub async fn get_otp(&self, patient_id: i32) -> Result<Option<String>, AppError> {
        let mut conn = self.connection.clone();

        let key = format!("otp:{}", patient_id);

        let otp: Option<String> = conn
            .get(key)
            .await
            .map_err(|_| AppError::InternalServerError)?;

        Ok(otp)
    }

    pub async fn delete_otp(&self, patient_id: i32) -> Result<(), AppError> {
        let mut conn = self.connection.clone();

        let key = format!("otp:{}", patient_id);

        conn.del::<_, ()>(key)
            .await
            .map_err(|_| AppError::InternalServerError)?;

        Ok(())
    }
}
