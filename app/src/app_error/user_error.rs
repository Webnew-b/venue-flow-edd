use thiserror::Error;

#[derive(Error, Debug)]
pub enum AppUserError {
    #[error(
        "Invalid gender,gender accept:male,female,not-binary,prefer-not-to-say"
    )]
    InvalidGender,
    #[error("The email is illegal.")]
    EmailIllegal,

    #[error("The {0} or password is incorrect.")]
    LoginIncrrect(String),
    #[error("Other error:{0}")]
    Other(String),
}
