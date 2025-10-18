use domain::domain_error::DomainError;
use thiserror::Error;

use crate::config::ConfigError;
use crate::database::DatabaseError;
use crate::queue::queue_error::QueueError;
use crate::repositroy::oss::OssError;
use crate::repositroy::redis::RedisError;

pub(crate) type InfraResult<T> = std::result::Result<T, InfraError>;

#[derive(Debug, Error)]
pub enum InfraError {
    #[error(transparent)]
    OssError(#[from] OssError),

    #[error(transparent)]
    RedisError(#[from] RedisError),

    #[error(transparent)]
    DatabaseError(#[from] DatabaseError),

    #[error(transparent)]
    ConfigError(#[from] ConfigError),

    #[error(transparent)]
    QueueError(#[from] QueueError),

    #[error("Fail to construct the log executor.")]
    FailToConstructLog,

    #[error("Fail to hash the password:{password}")]
    HashPasswordFail { password: String },

    #[error("Fail to verify the password")]
    VerifyPasswordFail,

    #[error("Fail to save image to repository.")]
    SaveImageFail,

    #[error("Fail to get file extension type.")]
    ObtainExtensionFail,

    #[error("File is not found.")]
    FileNotFound,

    #[error("File does not be read.")]
    FileNotRead,

    #[error("File type is invalid.")]
    FileTypeIsInvalid,

    #[error("Failed to initizalize event system:{message}")]
    FailToInitEventSystem { message: String },

    #[error("Failed to initizalize http server:{message}")]
    FailToInitHttpServer { message: String },
}

impl From<InfraError> for DomainError {
    fn from(value: InfraError) -> Self {
        DomainError::InfraError {
            message: value.to_string(),
            source: Box::new(value),
        }
    }
}
