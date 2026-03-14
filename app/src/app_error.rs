use domain::domain_error::DomainError;

pub type AppResult<T> = std::result::Result<T, DomainError>;
