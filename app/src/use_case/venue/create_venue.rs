use domain::user_domain::UserRepository;
use domain::venue_domain::VenueRepository;
use domain_core::utils::Clock;
use domain_core::venue::VenueBuilder;
use garde::Validate;

use crate::app_error::{AppError, AppResult};
use crate::commands::venue_commands::{CreateVenueRes, VenueImageRes};
use crate::{AppUseCase, Outcome};

pub async fn create_venue(
    user_repo:&impl UserRepository,
    veune_repo:&impl VenueRepository,
    venue_create:CreateVenueRes,
    time:&impl Clock,
) -> AppResult<Outcome<CreateVenueRes>>{
    let lessor = user_repo
        .find_lessor_by_user_id(venue_create.id)
        .await?;

    let lessor_id = lessor.id()
        .ok_or(AppError::IdInexisted("veune".to_string()))?;

    let bulider = VenueBuilder::default()
        .name(venue_create.name)
        .lessor_id(lessor_id)
        .address(venue_create.address)
        .description(venue_create.description)
        .capacity(venue_create.capacity)
        .updatetime(time.now())
        .createtime(time.now())
        .build().map_err(|e|{
            AppError::CreateEntityFailed { 
                entity_type: "venue".to_string(),
                message: e.to_string(), 
                source: e
            }
        })?;

    bulider.validate().map_err(|e|{
        AppError::EntityInvalid { 
            entity_type: "user".to_string(),
            cause: e.to_string()
        }
    })?;

    let venue_res = veune_repo.create_venue(bulider).await?;

    let id = venue_res.id().ok_or(AppError::IdInexisted("venue".to_string()))?;

    let res_image:Vec<VenueImageRes> = venue_res.images()
        .to_vec()
        .clone()
        .into_iter()
        .map(|x|{
            x.into()
        })
        .collect();

    let res =  CreateVenueRes {
        id,
        name:venue_res.name().to_string(),
        address:venue_res.address().to_string(),
        images:res_image,
        capacity:venue_res.capacity().clone(),
        description:venue_res.description().cloned(),
    };

    Ok(Outcome::new(res, AppUseCase::CreateVenue))
}
