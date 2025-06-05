use domain::user_domain::{UserRepository, UserValidation};
use domain_core::user::{UserBuilder, UserGender};
use garde::Validate;

use crate::app_error::user_error::AppUserError;
use crate::app_error::AppResult;
use crate::commands::user_commands::{RegisterUserCommand, RegisteredUserDto};



pub async fn register_user(
    repo:&impl UserRepository,
    validator:&impl UserValidation,
    data:RegisterUserCommand,
) -> AppResult<RegisteredUserDto> {

    let builder = UserBuilder::default();

    let email = data.email;
    validator.valid_email(&email).await?;
    let builder = builder.email(email);

    let username = data.username;
    validator.valid_username(&username).await?;
    let builder = builder.username(username);

    let builder = builder.password(data.password)
            .introduce(data.introduce);

    let gender = UserGender::get_gender(data.gender.as_str())
        .ok_or(AppUserError::InvalidGender)?;
    let builder = builder.gender(gender);

    //todo Upload the user avatar to the oss
    let avatar = data.avatar;
    let user = builder.avatar(avatar)
            .build()
            .map_err(|e|{
                AppUserError::CreateUserEntityFailed { 
                    message: e.to_string(), 
                    source: e 
                }
            })?;

    user.validate().map_err(|e|{
        AppUserError::UserIllegal(e.to_string())
    })?;

    let user = repo.create_user(user).await?;

    let id = user.id().ok_or(AppUserError::UserIdInexisted)?;

    let res = RegisteredUserDto {
        id,
        email:user.email().to_string(),
        username:user.username().to_string(),
        avatar:user.avatar().to_string(),
        gender:user.gender().to_string(),
        password:user.password().to_string()
    };
    Ok(res)
}
