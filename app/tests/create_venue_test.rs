use std::path::Path;

use app::app_event::AppEvent;
use app::commands::venue_commands::{CreateVenueCommand, VenueImageCommand};
use app::use_case::venue::create_venue::create_venue;
use app::AppUseCase;
use domain_core::user::lessor::{Lessor, LessorBuilder};
use domain_core::user::{User, UserBuilder, UserGender};
use domain_core::utils::Clock;
use domain_core::venue::venue_image::VenueImage;
use domain_core::venue::{Venue, VenueBuilder};

use crate::common::user_common::{mock_user_setup, TestUserMocks};
use crate::common::util_common::{mock_utils_setup, MockTime, TestUtilMock};
use crate::common::venue_common::{mock_venue_setup, TestVenueMocks};
use crate::common::{
    fake_address, fake_email, fake_name, fake_number, fake_number_i32,
    fake_username,
};

mod common;

fn generate_mock_success<'test_mock>(
    user_mock: &'test_mock mut TestUserMocks,
    venue_mock: &'test_mock mut TestVenueMocks,
    lessor: Lessor,
    venue: Venue,
) -> (&'test_mock TestUserMocks, &'test_mock TestVenueMocks) {
    user_mock
        .repo
        .expect_find_lessor_by_user_id()
        .times(1)
        .return_once(move |_| Ok(lessor));

    venue_mock
        .repo
        .expect_create_venue()
        .times(1)
        .return_once(move |_| Ok(venue));

    (user_mock, venue_mock)
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

fn fake_user_for_lessor() -> User {
    let time = MockTime {};

    UserBuilder::default()
        .id(Some(1))
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

fn fake_lessor() -> Lessor {
    let time = MockTime {};

    LessorBuilder::default()
        .id(Some(1))
        .user(fake_user_for_lessor())
        .phone("12345678901".to_string())
        .createtime(time.now())
        .updatetime(time.now())
        .build()
        .unwrap()
}

fn fake_venue() -> Venue {
    let time = MockTime {};

    let images = vec![
        VenueImage {
            title:   "Test Image 1".to_string(),
            uri:     "https://www.test.com/test1.jpg".to_string(),
            comment: Some("Test comment 1".to_string()),
        },
        VenueImage {
            title:   "Test Image 2".to_string(),
            uri:     "https://www.test.com/test2.jpg".to_string(),
            comment: None,
        },
    ];

    VenueBuilder::default()
        .id(Some(1))
        .name(fake_username())
        .address(fake_address())
        .lessor_id(fake_number())
        .images(images)
        .capacity(fake_number_i32())
        .description(Some("Test venue description".to_string()))
        .createtime(time.now())
        .updatetime(time.now())
        .build()
        .unwrap()
}

#[tokio::test]
async fn test_create_venue_success() {
    let mut user_mock = mock_user_setup();
    let mut venue_mock = mock_venue_setup();
    let mut util_mock = mock_utils_setup();

    let lessor = fake_lessor();
    let venue = fake_venue();

    let (user_mock, venue_mock) = generate_mock_success(
        &mut user_mock,
        &mut venue_mock,
        lessor.clone(),
        venue.clone(),
    );
    let util_mock = create_fake_util(&mut util_mock);

    let user_repo = &user_mock.repo;
    let venue_repo = &venue_mock.repo;
    let image_repo = &util_mock.image_repo;
    let time = MockTime {};

    let path1 = Path::new("/test1.jpg");
    let path2 = Path::new("/test2.jpg");

    let venue_images = vec![
        VenueImageCommand {
            title:   "Test Image 1".to_string(),
            image:   path1,
            comment: Some("Test comment 1".to_string()),
        },
        VenueImageCommand {
            title:   "Test Image 2".to_string(),
            image:   path2,
            comment: None,
        },
    ];

    let create_command = CreateVenueCommand {
        user_id:     1,
        name:        venue.name().to_string(),
        address:     venue.address().to_string(),
        images:      venue_images,
        capacity:    venue.capacity().clone(),
        description: venue.description().clone(),
    };

    let res =
        create_venue(user_repo, venue_repo, image_repo, create_command, &time)
            .await;

    match res {
        Ok(o) => {
            let data = o.data;
            let event = o.events.get_elements();
            let use_case = o.from_case;

            let test_vec = vec![AppEvent::LogUseCase];
            assert_eq!(test_vec, event, "The event doesn't equal.");
            assert_eq!(use_case, AppUseCase::CreateVenue);

            assert_eq!(data.id, 1i64);
            assert_eq!(data.name, venue.name().to_string());
            assert_eq!(data.address, venue.address().to_string());
            assert_eq!(data.capacity, venue.capacity().clone());
            assert_eq!(data.description, venue.description().clone());
            assert_eq!(data.images.len(), 2);
            assert_eq!(data.images[0].title, "Test Image 1");
            assert_eq!(data.images[0].uri, "https://www.test.com/test1.jpg");
            assert_eq!(data.images[1].title, "Test Image 2");
            assert_eq!(data.images[1].uri, "https://www.test.com/test2.jpg");
        },
        Err(e) => panic!("Unexpected error: {}", e),
    }
}
