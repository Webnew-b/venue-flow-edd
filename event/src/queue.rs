use app::app_event::AppEvent;
use async_trait::async_trait;
use chrono::{DateTime, Utc};
use uuid::Uuid;

use crate::event_error::EventResult;

#[derive(Debug, Clone)]
pub struct QueueEvent {
    pub id: Uuid,
    pub event: AppEvent,
    pub created_at: DateTime<Utc>,
    pub attempts: u32,
}

impl QueueEvent {
    pub fn new(event: AppEvent) -> Self {
        Self {
            id: Uuid::new_v4(),
            event,
            created_at: Utc::now(),
            attempts: 0,
        }
    }
}

#[async_trait]
pub trait AsyncQueue: Send + Sync {
    async fn push(&self, event: QueueEvent) -> EventResult<()>;
    async fn pop_batch(&self, size: usize) -> EventResult<Vec<QueueEvent>>;
    async fn mark_processed(&self, event_id: Uuid) -> EventResult<()>;
    async fn mark_failed(
        &self,
        event_id: Uuid,
        error: String,
    ) -> EventResult<()>;
}
