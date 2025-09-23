use thiserror::Error;

use crate::domain_error::api_error::ApiError;
use crate::domain_error::database_error::DatabaseError;
use crate::domain_error::domain_rental_error::DomainRentalError;
use crate::domain_error::domain_user_error::DomainUserError;
use crate::domain_error::domain_venue_error::DomainVenueError;
use crate::domain_error::infra_error::InfraError;

pub mod api_error;
pub mod database_error;
pub mod domain_rental_error;
pub mod domain_user_error;
pub mod domain_venue_error;
pub mod infra_error;

#[derive(Error, Debug)]
pub enum DomainError {
    #[error(transparent)]
    DomainUserError(#[from] DomainUserError),
    #[error(transparent)]
    DatabaseError(#[from] DatabaseError),
    #[error(transparent)]
    ApiError(#[from] ApiError),
    #[error(transparent)]
    DomaianRentalError(#[from] DomainRentalError),
    #[error(transparent)]
    DomainVenueError(#[from] DomainVenueError),
    #[error(transparent)]
    InfraError(#[from] InfraError),
}
