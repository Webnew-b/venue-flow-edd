use domain::venue_domain::VenueRepository;
use domain_core::utils::Clock;
use domain_core::venue::venue_update::VenueUpdate;

use crate::app_error::{AppError, AppResult};
use crate::commands::venue_commands::UpdateVenueCommand;
use crate::{AppUseCase, Outcome};

async fn get_update_struct(
    update:UpdateVenueCommand,
    )
    -> AppResult<VenueUpdate> {
    
    let mut update_struct = VenueUpdate::new();

    if let Some(g) = update.address {
        update_struct.address = Some(g);
    };

    //todo update file to the oss

    if let Some(n) = update.name{
        update_struct.name = Some(n);
    }

    if let Some(e) = update.capacity{
        //todo verify email and send event
        update_struct.capacity = Some(e);
    }
    update_struct.description = update.description;

    Ok(update_struct)
}

pub async fn update_user(
    repo:&impl VenueRepository,
    update:UpdateVenueCommand,
    clock:&impl Clock,
    )-> AppResult<Outcome<()>> {
    let id = update.id;
    let update_struct = get_update_struct(update).await?;
    let venue = repo.find_venue_by_id(id).await?;
    let user = venue.update_venue(update_struct,clock).map_err(|e|{
        AppError::UpdateEntityFailed {
            entity_type:"user".to_string(),
            message: e.to_string(), 
            source:e
        }
    })?;
    repo.save_venue(user).await?;
    Ok(Outcome::new((),AppUseCase::BasicUserProfile))
}
