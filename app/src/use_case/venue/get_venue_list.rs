use domain::venue_domain::venue_dto::IndexVenue;
use domain::venue_domain::VenueRepository;
use domain::PageLimit;

use crate::app_error::AppResult;
use crate::{AppUseCase, Outcome};

pub async fn get_venue_by_user(
    venue_repo: &impl VenueRepository,
    p: PageLimit,
) -> AppResult<Outcome<Vec<IndexVenue>>> {
    let venue = venue_repo.get_venues_for_index(p).await?;
    let use_case_msg = format!("venue list for index");
    Ok(Outcome::new(venue, AppUseCase::GetData(use_case_msg)))
}
