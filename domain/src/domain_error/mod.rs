use thiserror::Error;

use crate::domain_error::domain_rental_error::DomainRentalError;
use crate::domain_error::domain_user_error::DomainUserError;
use crate::domain_error::domain_venue_error::DomainVenueError;

pub mod domain_rental_error;
pub mod domain_user_error;
pub mod domain_venue_error;

#[derive(Error, Debug)]
pub enum DomainError {
    #[error(transparent)]
    DomainUserError(#[from] DomainUserError),
    #[error(transparent)]
    DomaianRentalError(#[from] DomainRentalError),
    #[error(transparent)]
    DomainVenueError(#[from] DomainVenueError),

    #[error(transparent)]
    InfraError(#[from] InfraError),

    #[error("Event error:{message}")]
    EventError {
        message: String,
        source:  Box<dyn std::error::Error + Send + Sync>,
    },

    #[error("{0} is not found")]
    DataIsNotFound(String),

    #[error("The {0} id should be existed.")]
    IdInexisted(String),

    #[error("The {entity_type} is illegal,cause:{cause}")]
    EntityInvalid {
        entity_type: String,
        cause:       String,
    },
    #[error("Could not create {entity_type} entity,cause:{message}")]
    CreateEntityFailed {
        entity_type: String,
        message:     String,
        #[source]
        source:      domain_core::domain_core_error::DomainCoreError,
    },
    #[error("Could not update {entity_type} entity,cause:{message}")]
    UpdateEntityFailed {
        entity_type: String,
        message:     String,
        #[source]
        source:      domain_core::domain_core_error::DomainCoreError,
    },

    #[error("Fail to hash the password:{password}")]
    HashPasswordFail { password: String },

    #[error("Fail to verify the password")]
    VerifyPasswordFail,
}

#[derive(Debug, Error)]
pub enum InfraError {
    #[error("Data select fail,Cause:{message}.")]
    DataSelectFail {
        message: String,
        source:  Box<dyn std::error::Error + Sync + Send>,
    },
    #[error("Data save fail,Cause:{message}.")]
    DataSaveFail {
        message: String,
        source:  Box<dyn std::error::Error + Sync + Send>,
    },

    #[error("System is block,cause:{message}.")]
    SystemError { message: String },

    #[error("{message}")]
    MiddlewareError {
        message: String,
        source:  Box<dyn std::error::Error + Send + Sync>,
    },

    #[error("Access denied.")]
    AccessDenied,

    #[error("File Format is invalid.")]
    FileFormatInvalid,

    #[error("iThe server has occur an unexpected error.")]
    ServerError,
}
