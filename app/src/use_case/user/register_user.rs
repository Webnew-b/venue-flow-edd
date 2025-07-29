use domain::user_domain::{UserRepository, UserValidation};
use domain::util_trait::{ImageRepository, PasswordHasher};
use domain_core::user::{UserBuilder, UserGender};
use domain_core::utils::Clock;
use garde::Validate;

use crate::app_error::user_error::AppUserError;
use crate::app_error::{AppError, AppResult};
use crate::commands::user_commands::{RegisterUserCommand, RegisteredUserDto};
use crate::{AppUseCase, Outcome};



pub async fn register_user<'image>(
    repo:&impl UserRepository,
    image_repo:&impl ImageRepository,
    validator:&impl UserValidation,
    time:&impl Clock,
    password_hasher:&impl PasswordHasher,
    data:RegisterUserCommand<'image>,
) -> AppResult<Outcome<RegisteredUserDto>> {

    let builder = UserBuilder::default();

    let email = data.email;
    validator.valid_email(&email).await?;
    let builder = builder.email(email);

    let username = data.username;
    validator.valid_username(&username).await?;
    let builder = builder.username(username);

    let password = password_hasher.hash(&data.password)?;
    let builder = builder.password(password)
            .introduce(data.introduce);

    let gender = UserGender::get_gender(data.gender.as_str())
        .ok_or(AppUserError::InvalidGender)?;
    let builder = builder.gender(gender);

    let avatar = image_repo.upload_image(data.avatar).await?;
    let user = builder.avatar(avatar)
            .updatetime(time.now())
            .createtime(time.now())
            .build()
            .map_err(|e|{
                AppError::CreateEntityFailed { 
                    entity_type:"user".to_string(),
                    message: e.to_string(), 
                    source: e 
                }
            })?;

    user.validate().map_err(|e|{
        AppError::EntityInvalid { 
            entity_type: "user".to_string(),
            cause: e.to_string()
        }
    })?;

    let user = repo.create_user(user).await?;

    let id = user.id().ok_or(AppError::IdInexisted("user".to_string()))?;

    let res = RegisteredUserDto {
        id,
        email:user.email().to_string(),
        username:user.username().to_string(),
        avatar:user.avatar().to_string(),
        gender:user.gender().to_string(),
        password:user.password().to_string()
    };

    Ok(Outcome::new(res,AppUseCase::UserRegistrantion))
}
