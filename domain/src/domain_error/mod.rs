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

    #[error("Infra error:{message}")]
    InfraError {
        message: String,
        source: Box<dyn std::error::Error + Send + Sync>,
    },

    #[error("Event error:{message}")]
    EventError {
        message: String,
        source: Box<dyn std::error::Error + Send + Sync>,
    },

    #[error("{0} is not found")]
    DataIsNotFound(String),

    #[error("The {0} id should be existed.")]
    IdInexisted(String),

    #[error("The {entity_type} is illegal,cause:{cause}")]
    EntityInvalid { entity_type: String, cause: String },
    #[error("Could not create {entity_type} entity,cause:{message}")]
    CreateEntityFailed {
        entity_type: String,
        message: String,
        #[source]
        source: domain_core::domain_core_error::DomainCoreError,
    },
    #[error("Could not update {entity_type} entity,cause:{message}")]
    UpdateEntityFailed {
        entity_type: String,
        message: String,
        #[source]
        source: domain_core::domain_core_error::DomainCoreError,
    },
}
