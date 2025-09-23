use thiserror::Error;

#[derive(Error, Debug)]
pub enum InfraError {
    #[error("Fail to hash the password:{password}")]
    HashPasswordFail { password: String },

    #[error("Fail to verify the password")]
    VerifyPasswordFail,

    #[error("Fail to save image to repository.")]
    SaveImageFail,

    #[error("Fail to get file extension type.")]
    ObtainExtensionFail,

    #[error("File is not found.")]
    FileNotFound,

    #[error("File does not be read.")]
    FileNotRead,

    #[error("File type is invalid.")]
    FileTypeIsInvalid,
}
