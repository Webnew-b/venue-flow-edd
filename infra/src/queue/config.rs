use config::{Config, File};
use serde::{Deserialize, Serialize};

use crate::{infra_error::InfraError, queue::queue_error::QueueError};

pub(super) fn get_redis_queue_config() -> Result<RedisQueueConfig, InfraError> {
    let config: QueueConfig = Config::builder()
        .add_source(File::with_name("config/redis_queue.toml"))
        .build()
        .map_err(|e| {
            tracing::error!("{}", e);
            QueueError::FailToFetchConfig
        })?
        .try_deserialize()
        .map_err(|e| {
            tracing::error!("{}", e);
            QueueError::FailToDeserializeConfig
        })?;
    Ok(config.queue_config)
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct QueueConfig {
    pub queue_config: RedisQueueConfig,
}
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct RedisQueueConfig {
    pub queue_prefix:       String,
    pub processing_timeout: i64,
    pub failed_retention:   i64,
    pub max_pool_size:      usize,
}

impl Default for RedisQueueConfig {
    fn default() -> Self {
        Self {
            queue_prefix:       "event_queue".to_string(),
            processing_timeout: 300,
            failed_retention:   86400,
            max_pool_size:      16,
        }
    }
}
