use deadpool_redis::{Config, Pool};
use thiserror::Error;

use super::redis::config::get_redis_url;
use crate::infra_error::InfraError;

pub mod config;

#[derive(Debug, Error)]
pub enum RedisError {
    #[error("Fail to create redis client.")]
    FailToCreateRedis,
}

pub fn create_redis_connection() -> Result<Pool, InfraError> {
    let url = get_redis_url()?;

    let cfg = Config::from_url(url);
    let pool = cfg
        .create_pool(Some(deadpool_redis::Runtime::Tokio1))
        .map_err(|e| {
            log::error!("{}", e);
            RedisError::FailToCreateRedis
        })?;

    Ok(pool)
}
