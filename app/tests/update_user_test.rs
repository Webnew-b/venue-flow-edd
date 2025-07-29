use std::path::Path;

use app::app_event::AppEvent;
use app::commands::user_commands::UpdateUserCommand;
use app::use_case::user::update_user::update_user;
use app::AppUseCase;
use domain_core::user::{User, UserBuilder, UserGender};
use domain_core::utils::Clock;
use fake::faker::internet::en::{FreeEmail, Password, Username};
use fake::Fake;

use crate::common::user_common::{mock_user_setup, TestUserMocks};
use crate::common::util_common::{util_mock_setup, MockTime, TestUtilMock};

mod common;

fn generate_mock_success<'test_mock>
    (
        mock:&'test_mock mut TestUserMocks,
        user:User
    ) 
    -> &'test_mock TestUserMocks 
{

    let origin_user = user;
    mock.repo.expect_find_user_by_id()
        .times(1)
        .return_once(move |_|{
            Ok(origin_user)
        });

    mock.repo.expect_save_user()
        .times(1)
        .return_once(move |_| {
            Ok(())
        });

    mock.validator.expect_valid_email()
        .times(1)
        .returning(|_|{Ok(())});

    mock.validator.expect_valid_username()
        .times(1)
        .returning(|_|{Ok(())});
    mock
}

fn create_fake_util<'test_mock>(
        mock:&'test_mock mut TestUtilMock
    ) -> &'test_mock TestUtilMock{
    mock.password_hash
        .expect_hash()
        .returning(|s| Ok(s.to_string()));

    mock.image_repo
        .expect_upload_image()
        .returning(|e|{
            let url = e.to_str().unwrap().to_string();
            let url = format!("https://www.test.com{}",url);
            Ok(url)
        });
    mock
}

fn create_fake_user() -> User {
    let time = MockTime{};
    
    UserBuilder::default()
        .id(Some(1))
        .username(Username().fake::<String>())
        .email(FreeEmail().fake::<String>())
        .password(Password(6..30).fake::<String>())
        .avatar("wudawd".to_string())
        .gender(UserGender::Male)
        .createtime(time.now())
        .updatetime(time.now())
        .build().unwrap()
}

#[tokio::test]
async fn test_login_success() {
    let mut test_mock = mock_user_setup();
    let mut util_mock = util_mock_setup();
    let user = create_fake_user();

    let test_mock = generate_mock_success(&mut test_mock,user);
    let util_mock = create_fake_util(&mut util_mock);

    let repo = &test_mock.repo;
    let time = MockTime{};
    let validator = &test_mock.validator;
    let util = &util_mock.password_hash;
    let image_repo = &util_mock.image_repo;
    
    let path = Path::new("/aa/aaww.png");
    let pwd = Password(6..30).fake::<String>();
    let email = FreeEmail().fake::<String>();
    let username = Username().fake::<String>();

    let data = UpdateUserCommand{
        id:1,
        password:Some(pwd.clone()),
        username: Some(username.clone()), 
        email: Some(email.clone()),
        avatar: Some(path.as_ref()), 
        gender: Some("male".to_string()),
        introduce: None 
    };
    let res = update_user(
        repo,
        validator,
        data,
        image_repo,
        util,
        &time,
    ).await;

    match res {
        Ok(o) => {

            let event = o.events.get_elements();
            let use_case = o.from_case;

            let test_vec = vec![AppEvent::LogUseCase];
            assert_eq!(test_vec,event,"The event doesn't equal.");
            assert_eq!(use_case,AppUseCase::BasicUserProfile);
        },
        Err(e) => panic!("Unexpect arm:{}",e),
    }
}
