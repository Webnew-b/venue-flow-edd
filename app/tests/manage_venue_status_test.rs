use app::app_event::AppEvent;
use app::commands::venue_commands::VenueStatusRes;
use app::use_case::venue::manage_venue_status::{
    publish_venue, unpublish_venue,
};
use app::AppUseCase;
use domain_core::utils::Clock;
use domain_core::venue::venue_image::VenueImage;
use domain_core::venue::{Venue, VenueBuilder, VenueStatus};

use crate::common::util_common::MockTime;
use crate::common::venue_common::{mock_venue_setup, TestVenueMocks};
use crate::common::{fake_address, fake_name, fake_number, fake_number_i32};

mod common;

fn generate_mock_success_publish<'test_mock>(
    venue_mock: &'test_mock mut TestVenueMocks,
    venue: Venue,
) -> &'test_mock TestVenueMocks {
    venue_mock
        .repo
        .expect_find_venue_by_id()
        .times(1)
        .return_once(move |_| Ok(venue));

    venue_mock
        .repo
        .expect_save_venue()
        .times(1)
        .return_once(move |_| Ok(()));

    venue_mock
}

fn generate_mock_success_unpublish<'test_mock>(
    venue_mock: &'test_mock mut TestVenueMocks,
    venue: Venue,
) -> &'test_mock TestVenueMocks {
    venue_mock
        .repo
        .expect_find_venue_by_id()
        .times(1)
        .return_once(move |_| Ok(venue));

    venue_mock
        .repo
        .expect_save_venue()
        .times(1)
        .return_once(move |_| Ok(()));

    venue_mock
}

fn fake_venue_unpublished() -> Venue {
    let time = MockTime {};

    let images = vec![VenueImage {
        title:   "Test Image 1".to_string(),
        uri:     "https://www.test.com/test1.jpg".to_string(),
        comment: Some("Test comment 1".to_string()),
    }];

    VenueBuilder::default()
        .id(Some(fake_number()))
        .name(fake_name())
        .address(fake_address())
        .lessor_id(fake_number())
        .images(images)
        .capacity(fake_number_i32())
        .description(Some("Test venue description".to_string()))
        .status(VenueStatus::Unpublished)
        .createtime(time.now())
        .updatetime(time.now())
        .build()
        .unwrap()
}

fn fake_venue_published() -> Venue {
    let time = MockTime {};

    let images = vec![VenueImage {
        title:   "Test Image 1".to_string(),
        uri:     "https://www.test.com/test1.jpg".to_string(),
        comment: Some("Test comment 1".to_string()),
    }];

    VenueBuilder::default()
        .id(Some(fake_number()))
        .name(fake_name())
        .address(fake_address())
        .lessor_id(fake_number())
        .images(images)
        .capacity(fake_number_i32())
        .description(Some("Test venue description".to_string()))
        .status(VenueStatus::Published)
        .createtime(time.now())
        .updatetime(time.now())
        .build()
        .unwrap()
}

#[tokio::test]
async fn test_publish_venue_success() {
    let mut venue_mock = mock_venue_setup();
    let venue = fake_venue_unpublished();
    let venue_id = venue.id().unwrap();

    let venue_mock = generate_mock_success_publish(&mut venue_mock, venue);

    let repo = &venue_mock.repo;
    let time = MockTime {};

    let res = publish_venue(repo, &time, venue_id).await;

    match res {
        Ok(o) => {
            let data = o.data;
            let event = o.events.get_elements();
            let use_case = o.from_case;

            let test_vec = vec![AppEvent::LogUseCase];
            assert_eq!(test_vec, event, "The event doesn't equal.");
            assert_eq!(use_case, AppUseCase::ManageVenueStatus);
            assert_eq!(data.id, venue_id);
            assert_eq!(data.status, VenueStatusRes::Published);
        },
        Err(e) => panic!("Unexpected error: {}", e),
    }
}

#[tokio::test]
async fn test_unpublish_venue_success() {
    let mut venue_mock = mock_venue_setup();
    let venue = fake_venue_published();
    let venue_id = venue.id().unwrap();

    let venue_mock = generate_mock_success_unpublish(&mut venue_mock, venue);

    let repo = &venue_mock.repo;
    let time = MockTime {};

    let res = unpublish_venue(repo, &time, venue_id).await;

    match res {
        Ok(o) => {
            let data = o.data;
            let event = o.events.get_elements();
            let use_case = o.from_case;

            let test_vec = vec![AppEvent::LogUseCase];
            assert_eq!(test_vec, event, "The event doesn't equal.");
            assert_eq!(use_case, AppUseCase::ManageVenueStatus);
            assert_eq!(data.id, venue_id);
            assert_eq!(data.status, VenueStatusRes::UnPublished);
        },
        Err(e) => panic!("Unexpected error: {}", e),
    }
}
