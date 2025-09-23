use std::path::Path;

use app::app_event::AppEvent;
use app::commands::venue_commands::{UpdateVenueCommand, VenueImageCommand};
use app::use_case::venue::update_venue::update_venue;
use app::AppUseCase;
use domain_core::utils::Clock;
use domain_core::venue::venue_image::VenueImage;
use domain_core::venue::{Venue, VenueBuilder};

use crate::common::util_common::{mock_utils_setup, MockTime, TestUtilMock};
use crate::common::venue_common::{mock_venue_setup, TestVenueMocks};
use crate::common::{fake_address, fake_name, fake_number, fake_number_i32};

mod common;

fn generate_mock_success<'test_mock>(
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

fn create_fake_util<'test_mock>(
    mock: &'test_mock mut TestUtilMock,
) -> &'test_mock TestUtilMock {
    mock.image_repo.expect_upload_image().returning(|e| {
        let url = e.to_str().unwrap().to_string();
        let url = format!("https://www.test.com{}", url);
        Ok(url)
    });
    mock
}

fn fake_venue() -> Venue {
    let time = MockTime {};

    let images = vec![VenueImage {
        title: "Original Image".to_string(),
        uri: "https://www.test.com/original.jpg".to_string(),
        comment: Some("Original comment".to_string()),
    }];

    VenueBuilder::default()
        .id(Some(fake_number()))
        .name(fake_name())
        .address(fake_address())
        .lessor_id(fake_number())
        .images(images)
        .capacity(fake_number_i32())
        .description(Some("Original venue description".to_string()))
        .createtime(time.now())
        .updatetime(time.now())
        .build()
        .unwrap()
}

#[tokio::test]
async fn test_update_venue_success() {
    let mut venue_mock = mock_venue_setup();
    let mut util_mock = mock_utils_setup();

    let venue = fake_venue();
    let venue_id = venue.id().unwrap();

    let venue_mock = generate_mock_success(&mut venue_mock, venue);
    let util_mock = create_fake_util(&mut util_mock);

    let repo = &venue_mock.repo;
    let image_repo = &util_mock.image_repo;
    let time = MockTime {};

    let path1 = Path::new("/updated1.jpg");
    let path2 = Path::new("/updated2.jpg");

    let venue_images = vec![
        VenueImageCommand {
            title: "Updated Image 1".to_string(),
            image: path1,
            comment: Some("Updated comment 1".to_string()),
        },
        VenueImageCommand {
            title: "Updated Image 2".to_string(),
            image: path2,
            comment: None,
        },
    ];

    let update_command = UpdateVenueCommand {
        id: venue_id,
        name: Some("Updated Venue Name".to_string()),
        address: Some("Updated Address".to_string()),
        images: venue_images,
        capacity: Some(200),
        description: Some("Updated venue description".to_string()),
    };

    let res = update_venue(repo, image_repo, update_command, &time).await;

    match res {
        Ok(o) => {
            let event = o.events.get_elements();
            let use_case = o.from_case;

            let test_vec = vec![AppEvent::LogUseCase];
            assert_eq!(test_vec, event, "The event doesn't equal.");
            assert_eq!(use_case, AppUseCase::BasicUserProfile);
        },
        Err(e) => panic!("Unexpected error: {}", e),
    }
}
