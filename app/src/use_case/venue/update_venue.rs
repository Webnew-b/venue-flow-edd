use domain::util_trait::ImageRepository;
use domain::venue_domain::VenueRepository;
use domain_core::utils::Clock;
use domain_core::venue::venue_image::VenueImage;
use domain_core::venue::venue_update::VenueUpdate;

use crate::app_error::{AppError, AppResult};
use crate::commands::venue_commands::UpdateVenueCommand;
use crate::{AppUseCase, Outcome};

async fn get_update_struct<'image>(
    update:UpdateVenueCommand<'image>,
    image_repo:&impl ImageRepository,
    )
    -> AppResult<(VenueUpdate,Vec<VenueImage>)> {
    
    let mut update_struct = VenueUpdate::new();
    let mut images:Vec<VenueImage> = vec![];

    if let Some(g) = update.address {
        update_struct.address = Some(g);
    };

    if update.images.len() > 0 {
        for item in update.images  {
            let uri = image_repo.upload_image(item.image).await?;
            images.push(VenueImage{
                title:item.title,
                uri,
                comment:item.comment,
            });
        }
    }

    if let Some(n) = update.name{
        update_struct.name = Some(n);
    }

    if let Some(e) = update.capacity{
        //todo verify email and send event
        update_struct.capacity = Some(e);
    }
    update_struct.description = update.description;

    Ok((update_struct,images))
}

pub async fn update_user<'image>(
    repo:&impl VenueRepository,
    image_repo:&impl ImageRepository,
    update:UpdateVenueCommand<'image>,
    clock:&impl Clock,
    )-> AppResult<Outcome<()>> {
    let id = update.id;
    let (update_struct,images) = get_update_struct(update,image_repo).await?;
    let venue = repo.find_venue_by_id(id).await?;
    let venue = venue.update_venue(update_struct,clock).map_err(|e|{
        AppError::UpdateEntityFailed {
            entity_type:"user".to_string(),
            message: e.to_string(), 
            source:e
        }
    })?;
    let venue = venue.update_images(images, clock);
    repo.save_venue(venue).await?;
    Ok(Outcome::new((),AppUseCase::BasicUserProfile))
}
