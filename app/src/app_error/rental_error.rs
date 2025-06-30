use thiserror::Error;

#[derive(Error,Debug)]
pub enum AppRentalError {
    #[error("The venue is not owned lessor.")]
    VenueNotOwnedLessor,

    #[error("Other error:{0}")]
    Other(String),
}
