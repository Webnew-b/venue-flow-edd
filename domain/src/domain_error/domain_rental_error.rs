use thiserror::Error;

#[derive(Debug, Error)]
pub enum DomainRentalError {
    #[error("The user is not found.")]
    UserNotFound,

    #[error("Other message:{0}")]
    Other(String),

    #[error("Failed to contsturct domain rental from db rental.")]
    InvalidRentalContstruction,
}
