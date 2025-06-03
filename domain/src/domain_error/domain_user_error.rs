use thiserror::Error;

#[derive(Debug,Error)]
pub enum DomainUserError {
    
    #[error("The email has been used.")]
    EmailDuplication,

    #[error("The user is not found.")]
    UserNotFound,

    #[error("Other message:{0}")]
    Other(String)
}
