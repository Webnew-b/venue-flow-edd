use chrono::{DateTime, Utc};
use domain::domain_error::DomainError;
use domain::rental_domain::RentalRespository;
use domain_core::utils::Clock;

use crate::app_error::AppResult;
use crate::{AppUseCase, Outcome};

pub async fn update_rental_time(
    repo: &impl RentalRespository,
    organizer_id: i64,
    time_range: (DateTime<Utc>, DateTime<Utc>),
    id: i64,
    time: &impl Clock,
) -> AppResult<Outcome<()>> {
    let rental = repo.find_rental_by_id(id).await?;

    let rental = rental
        .set_rental_date(time, time_range.0, time_range.1, organizer_id)
        .map_err(|e| DomainError::EntityInvalid {
            entity_type: "rental".to_string(),
            cause: e.to_string(),
        })?;

    repo.save_rental(rental).await?;

    Ok(Outcome::new((), AppUseCase::UpdateRentalTime))
}
