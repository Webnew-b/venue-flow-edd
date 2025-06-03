use domain::domain_error::DomainError;
use thiserror::Error;

use crate::app_error::user_error::AppUserError;

pub mod user_error;

pub type AppResult<T> = std::result::Result<T,AppError>;

#[derive(Debug,Error)]
pub enum AppError {

    #[error(transparent)]
    DomainError(#[from] DomainError),

    #[error(transparent)]
    AppUserError(#[from] AppUserError),

    #[error("Other error:{0}")]
    Other(String)
}
