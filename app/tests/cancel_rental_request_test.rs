use chrono::{DateTime, Utc};

use app::app_event::AppEvent;
use app::use_case::rental::cancel_rental_request::cancel_rental_request;
use app::AppUseCase;
use domain_core::rental::{Rental, RentalBuilder, RentalStatus};
use domain_core::user::lessor::{Lessor, LessorBuilder};
use domain_core::user::organizer::{Organizer, OrganizerBuilder};
use domain_core::user::{User, UserBuilder, UserGender};
use domain_core::utils::Clock;

use crate::common::rental_common::{mock_rental_setup, TestRentalMocks};
use crate::common::user_common::{mock_user_setup, TestUserMocks};
use crate::common::util_common::MockTime;
use crate::common::venue_common::{mock_venue_setup, TestVenueMocks};
use crate::common::{fake_email, fake_number, fake_username};

mod common;

fn generate_mock_success<'test_mock>(
    rental_mock: &'test_mock mut TestRentalMocks,
    venue_mock: &'test_mock mut TestVenueMocks,
    user_mock: &'test_mock mut TestUserMocks,
    rental: Rental,
    lessor: Lessor,
    organizer: Organizer,
) -> (
    &'test_mock TestRentalMocks,
    &'test_mock TestVenueMocks,
    &'test_mock TestUserMocks,
) {
    rental_mock
        .repo
        .expect_find_rental_by_id()
        .times(1)
        .return_once(move |_| Ok(rental));

    venue_mock
        .repo
        .expect_find_lessor_by_venue_id()
        .times(1)
        .return_once(move |_| Ok(lessor));

    user_mock
        .repo
        .expect_find_organizer_by_id()
        .times(1)
        .return_once(move |_| Ok(organizer));

    rental_mock
        .repo
        .expect_save_rental()
        .times(1)
        .return_once(move |_| Ok(()));

    (rental_mock, venue_mock, user_mock)
}

fn fake_user_for_lessor() -> User {
    let time = MockTime {};

    UserBuilder::default()
        .id(Some(fake_number()))
        .username(fake_username())
        .email(fake_email())
        .password("password123".to_string())
        .avatar("avatar.jpg".to_string())
        .gender(UserGender::Male)
        .createtime(time.now())
        .updatetime(time.now())
        .build()
        .unwrap()
}

fn fake_user_for_organizer() -> User {
    let time = MockTime {};

    UserBuilder::default()
        .id(Some(fake_number()))
        .username(fake_username())
        .email(fake_email())
        .password("password123".to_string())
        .avatar("avatar.jpg".to_string())
        .gender(UserGender::Female)
        .createtime(time.now())
        .updatetime(time.now())
        .build()
        .unwrap()
}

fn fake_lessor() -> Lessor {
    let time = MockTime {};
    let user = fake_user_for_lessor();

    LessorBuilder::default()
        .id(Some(2))
        .user(user)
        .phone("12345678901".to_string())
        .createtime(time.now())
        .updatetime(time.now())
        .build()
        .unwrap()
}

fn fake_organizer() -> Organizer {
    let time = MockTime {};
    let user = fake_user_for_organizer();

    OrganizerBuilder::default()
        .id(Some(1))
        .user(user)
        .phone("12345678901".to_string())
        .createtime(time.now())
        .updatetime(time.now())
        .build()
        .unwrap()
}

fn fake_rental() -> Rental {
    let time = MockTime {};
    let start_time: DateTime<Utc> = "2024-01-01T10:00:00Z".parse().unwrap();
    let end_time: DateTime<Utc> = "2024-01-01T12:00:00Z".parse().unwrap();

    RentalBuilder::default()
        .id(Some(fake_number()))
        .venue_id(2)
        .organizer_id(1)
        .activity_type("test".to_string())
        .start_time(start_time)
        .end_time(end_time)
        .status(RentalStatus::Pending)
        .createtime(time.now())
        .updatetime(time.now())
        .build()
        .unwrap()
}

#[tokio::test]
async fn test_cancel_rental_request_success() {
    let mut rental_mock = mock_rental_setup();
    let mut venue_mock = mock_venue_setup();
    let mut user_mock = mock_user_setup();

    let rental = fake_rental();
    let lessor = fake_lessor();
    let organizer = fake_organizer();
    let rental_id = rental.id().unwrap();
    let organizer_id = organizer.id().unwrap();

    let (rental_mock, venue_mock, user_mock) = generate_mock_success(
        &mut rental_mock,
        &mut venue_mock,
        &mut user_mock,
        rental,
        lessor.clone(),
        organizer.clone(),
    );

    let rental_repo = &rental_mock.repo;
    let venue_repo = &venue_mock.repo;
    let user_repo = &user_mock.repo;
    let time = MockTime {};

    let res = cancel_rental_request(
        rental_repo,
        user_repo,
        venue_repo,
        organizer_id,
        rental_id,
        &time,
    )
    .await;

    match res {
        Ok(o) => {
            let events = o.events.get_elements();
            let use_case = o.from_case;

            assert_eq!(use_case, AppUseCase::CancelRentalRequest);
            assert_eq!(events.len(), 2);

            let expected_events = vec![
                AppEvent::LogUseCase,
                AppEvent::CanceledRentalRequest {
                    organizer_email: organizer.user().email().to_string(),
                    organizer_name:  organizer.user().username().to_string(),
                    organizer_id:    organizer.id().unwrap(),
                    lessor_id:       lessor.id().unwrap(),
                    lessor_name:     lessor.user().username().to_string(),
                    lessor_email:    lessor.user().email().to_string(),
                },
            ];
            assert_eq!(events, expected_events, "The events don't match.");
        },
        Err(e) => panic!("Unexpected error: {}", e),
    }
}
