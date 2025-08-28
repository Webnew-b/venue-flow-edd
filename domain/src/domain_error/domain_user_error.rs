use thiserror::Error;

#[derive(Debug, Error)]
pub enum DomainUserError {
    #[error("The email has been used.")]
    EmailDuplication,

    #[error("The user is not found.")]
    UserNotFound,

    #[error("Failed to contsturct domain user from db user.")]
    InvalidUserContstruction,

    #[error("Other message:{0}")]
    Other(String),

    #[error("Failed to contsturct domain organizer from db user.")]
    InvalidOrganizerContstruction,

    #[error("Failed to contsturct domain lessor from db user.")]
    InvalidLessorContstruction,

    #[error("The Email is not found")]
    EmailNotFound,

    #[error("The user name is not found")]
    UserNameNotFound,
}
