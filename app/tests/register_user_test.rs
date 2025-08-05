use std::path::Path;

use app::app_event::AppEvent;
use app::commands::user_commands::RegisterUserCommand;
use app::use_case::user::register_user::register_user;
use app::AppUseCase;
use domain_core::user::UserGender;
use fake::faker::internet::en::{FreeEmail, Password, Username};
use fake::Fake;

use crate::common::user_common::{mock_user_setup, TestUserMocks};
use crate::common::util_common::{mock_utils_setup, MockTime, TestUtilMock};

mod common;

fn generate_mock_success<'test_mock>
    (
        mock:&'test_mock mut TestUserMocks,
    ) 
    -> &'test_mock TestUserMocks 
{
    mock.repo.expect_create_user()
        .times(1)
        .return_once(move |u| {
            let time = MockTime{};
            let user = u.update_id(1,&time);
            Ok(user)
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

#[tokio::test]
async fn test_login_success() {
    let mut test_mock = mock_user_setup();
    let mut util_mock = mock_utils_setup();

    let test_mock = generate_mock_success(&mut test_mock);
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

    let data = RegisterUserCommand{
        password:pwd.clone(),
        username: username.clone(), 
        email: email.clone(),
        avatar: path.as_ref(), 
        gender: "male".to_string(),
        introduce: None 
    };
    let res = register_user(
        repo,
        image_repo,
        validator,
        &time,
        util, 
        data
    ).await;

    let avatar = path.to_str().unwrap().to_string();
    let avatar = format!("https://www.test.com{}",avatar);

    match res {
        Ok(o) => {

            let data = o.data;
            let event = o.events.get_elements();
            let use_case = o.from_case;

            let test_vec = vec![AppEvent::LogUseCase];
            assert_eq!(test_vec,event,"The event doesn't equal.");
            assert_eq!(use_case,AppUseCase::UserRegistrantion);

            assert_eq!(data.id,1i64);
            assert_eq!(data.username,username);
            assert_eq!(data.email,email);
            assert_eq!(data.avatar,avatar);
            assert_eq!(data.gender,UserGender::Male.to_string());
            assert_eq!(data.password,pwd);
        },
        Err(e) => panic!("Unexpect arm:{}",e),
    }
}
