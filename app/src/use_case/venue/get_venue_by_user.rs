use domain::domain_error::DomainError;
use domain::user_domain::UserRepository;
use domain::venue_domain::VenueRepository;
use domain::PageLimit;
use domain_core::user::lessor::Lessor;
use domain_core::venue::Venue;

use crate::app_error::{AppError, AppResult};
use crate::commands::venue_commands::VenueImageRes;
use crate::querys::venue_querys::GetVenueRes;
use crate::{AppUseCase, Outcome};

pub fn tranform_venue(
    venue: &Venue,
    lessor: &Lessor,
) -> AppResult<GetVenueRes> {
    let id = venue
        .id()
        .ok_or(AppError::IdInexisted("venue".to_string()))?;
    let lessor_id = lessor
        .id()
        .ok_or(AppError::IdInexisted("lessor".to_string()))?;

    let images: Vec<VenueImageRes> =
        venue.images().to_vec().clone().into_iter().try_fold(
            Vec::new(),
            |mut acc, x| {
                acc.push(x.try_into()?);
                Ok::<Vec<VenueImageRes>, DomainError>(acc)
            },
        )?;

    let res = GetVenueRes {
        id,
        name: venue.name().to_string(),
        address: venue.address().to_string(),
        images,
        capacity: venue.capacity().clone(),
        description: venue.description().clone(),
        lessor_id,
        lessor_name: lessor.user().username().to_string(),
        lessor_phone: lessor.phone().to_string(),
        lessor_avatar: lessor.user().avatar().to_string(),
    };
    Ok(res)
}

pub async fn get_venue_by_user(
    venue_repo: &impl VenueRepository,
    user_repo: &impl UserRepository,
    page: PageLimit,
    user_id: i64,
) -> AppResult<Outcome<Vec<GetVenueRes>>> {
    let lessor = user_repo.find_lessor_by_user_id(user_id).await?;
    let venue = venue_repo
        .find_venue_by_lessor_id(
            lessor.id().expect("lessor id must be existed."),
            page,
        )
        .await?;

    let res = venue
        .iter()
        .map(|v| tranform_venue(&v, &lessor))
        .collect::<AppResult<Vec<GetVenueRes>>>()?;

    let use_case_msg = format!("venue list from user id {}", user_id);
    Ok(Outcome::new(res, AppUseCase::GetData(use_case_msg)))
}
