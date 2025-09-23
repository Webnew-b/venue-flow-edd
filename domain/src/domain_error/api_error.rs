use thiserror::Error;

#[derive(Debug, Error)]
pub enum ApiError {
    #[error("Other message:{0}")]
    Other(String),
}
