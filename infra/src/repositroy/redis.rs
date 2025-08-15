use std::io::{Error, ErrorKind};
use std::sync::Arc;

use r2d2::Pool;
use r2d2_redis::RedisConnectionManager;
use tokio::sync::Mutex;
use tracing::error;

use crate::config::config::get_redis_url;

pub fn create_redis_connection(
) -> Result<Arc<Mutex<Pool<RedisConnectionManager>>>, Error> {
    let url = get_redis_url().map_err(|e| {
        error!("{}", e.to_string());
        Error::new(
            ErrorKind::InvalidData,
            "Redis configurtion seems to have an Illegal field.",
        )
    })?;

    let manager = RedisConnectionManager::new(url).expect("Invaild Redis URL");

    let pool = Pool::builder()
        .max_size(20)
        .build(manager)
        .expect("Fail to load the connection pool");

    Ok(Arc::new(Mutex::new(pool)))
}
