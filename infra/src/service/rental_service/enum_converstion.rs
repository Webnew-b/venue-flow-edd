use domain_core::rental::ActivityType as CoreAType;
use domain_core::rental::RentalStatus as CoreRentalStatus;

use crate::database::entities::sea_orm_active_enums::ActivityType;
use crate::database::entities::sea_orm_active_enums::RequestStatus;

pub(crate) fn rental_status_to_db(value: CoreRentalStatus) -> RequestStatus {
    match value {
        CoreRentalStatus::Pending => RequestStatus::Pending,
        CoreRentalStatus::Accepted => RequestStatus::Accepted,
        CoreRentalStatus::Rejected => RequestStatus::Rejected,
        CoreRentalStatus::Finished => RequestStatus::Finished,
        CoreRentalStatus::Canceled => RequestStatus::Canceled,
    }
}

pub(crate) fn rental_status_to_domain(
    value: RequestStatus,
) -> CoreRentalStatus {
    match value {
        RequestStatus::Pending => CoreRentalStatus::Pending,
        RequestStatus::Accepted => CoreRentalStatus::Accepted,
        RequestStatus::Rejected => CoreRentalStatus::Rejected,
        RequestStatus::Finished => CoreRentalStatus::Finished,
        RequestStatus::Canceled => CoreRentalStatus::Canceled,
    }
}

pub(crate) fn rental_activity_type_to_db(value: CoreAType) -> ActivityType {
    match value {
        CoreAType::All => ActivityType::All,
        CoreAType::Exhibition => ActivityType::Exhibition,
        CoreAType::Seminar => ActivityType::Seminar,
    }
}

pub(crate) fn rental_activity_type_to_domain(value: ActivityType) -> CoreAType {
    match value {
        ActivityType::All => CoreAType::All,
        ActivityType::Exhibition => CoreAType::Exhibition,
        ActivityType::Seminar => CoreAType::Seminar,
    }
}
