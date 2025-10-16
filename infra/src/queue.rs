use async_trait::async_trait;
use chrono::{DateTime, Utc};
use deadpool_redis::{redis::AsyncCommands, Connection, Pool};
use event::{
    event_error::{EventError, EventResult},
    queue::{AsyncQueue, QueueEvent},
};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use app::app_event::AppEvent;

use crate::{
    infra_error::InfraError,
    queue::config::{get_redis_queue_config, RedisQueueConfig},
};

pub mod config;
pub mod queue_error;

#[derive(Debug, Clone, Serialize, Deserialize)]
struct StoredQueueEvent {
    pub id: Uuid,
    pub event: AppEvent,
    pub created_at: DateTime<Utc>,
    pub attempts: u32,
}

impl From<QueueEvent> for StoredQueueEvent {
    fn from(qe: QueueEvent) -> Self {
        Self {
            id: qe.id,
            event: qe.event,
            created_at: qe.created_at,
            attempts: qe.attempts,
        }
    }
}

impl From<StoredQueueEvent> for QueueEvent {
    fn from(se: StoredQueueEvent) -> Self {
        Self {
            id: se.id,
            event: se.event,
            created_at: se.created_at,
            attempts: se.attempts,
        }
    }
}

pub struct RedisAsyncQueue {
    pool: Pool,
    config: RedisQueueConfig,
}

impl RedisAsyncQueue {
    pub fn new(pool: Pool) -> Result<Self, InfraError> {
        let config = get_redis_queue_config()?;
        Ok(Self { pool, config })
    }

    async fn get_connection(&self) -> EventResult<Connection> {
        self.pool.get().await.map_err(|e| {
            EventError::queue(anyhow::anyhow!(
                "Failed to get connection: {}",
                e
            ))
        })
    }

    fn queue_key(&self) -> String {
        format!("{}:pending", self.config.queue_prefix)
    }

    fn processing_key(&self) -> String {
        format!("{}:processing", self.config.queue_prefix)
    }

    fn failed_key(&self) -> String {
        format!("{}:failed", self.config.queue_prefix)
    }

    fn event_key(&self, event_id: Uuid) -> String {
        format!("{}:event:{}", self.config.queue_prefix, event_id)
    }

    fn serialize_event(&self, event: &StoredQueueEvent) -> EventResult<String> {
        serde_json::to_string(event).map_err(EventError::from)
    }

    fn deserialize_event(&self, data: &str) -> EventResult<StoredQueueEvent> {
        serde_json::from_str(data).map_err(EventError::from)
    }

    pub fn pool_status(&self) -> deadpool_redis::Status {
        self.pool.status()
    }
}

#[async_trait]
impl AsyncQueue for RedisAsyncQueue {
    async fn push(&self, event: QueueEvent) -> EventResult<()> {
        let mut conn = self.get_connection().await?;
        let stored_event: StoredQueueEvent = event.into();
        let event_id = stored_event.id;

        let event_data = self.serialize_event(&stored_event)?;

        let queue_key = self.queue_key();
        let event_key = self.event_key(event_id);

        conn.set_ex::<_, _, ()>(&event_key, &event_data, 86400)
            .await
            .map_err(|e| {
                EventError::queue(anyhow::anyhow!("Failed to set event: {}", e))
            })?;

        conn.rpush::<_, _, ()>(&queue_key, event_id.to_string())
            .await
            .map_err(|e| {
                EventError::queue(anyhow::anyhow!(
                    "Failed to push to queue: {}",
                    e
                ))
            })?;

        tracing::debug!("Pushed event {} to queue", event_id);
        Ok(())
    }

