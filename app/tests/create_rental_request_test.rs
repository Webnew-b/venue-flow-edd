use chrono::{DateTime, Utc};

use app::app_event::AppEvent;
use app::commands::rental_commands::CreateRentalCommand;
use app::use_case::rental::create_rental_request::create_rental_request;
use app::AppUseCase;
use domain_core::rental::{Rental, RentalBuilder, RentalStatus};
use domain_core::utils::Clock;

use crate::common::{fake_number};
use crate::common::rental_common::{mock_rental_setup, TestRentalMocks};
use crate::common::util_common::{MockTime};

mod common;

fn generate_mock_success<'test_mock>(
    rental_mock: &'test_mock mut TestRentalMocks,
    rental: Rental,
) -> &'test_mock TestRentalMocks {
    rental_mock.repo.expect_create_rental_request()
        .times(1)
        .return_once(move |_| Ok(rental));

    rental_mock
}

fn fake_rental() -> Rental {
    let time = MockTime {};
    let start_time: DateTime<Utc> = "2024-01-01T10:00:00Z".parse().unwrap();
    let end_time: DateTime<Utc> = "2024-01-01T12:00:00Z".parse().unwrap();
    
    RentalBuilder::default()
        .id(Some(fake_number()))
        .activity_type("test".to_string())
        .venue_id(fake_number())
        .organizer_id(fake_number())
        .start_time(start_time)
        .end_time(end_time)
        .status(RentalStatus::Pending)
        .createtime(time.now())
        .updatetime(time.now())
        .build().unwrap()
}

#[tokio::test]
async fn test_create_rental_request_success() {
    let mut rental_mock = mock_rental_setup();
    let rental = fake_rental();
    let venue_id = rental.venue_id().clone();
    let organizer_id = rental.organizer_id().clone();
    let start_time = rental.start_time().clone();
    let end_time = rental.end_time().clone();
    
    let rental_mock = generate_mock_success(&mut rental_mock, rental.clone());

    let repo = &rental_mock.repo;
    let time = MockTime {};

    let create_command = CreateRentalCommand {
        venue_id,
        organizer_id,
        start_time,
        end_time,
        activity_type: "Conference".to_string(),
        request_comments: Some("Test rental request".to_string()),
    };

    let res = create_rental_request(
        repo,
        &time,
        create_command,
    ).await;

    match res {
        Ok(o) => {
            let data = o.data;
            let event = o.events.get_elements();
            let use_case = o.from_case;

            let test_vec = vec![AppEvent::LogUseCase];
            assert_eq!(test_vec, event, "The event doesn't equal.");
            assert_eq!(use_case, AppUseCase::CreateRentalRequest);

            assert_eq!(data.id, rental.id().unwrap());
            assert_eq!(data.venue_id, rental.venue_id().clone());
            assert_eq!(data.organizer_id, rental.organizer_id().clone());
            assert_eq!(data.start_time, rental.start_time().format("%Y-%m-%d %H:%M:%S").to_string());
            assert_eq!(data.end_time, rental.end_time().format("%Y-%m-%d %H:%M:%S").to_string());
        },
        Err(e) => panic!("Unexpected error: {}", e),
    }
}
