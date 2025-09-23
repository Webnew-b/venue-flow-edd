use thiserror::Error;

#[derive(Debug, Error)]
pub enum UserError {
    #[error("Field validated fail,cause:\n {0}")]
    FieldValidatedFail(String),
}
