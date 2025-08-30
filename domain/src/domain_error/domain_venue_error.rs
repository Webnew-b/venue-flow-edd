use thiserror::Error;

#[derive(Debug, Error)]
pub enum DomainVenueError {
    #[error("Other message:{0}")]
    Other(String),

    #[error("Failed to contsturct domain veune from db venue.")]
    InvalidVeuneContstruction,

    #[error("Failed to contsturct domain lessor from db user.")]
    InvalidLessorContstruction,
}
