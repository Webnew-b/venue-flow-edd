use domain::domain_error::DomainError;
use domain::domain_error::InfraError as DomainInfraError;
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

    #[error("Failed to decode the JWT:{message}")]
    FailToDecodeJWT { message: String },

    #[error("Access denied")]
    AccessDenied,
}

impl From<InfraError> for DomainError {
    fn from(value: InfraError) -> Self {
        match value {
            InfraError::OssError(e) => DomainInfraError::MiddlewareError {
                message: e.to_string(),
                source:  e.into(),
            },
            InfraError::RedisError(e) => DomainInfraError::MiddlewareError {
                message: e.to_string(),
                source:  e.into(),
            },
            InfraError::ConfigError(e) => DomainInfraError::MiddlewareError {
                message: e.to_string(),
                source:  e.into(),
            },
            InfraError::QueueError(e) => DomainInfraError::MiddlewareError {
                message: e.to_string(),
                source:  e.into(),
            },

            InfraError::DatabaseError(database_error) => match database_error {
                DatabaseError::ConnectionRefused { source } => {
                    DomainInfraError::SystemError {
                        message: source.to_string(),
                    }
                },
                DatabaseError::Other(s) => {
                    DomainInfraError::SystemError { message: s }
                },
                DatabaseError::DeleteEntityFail => todo!(),
                e => DomainInfraError::DataSelectFail {
                    message: e.to_string(),
                    source:  e.into(),
                },
            },

            InfraError::FailToConstructLog
            | InfraError::SaveImageFail
            | InfraError::FileNotRead => DomainInfraError::ServerError,

            InfraError::ObtainExtensionFail
            | InfraError::FileNotFound
            | InfraError::FileTypeIsInvalid => {
                DomainInfraError::FileFormatInvalid
            },

            InfraError::FailToInitEventSystem { message }
            | InfraError::FailToDecodeJWT { message }
            | InfraError::FailToInitHttpServer { message } => {
                DomainInfraError::SystemError { message }
            },

            InfraError::AccessDenied => DomainInfraError::AccessDenied,
        }
        .into()
    }
}
