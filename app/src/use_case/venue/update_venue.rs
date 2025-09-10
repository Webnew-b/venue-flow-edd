use domain::util_trait::ImageRepository;
use domain::venue_domain::VenueRepository;
use domain_core::utils::Clock;
use domain_core::venue::venue_update::VenueUpdate;

use crate::app_error::{AppError, AppResult};
use crate::commands::venue_commands::{
    ImageDeleteCommand, ImageUploadCommand, ImageUploadRes, UpdateVenueCommand,
};
use crate::use_case::venue::create_venue::{
    create_image_data, save_image_data,
};
use crate::{AppUseCase, Outcome};

async fn get_update_struct<'image>(
    update: UpdateVenueCommand<'image>,
) -> AppResult<VenueUpdate> {
    let mut update_struct = VenueUpdate::new();

    if let Some(g) = update.address {
        update_struct.address = Some(g);
    };

    if let Some(n) = update.name {
        update_struct.name = Some(n);
    }

    if let Some(e) = update.capacity {
        //todo verify email and send event
        update_struct.capacity = Some(e);
    }
    update_struct.description = update.description;

    Ok(update_struct)
}

pub async fn update_venue<'image>(
    repo: &impl VenueRepository,
    update: UpdateVenueCommand<'image>,
    clock: &impl Clock,
) -> AppResult<Outcome<()>> {
    let id = update.id;
    let update_struct = get_update_struct(update).await?;
    let venue = repo.find_venue_by_id(id).await?;
    let venue = venue.update_venue(update_struct, clock).map_err(|e| {
        AppError::UpdateEntityFailed {
            entity_type: "user".to_string(),
            message: e.to_string(),
            source: e,
        }
    })?;
    repo.save_venue(venue).await?;
    Ok(Outcome::new((), AppUseCase::BasicUserProfile))
}

pub async fn upload_image<'image>(
    repo: &impl VenueRepository,
    image_repo: &impl ImageRepository,
    time: &impl Clock,
    images: ImageUploadCommand<'image>,
) -> AppResult<Outcome<ImageUploadRes>> {
    let venue_id = images.venue_id;
    let res =
        create_image_data(image_repo, time, venue_id, images.images.as_ref())
            .await?;
    let images = save_image_data(repo, res).await?;
    Ok(Outcome::new(
        ImageUploadRes { venue_id, images },
        AppUseCase::UploadVenueImage,
    ))
}

pub async fn delete_image(
    repo: &impl VenueRepository,
    deletion: ImageDeleteCommand,
) -> AppResult<Outcome<()>> {
    repo.delete_images(deletion.image_id, deletion.venue_id)
        .await?;
    Ok(Outcome::new((), AppUseCase::DeleteVenueImage))
}
