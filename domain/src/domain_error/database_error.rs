use thiserror::Error;


#[derive(Debug,Error)]
pub enum DatabaseError {
    #[error(transparent)]
    DomainUserError(#[from] DomainUserError),
    #[error("Other message:{0}")]
    Other(String)
}


#[derive(Debug,Error)]
pub enum DomainUserError {
    
    #[error("The email has been used.")]
    EmailDuplication,

    #[error("Other message:{0}")]
    Other(String)
}
