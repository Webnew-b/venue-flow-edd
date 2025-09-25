use std::sync::Arc;

use r2d2::Pool;
use r2d2_redis::RedisConnectionManager;
use tokio::sync::Mutex;

use super::redis::config::get_redis_url;
use crate::infra_error::InfraError;

pub mod config;

pub fn create_redis_connection(
) -> Result<Arc<Mutex<Pool<RedisConnectionManager>>>, InfraError> {
    let url = get_redis_url()?;

    let manager = RedisConnectionManager::new(url).expect("Invaild Redis URL");

    let pool = Pool::builder()
        .max_size(20)
        .build(manager)
        .expect("Fail to load the connection pool");

    Ok(Arc::new(Mutex::new(pool)))
}
