use domain::domain_error::DomainError;
use thiserror::Error;

use crate::app_error::user_error::AppUserError;

pub mod user_error;
pub mod venue_error;

pub type AppResult<T> = std::result::Result<T,AppError>;

#[derive(Debug,Error)]
pub enum AppError {

    #[error(transparent)]
    DomainError(#[from] DomainError),

    #[error(transparent)]
    AppUserError(#[from] AppUserError),

    #[error("The {0} id should be existed.")]
    IdInexisted(String),
    #[error("Could not create {entity_type} entity,cause:{message}")]
    CreateEntityFailed {
        entity_type:String,
        message:String,
        #[source]
        source:domain_core::domain_core_error::DomainCoreError,
    },
    #[error("Could not update {entity_type} entity,cause:{message}")]
    UpdateEntityFailed {
        entity_type:String,
        message:String,
        #[source]
        source:domain_core::domain_core_error::DomainCoreError,
    },

    #[error("Other error:{0}")]
    Other(String)
}
