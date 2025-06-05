use thiserror::Error;


#[derive(Error,Debug)]
pub enum AppUserError {
    #[error("The user id should be existed.")]
    UserIdInexisted,
    #[error("Could not create user entity,cause:{message}")]
    CreateUserEntityFailed {
        message:String,
        #[source]
        source:domain_core::domain_core_error::DomainCoreError,
    },
    #[error("Could not update user entity,cause:{message}")]
    UpdateUserEntityFailed {
        message:String,
        #[source]
        source:domain_core::domain_core_error::DomainCoreError,
    },
    #[error("Invalid gender,gender accept:male,female,not-binary,prefer-not-to-say")]
    InvalidGender,
    #[error("The user is illegal,cause:{0}")]
    UserIllegal(String),
    #[error("The email is illegal.")]
    EmailIllegal,

    #[error("The {0} or password is incorrect.")]
    LoginIncrrect(String),
    #[error("Other error:{0}")]
    Other(String),
}
