use app::app_event::AppEvent;
use app::commands::user_commands::{Email, LoginUserCommand, UserLoginType};
use app::use_case::user::login::login_user;
use app::AppUseCase;
use domain::user_domain::user_dto::UserLoginToken;
use domain_core::user::{User, UserBuilder, UserGender};
use domain_core::utils::Clock;
use fake::faker::internet::en::{FreeEmail, Password, Username};
use fake::Fake;

use crate::common::user_common::{mock_user_setup, TestUserMocks};
use crate::common::util_common::{mock_utils_setup, MockTime, TestUtilMock};

mod common;

fn generate_mock_success<'test_mock>(
    mock: &'test_mock mut TestUserMocks,
    user: User,
) -> &'test_mock TestUserMocks {
    mock.repo
        .expect_find_user_by_name()
        .times(1)
        .return_once(move |_| Ok(user));

    mock.generator.expect_generate_token().returning(|_| {
        Ok(UserLoginToken {
            token: "test_token".to_string(),
        })
    });

    mock.validator
        .expect_exist_email()
        .times(1)
        .returning(|_| Ok(()));

    mock
}

fn create_fake_util<'test_mock>(
    mock: &'test_mock mut TestUtilMock,
) -> &'test_mock TestUtilMock {
    mock.password_hash.expect_verify().returning(|_, _| Ok(()));
    mock
}

fn create_fake_user() -> User {
    let time = MockTime {};

    UserBuilder::default()
        .id(Some(1))
        .username(Username().fake::<String>())
        .email(FreeEmail().fake::<String>())
        .password(Password(6..30).fake::<String>())
        .avatar("wudawd".to_string())
        .gender(UserGender::Male)
        .createtime(time.now())
        .updatetime(time.now())
        .build()
        .unwrap()
}

#[tokio::test]
async fn test_login_success() {
    let mut test_mock = mock_user_setup();
    let mut util_mock = mock_utils_setup();
    let user = create_fake_user();
    let test_mock = generate_mock_success(&mut test_mock, user.clone());
    let util_mock = create_fake_util(&mut util_mock);

    let repo = &test_mock.repo;
    let generator = &test_mock.generator;
    let validator = &test_mock.validator;
    let util = &util_mock.password_hash;
    let info = LoginUserCommand {
        login_type: UserLoginType::Email(Email {
            address: user.email().to_string(),
        }),
        password: user.password().to_string(),
    };
    let res = login_user(repo, validator, generator, util, info).await;
    match res {
        Ok(o) => {
            let id = user.id().unwrap();
            let data = o.data;
            let event = o.events.get_elements();
            let use_case = o.from_case;
            let test_vec = vec![AppEvent::LogUseCase];
            assert_eq!(test_vec, event, "The event doesn't equal.");
            assert_eq!(use_case, AppUseCase::UserLogin);
            assert_eq!(data.id, id);
            assert_eq!(data.username, user.username().to_string());
            assert_eq!(data.token, "test_token".to_string());
        },
        Err(e) => panic!("Unexpect arm:{}", e),
    }
}
