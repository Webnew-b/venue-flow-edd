

use thiserror::Error;

use crate::domain_error::api_error::ApiError;
use crate::domain_error::database_error::DatabaseError;
use crate::domain_error::domain_user_error::DomainUserError;

pub mod database_error;
pub mod api_error;
pub mod domain_user_error;

#[derive(Error,Debug)]
pub enum DomainError {
    #[error(transparent)]
    DomainUserError(#[from] DomainUserError),
    #[error(transparent)]
    DatabaseError(#[from] DatabaseError),
    #[error(transparent)]
    ApiError(#[from] ApiError),
}


