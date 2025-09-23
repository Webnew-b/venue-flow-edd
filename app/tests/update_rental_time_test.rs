use chrono::{DateTime, Utc};

use app::app_event::AppEvent;
use app::use_case::rental::update_rental_time::reject_rental_request;
use app::AppUseCase;
use domain_core::rental::{Rental, RentalBuilder, RentalStatus};
use domain_core::utils::Clock;

use crate::common::fake_number;
use crate::common::rental_common::{mock_rental_setup, TestRentalMocks};
use crate::common::util_common::MockTime;

mod common;

fn generate_mock_success<'test_mock>(
    rental_mock: &'test_mock mut TestRentalMocks,
    rental: Rental,
) -> &'test_mock TestRentalMocks {
    rental_mock
        .repo
        .expect_find_rental_by_id()
        .times(1)
        .return_once(move |_| Ok(rental));

    rental_mock
        .repo
        .expect_save_rental()
        .times(1)
        .return_once(move |_| Ok(()));

    rental_mock
}

fn fake_rental() -> Rental {
    let time = MockTime {};
    let start_time: DateTime<Utc> = "2026-01-01T10:00:00Z".parse().unwrap();
    let end_time: DateTime<Utc> = "2026-01-03T12:00:00Z".parse().unwrap();

    RentalBuilder::default()
        .id(Some(fake_number()))
        .venue_id(1)
        .activity_type("test".to_string())
        .organizer_id(1)
        .start_time(start_time)
        .end_time(end_time)
        .status(RentalStatus::Pending)
        .createtime(time.now())
        .updatetime(time.now())
        .build()
        .unwrap()
}

#[tokio::test]
async fn test_update_rental_time_success() {
    let mut rental_mock = mock_rental_setup();
    let rental = fake_rental();
    let rental_id = rental.id().unwrap();
    let organizer_id = 1;

    let rental_mock = generate_mock_success(&mut rental_mock, rental);

    let repo = &rental_mock.repo;
    let time = MockTime {};

    let new_start_time: DateTime<Utc> = "2026-01-03T14:00:00Z".parse().unwrap();
    let new_end_time: DateTime<Utc> = "2026-01-05T16:00:00Z".parse().unwrap();
    let time_range = (new_start_time, new_end_time);

    let res =
        reject_rental_request(repo, organizer_id, time_range, rental_id, &time)
            .await;

    match res {
        Ok(o) => {
            let event = o.events.get_elements();
            let use_case = o.from_case;

            let test_vec = vec![AppEvent::LogUseCase];
            assert_eq!(test_vec, event, "The event doesn't equal.");
            assert_eq!(use_case, AppUseCase::UpdateRentalTime);
        },
        Err(e) => panic!("Unexpected error: {}", e),
    }
}
