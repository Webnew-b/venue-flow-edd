use domain::domain_error::DomainError;
use domain::user_domain::UserRepository;
use domain::venue_domain::VenueRepository;

use crate::app_error::{AppError, AppResult};
use crate::commands::venue_commands::VenueImageRes;
use crate::querys::venue_querys::GetVenueRes;
use crate::{AppUseCase, Outcome};

pub async fn get_venue(
    venue_repo: &impl VenueRepository,
    user_repo: &impl UserRepository,
    id: i64,
) -> AppResult<Outcome<GetVenueRes>> {
    let veune = venue_repo.find_venue_by_id(id).await?;
    let lessor_id = veune.lessor_id().clone();
    let lessor = user_repo.find_lessor_by_id(lessor_id.clone()).await?;

    let id = veune
        .id()
        .ok_or(AppError::IdInexisted("veune".to_string()))?;

    let images: Vec<VenueImageRes> = veune
        .images()
        .clone()
        .into_iter()
        .try_fold(Vec::new(), |mut acc, x| {
            acc.push(x.try_into()?);
            Ok::<Vec<VenueImageRes>, DomainError>(acc)
        })?;

    let res = GetVenueRes {
        id,
        name: veune.name().to_string(),
        address: veune.address().to_string(),
        images,
        capacity: veune.capacity().clone(),
        description: veune.description().clone(),
        lessor_id,
        lessor_name: lessor.user().username().to_string(),
        lessor_phone: lessor.phone().to_string(),
        lessor_avatar: lessor.user().avatar().to_string(),
    };
    let use_case_msg = format!("veune detail from id {}", id);
    Ok(Outcome::new(res, AppUseCase::GetData(use_case_msg)))
}
