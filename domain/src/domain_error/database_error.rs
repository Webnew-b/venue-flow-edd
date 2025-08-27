use thiserror::Error;

#[derive(Debug, Error)]
pub enum DatabaseError {
    #[error("Other message:{0}")]
    Other(String),

    #[error("Conld not select the element in database.")]
    SelectFail,

    #[error("Data is not found.")]
    DataNotFound,

    #[error("Failed to save entity.")]
    SaveEntityFail,

    #[error("Failed to delete the entity.")]
    DeleteEntityFail,

    #[error("Fail to select the preant entity.")]
    SelectPreantEntityFail,
}
