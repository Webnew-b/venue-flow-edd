use thiserror::Error;

#[derive(Debug,Error)]
pub enum VenueError {
    #[error("Field validated fail,cause:\n {0}")]
    FieldValidatedFail(String),
}
