use domain::rental_domain::RentalRespository;
use domain::user_domain::UserRepository;
use domain::venue_domain::VenueRepository;
use domain_core::utils::Clock;

use crate::app_error::rental_error::AppRentalError;
use crate::app_error::{AppError, AppResult};
use crate::app_event::{AppEvent, AppEventList};
use crate::{AppUseCase, Outcome};


pub async fn approve_rental_request(
    repo:&impl RentalRespository,
    venue_repo:&impl VenueRepository,
    user_repo:&impl UserRepository,
    lessor_id:i64,
    id:i64,
    time:&impl Clock,
    ) -> AppResult<Outcome<()>>{

    let rental = repo.find_rental_by_id(id).await?;

    let organizer = user_repo.find_organizer_by_id(
        rental.organizer_id().clone()
    ).await?;

    venue_repo.is_venue_owned_by_lessor(
        lessor_id,
        rental.venue_id().clone()
    ).await?
    .then_some(())
    .ok_or(AppRentalError::VenueNotOwnedLessor)?;

    let rental = rental.accepet_rental(time).map_err(|e|{
        AppError::EntityInvalid { 
            entity_type: "rental".to_string(),
            cause: e.to_string()
        }
    })?;

    repo.save_rental(rental).await?;

    let organizer_id = organizer.id()
        .ok_or(AppError::IdInexisted("organizer".to_string()))?;

    let mut events = AppEventList::new();

    events.push(AppEvent::ApprovedRentalRequest { 
        organizer_email: organizer.user().email().to_string(),
        organizer_name: organizer.user().username().to_string(),
        organizer_id ,
    });

    let outcome = Outcome::new_with_events(
        (), AppUseCase::ProcessRentalRequests, events);

    Ok(outcome)
}

pub async fn reject_rental_request(
    repo:&impl RentalRespository,
    venue_repo:&impl VenueRepository,
    user_repo:&impl UserRepository,
    lessor_id:i64,
    id:i64,
    time:&impl Clock,
    ) -> AppResult<Outcome<()>>{

    let rental = repo.find_rental_by_id(id).await?;

    let organizer = user_repo.find_organizer_by_id(
        rental.organizer_id().clone()
    ).await?;

    venue_repo.is_venue_owned_by_lessor(
        lessor_id,
        rental.venue_id().clone()
    ).await?
    .then_some(())
    .ok_or(AppRentalError::VenueNotOwnedLessor)?;

    let rental = rental.reject_rental(time).map_err(|e|{
        AppError::EntityInvalid { 
            entity_type: "rental".to_string(),
            cause: e.to_string()
        }
    })?;

    repo.save_rental(rental).await?;

    let organizer_id = organizer.id()
        .ok_or(AppError::IdInexisted("organizer".to_string()))?;

    let mut events = AppEventList::new();

    events.push(AppEvent::RejectedRentalRequest { 
        organizer_email: organizer.user().email().to_string(),
        organizer_name: organizer.user().username().to_string(),
        organizer_id ,
    });

    let outcome = Outcome::new_with_events(
        (), AppUseCase::ProcessRentalRequests, events);

    Ok(outcome)
}

