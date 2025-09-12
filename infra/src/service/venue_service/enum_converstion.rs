use domain_core::venue::VenueStatus as CoreVenueStatus;

use crate::database::entities::sea_orm_active_enums::VenueState;

pub(crate) fn venue_status_to_db(value: CoreVenueStatus) -> VenueState {
    match value {
        CoreVenueStatus::Published => VenueState::Published,
        CoreVenueStatus::Unpublished => VenueState::Unpublished,
    }
}

pub(crate) fn venue_status_to_domain(value: VenueState) -> CoreVenueStatus {
    match value {
        VenueState::Published => CoreVenueStatus::Published,
        VenueState::Unpublished => CoreVenueStatus::Unpublished,
    }
}
