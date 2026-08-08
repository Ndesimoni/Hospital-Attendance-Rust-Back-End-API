// use crate::models::AppError;
// use redis::{Client, aio::MultiplexedConnection};

// pub async fn create_redis_connection(url: &str) -> Result<MultiplexedConnection, AppError> {
//     let client = Client::open(url).map_err(|_| AppError::InternalServerError)?;

//     let connection = client
//         .get_multiplexed_async_connection()
//         .await
//         .map_err(|_| AppError::InternalServerError)?;

//     Ok(connection)
// }

use crate::models::AppError;
use redis::{Client, aio::MultiplexedConnection};

pub async fn create_redis_connection(url: &str) -> Result<MultiplexedConnection, AppError> {
    let client = Client::open(url).map_err(|e| {
        println!("Redis client error: {}", e);
        AppError::InternalServerError
    })?;

    let connection = client
        .get_multiplexed_async_connection()
        .await
        .map_err(|e| {
            println!("Redis connection error: {}", e);
            AppError::InternalServerError
        })?;

    Ok(connection)
}
