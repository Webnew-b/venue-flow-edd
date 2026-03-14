use thiserror::Error;

#[derive(Debug, Error)]
pub enum DomainUserError {
    #[error("The email has been used.")]
    EmailDuplication,

    #[error("The username has been used.")]
    UserNameDuplication,

    #[error("The user is not found.")]
    UserNotFound,

    #[error("The username is not existed.")]
    UserNameIsNotExist,

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

    #[error("Fail to generate token.")]
    InvalidTokenGeneration,

    #[error("Password is incorrect.")]
    PasswordIncorrect,

    #[error("Email format is illegal.")]
    EmailIsIllegal,

    #[error("Gender format is invalid.")]
    InvalidGender,
}
