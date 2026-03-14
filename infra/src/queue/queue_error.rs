use thiserror::Error;

#[derive(Error, Debug)]
pub enum QueueError {
    #[error("Failed to fetch queue configuration.")]
    FailToFetchConfig,
    #[error("Failed  to deserialize queue configuration.")]
    FailToDeserializeConfig,
}
