use thiserror::Error;

pub type EventResult<T> = std::result::Result<T, EventError>;

#[derive(Debug, Error)]
pub enum EventError {
    #[error("Wrong execution mode for event")]
    WrongExecutionMode,

    #[error("Event serialization failed: {0}")]
    SerializationError(#[from] serde_json::Error),

    #[error("Queue operation failed: {source}")]
    QueueError {
        #[source]
        source: anyhow::Error,
    },

    #[error("Event execution failed: {event_type}, reason: {source}")]
    ExecutionError {
        event_type: String,
        #[source]
        source:     anyhow::Error,
    },

    #[error("Event handler not found for type: {event_type}")]
    HandlerNotFound { event_type: String },

    #[error("Event processing timeout: {event_type}")]
    Timeout { event_type: String },

    #[error("Event retry limit exceeded: {event_type}, attempts: {attempts}")]
    RetryLimitExceeded { event_type: String, attempts: u32 },

    #[error("Infrastructure error: {0}")]
    InfrastructureError(#[from] anyhow::Error),
}

impl EventError {
    pub fn queue<E: Into<anyhow::Error>>(err: E) -> Self {
        Self::QueueError { source: err.into() }
    }

    pub fn execution<E: Into<anyhow::Error>>(
        event_type: impl Into<String>,
        err: E,
    ) -> Self {
        Self::ExecutionError {
            event_type: event_type.into(),
            source:     err.into(),
        }
    }
}
