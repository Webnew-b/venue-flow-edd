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
    #[error("Invalid gender,gender accept:male,female,not-binary,prefer-not-to-say")]
    InvalidGender,
    #[error("Other error:{0}")]
    Other(String),
}
