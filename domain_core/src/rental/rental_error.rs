use thiserror::Error;

#[derive(Debug, Error)]
pub enum RentalError {
    #[error("The Field validated fail,cause:\n {0}")]
    FieldValidatedFail(String),

    #[error("The Rental time is invalid,start_time:{0},end_time:{1}")]
    InvalidRentalTime(String, String),

    #[error("The Rental start time must be future.")]
    RentalStartTimeMustBeFuture,

    #[error("The Rental must be Pending")]
    RentalMustBePending,

    #[error("The Rental must be Accepted")]
    RentalMustBeAccepted,

    #[error("The rental is not owned by organizer {0}")]
    RentalNotOwnedOrganizer(i64),
}
