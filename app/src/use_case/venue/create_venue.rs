use domain::domain_error::DomainError;
use domain::user_domain::UserRepository;
use domain::venue_domain::VenueRepository;
use domain_core::utils::Clock;
use domain_core::venue::venue_image::VenueImage;
use domain_core::venue::VenueBuilder;
use garde::Validate;

use crate::app_error::{AppError, AppResult};
use crate::commands::venue_commands::{
    CreateVenueCommand, CreateVenueRes, VenueImageRes,
};
use crate::{AppUseCase, Outcome};

pub(super) async fn save_image_data(
    venue_repo: &impl VenueRepository,
    images: Vec<VenueImage>,
) -> AppResult<Vec<VenueImageRes>> {
    let images = venue_repo.save_image_data(images).await?;

    let mut res_image: Vec<VenueImageRes> = vec![];
    images.into_iter().try_for_each(|x| {
        let res: VenueImageRes = x.try_into()?;
        res_image.push(res);
        Ok::<(), DomainError>(())
    })?;
    Ok(res_image)
}

pub async fn create_venue(
    user_repo: &impl UserRepository,
    venue_repo: &impl VenueRepository,
    venue_create: CreateVenueCommand,
    time: &impl Clock,
) -> AppResult<Outcome<CreateVenueRes>> {
    let lessor = user_repo
        .find_lessor_by_user_id(venue_create.user_id)
        .await?;

    let lessor_id = lessor
        .id()
        .ok_or(AppError::IdInexisted("venue".to_string()))?;

    let bulider = VenueBuilder::default()
        .images(vec![])
        .name(venue_create.name)
        .lessor_id(lessor_id)
        .address(venue_create.address)
        .description(venue_create.description)
        .capacity(venue_create.capacity)
        .updatetime(time.now())
        .createtime(time.now())
        .build()
        .map_err(|e| AppError::CreateEntityFailed {
            entity_type: "venue".to_string(),
            message: e.to_string(),
            source: e,
        })?;

    bulider.validate().map_err(|e| AppError::EntityInvalid {
        entity_type: "venue".to_string(),
        cause: e.to_string(),
    })?;

    let venue_res = venue_repo.create_venue(bulider).await?;

    let id = venue_res
        .id()
        .ok_or(AppError::IdInexisted("venue".to_string()))?;

    let images = if venue_create.images.is_empty() {
        vec![]
    } else {
        let images = venue_create
            .images
            .into_iter()
            .map(|item| VenueImage {
                id: None,
                venue_id: id.clone(),
                title: item.title.to_string(),
                uri: item.image.to_string(),
                comment: item.comment.clone(),
                createtime: time.now(),
            })
            .collect();

        save_image_data(venue_repo, images).await?
    };
    let res = CreateVenueRes {
        id,
        name: venue_res.name().to_string(),
        address: venue_res.address().to_string(),
        images,
        capacity: venue_res.capacity().clone(),
        description: venue_res.description().clone(),
    };

    Ok(Outcome::new(res, AppUseCase::CreateVenue))
}
