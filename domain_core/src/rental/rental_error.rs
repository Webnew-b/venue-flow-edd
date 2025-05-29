use thiserror::Error;

#[derive(Debug,Error)]
pub enum RentalError {
    #[error("Field validated fail,cause:\n {0}")]
    FieldValidatedFail(String),

    #[error("Rental time is invalid,start_time:{0},end_time:{1}")]
    InvalidRentalTime(String,String),

    #[error("Rental start time must be future.")]
    RentalStartTimeMustBeFuture,
}
