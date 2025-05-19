use thiserror::Error;

use crate::user::user_error::UserError;

pub type DomainCoreResult<T> = std::result::Result<T,DomainCoreError>;




#[derive(Debug,Error)]
pub enum DomainCoreError {
    #[error(transparent)]
    UserError(#[from] UserError),

    #[error("Build entity fail,cause:{0}")]
    EntityBuildFail(String),

    #[error("Field {0} is missing.")]
    MissingField(String),

    #[error("Must include a update field.")]
    MustIncludeFieldForUpdate,

    #[error("Other error:{0}")]
    Other(String),
}

impl From<derive_builder::UninitializedFieldError> for DomainCoreError {
    fn from(value: derive_builder::UninitializedFieldError) -> Self {
        DomainCoreError::EntityBuildFail(value.to_string())
    }
}
