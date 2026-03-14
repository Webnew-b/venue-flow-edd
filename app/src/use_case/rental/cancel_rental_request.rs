use domain::domain_error::DomainError;
use domain::rental_domain::RentalRespository;
use domain::user_domain::UserRepository;
use domain::venue_domain::VenueRepository;
use domain_core::utils::Clock;

use crate::app_error::AppResult;
use crate::app_event::{AppEvent, AppEventList};
use crate::{AppUseCase, Outcome};

pub async fn cancel_rental_request(
    repo: &impl RentalRespository,
    user_repo: &impl UserRepository,
    venue_repo: &impl VenueRepository,
    organizer_id: i64,
    id: i64,
    time: &impl Clock,
) -> AppResult<Outcome<()>> {
    let rental = repo.find_rental_by_id(id).await?;

    let lessor = venue_repo
        .find_lessor_by_venue_id(rental.venue_id().clone())
        .await?;

    let organizer = user_repo.find_organizer_by_id(organizer_id).await?;

    let rental = rental.cancel_rental(organizer_id, time).map_err(|e| {
        DomainError::EntityInvalid {
            entity_type: "rental".to_string(),
            cause:       e.to_string(),
        }
    })?;

    repo.save_rental(rental).await?;

    let mut events = AppEventList::new();

    let organizer_id = organizer
        .id()
        .ok_or(DomainError::IdInexisted("organizer".to_string()))?;
    let lessor_id = lessor
        .id()
        .ok_or(DomainError::IdInexisted("organizer".to_string()))?;

    events.push(AppEvent::CanceledRentalRequest {
        organizer_email: organizer.user().email().to_string(),
        organizer_name: organizer.user().username().to_string(),
        organizer_id,
        lessor_id,
        lessor_name: lessor.user().username().to_string(),
        lessor_email: lessor.user().email().to_string(),
    });

    let outcome =
        Outcome::new_with_events((), AppUseCase::CancelRentalRequest, events);

    Ok(outcome)
}
