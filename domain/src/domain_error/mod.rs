

use thiserror::Error;

use crate::domain_error::api_error::ApiError;
use crate::domain_error::database_error::DatabaseError;

pub mod database_error;
pub mod api_error;

#[derive(Error,Debug)]
pub enum DomainError {
    #[error(transparent)]
    DatabaseError(#[from] DatabaseError),
    #[error(transparent)]
    ApiError(#[from] ApiError),
}


