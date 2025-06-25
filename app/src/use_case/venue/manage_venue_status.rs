use domain::venue_domain::VenueRepository;
use domain_core::utils::Clock;

use crate::app_error::AppResult;
use crate::commands::venue_commands::{ManageVenueRes, VenueStatusRes};
use crate::{AppUseCase, Outcome};

pub async fn publish_venue(
    repo:&impl VenueRepository,
    time:&impl Clock,
    id:i64
    )->AppResult<Outcome<ManageVenueRes>> {
    let venue = repo.find_venue_by_id(id).await?;
    let venue = venue.list_venue(time);
    repo.save_venue(venue).await?;
    let res = ManageVenueRes {
        id,
        status:VenueStatusRes::Published,
    };
    Ok(Outcome::new(res, AppUseCase::ManageVenueStatus))
}

pub async fn unpublish_venue(
    repo:&impl VenueRepository,
    time:&impl Clock,
    id:i64
    )->AppResult<Outcome<ManageVenueRes>> {
    let venue = repo.find_venue_by_id(id).await?;
    let venue = venue.unlist_venue(time);
    repo.save_venue(venue).await?;
    let res = ManageVenueRes {
        id,
        status:VenueStatusRes::UnPublished,
    };
    Ok(Outcome::new(res, AppUseCase::ManageVenueStatus))
}