    async fn pop_batch(&self, size: usize) -> EventResult<Vec<QueueEvent>> {
        let mut conn = self.get_connection().await?;
        let queue_key = self.queue_key();
        let processing_key = self.processing_key();

        let mut events = Vec::new();

        for _ in 0..size {
            let event_id: Option<String> = conn
                .rpoplpush(&queue_key, &processing_key)
                .await
                .map_err(|e| {
                    EventError::queue(anyhow::anyhow!(
                        "Failed to rpoplpush: {}",
                        e
                    ))
                })?;

            if let Some(id_str) = event_id {
                let event_id = Uuid::parse_str(&id_str).map_err(|e| {
                    EventError::queue(anyhow::anyhow!("Invalid UUID: {}", e))
                })?;

                let event_key = self.event_key(event_id);

                let event_data: Option<String> =
                    conn.get(&event_key).await.map_err(|e| {
                        EventError::queue(anyhow::anyhow!(
                            "Failed to get event: {}",
                            e
                        ))
                    })?;

                if let Some(data) = event_data {
                    let mut stored_event = self.deserialize_event(&data)?;
                    stored_event.attempts += 1;

                    let updated_data = self.serialize_event(&stored_event)?;
                    conn.set_ex::<_, _, ()>(&event_key, &updated_data, 86400)
                        .await
                        .map_err(|e| {
                            EventError::queue(anyhow::anyhow!(
                                "Failed to update event: {}",
                                e
                            ))
                        })?;

                    events.push(stored_event.into());
                } else {
                    conn.lrem::<_, _, ()>(&processing_key, 1, id_str)
                        .await
                        .map_err(|e| {
                            EventError::queue(anyhow::anyhow!(
                                "Failed to lrem: {}",
                                e
                            ))
                        })?;

                    tracing::warn!("Event {} data not found, removed from processing queue", event_id);
                }
            } else {
                break;
            }
        }

        Ok(events)
    }

    async fn mark_processed(&self, event_id: Uuid) -> EventResult<()> {
        let mut conn = self.get_connection().await?;
        let processing_key = self.processing_key();
        let event_key = self.event_key(event_id);

        conn.lrem::<_, _, ()>(&processing_key, 1, event_id.to_string())
            .await
            .map_err(|e| {
                EventError::queue(anyhow::anyhow!("Failed to lrem: {}", e))
            })?;

        conn.del::<_, ()>(&event_key).await.map_err(|e| {
            EventError::queue(anyhow::anyhow!("Failed to delete event: {}", e))
        })?;

        tracing::debug!("Marked event {} as processed", event_id);
        Ok(())
    }

    async fn mark_failed(
        &self,
        event_id: Uuid,
        error: String,
    ) -> EventResult<()> {
        let mut conn = self.get_connection().await?;
        let processing_key = self.processing_key();
        let failed_key = self.failed_key();
        let event_key = self.event_key(event_id);

        conn.lrem::<_, _, ()>(&processing_key, 1, event_id.to_string())
            .await
            .map_err(|e| {
                EventError::queue(anyhow::anyhow!("Failed to lrem: {}", e))
            })?;

        let event_data: Option<String> =
            conn.get(&event_key).await.map_err(|e| {
                EventError::queue(anyhow::anyhow!("Failed to get event: {}", e))
            })?;

        if let Some(data) = event_data {
            let failed_record = serde_json::json!({
                "event_id": event_id.to_string(),
                "event_data": data,
                "error": error,
                "failed_at": Utc::now().to_rfc3339(),
            });

            let failed_data = serde_json::to_string(&failed_record)
                .map_err(EventError::from)?;

            let score = Utc::now().timestamp() as f64;
            conn.zadd::<_, _, _, ()>(&failed_key, event_id.to_string(), score)
                .await
                .map_err(|e| {
                    EventError::queue(anyhow::anyhow!("Failed to zadd: {}", e))
                })?;

            let failed_detail_key =
                format!("{}:failed:{}", self.config.queue_prefix, event_id);
            conn.set_ex::<_, _, ()>(
                &failed_detail_key,
                &failed_data,
                self.config.failed_retention as u64,
            )
            .await
            .map_err(|e| {
                EventError::queue(anyhow::anyhow!(
                    "Failed to set failed detail: {}",
                    e
                ))
            })?;

            conn.del::<_, ()>(&event_key).await.map_err(|e| {
                EventError::queue(anyhow::anyhow!(
                    "Failed to delete event: {}",
                    e
                ))
            })?;
        }

        tracing::warn!("Marked event {} as failed: {}", event_id, error);
        Ok(())
    }
}
