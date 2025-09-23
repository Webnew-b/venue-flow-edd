use domain::user_domain::{UserRepository, UserValidation};
use domain::util_trait::{ImageRepository, PasswordHasher};
use domain_core::user::user_update::UserUpdate;
use domain_core::user::UserGender;
use domain_core::utils::Clock;

use crate::app_error::user_error::AppUserError;
use crate::app_error::{AppError, AppResult};
use crate::commands::user_commands::UpdateUserCommand;
use crate::{AppUseCase, Outcome};

async fn get_update_struct<'image>(
    update: UpdateUserCommand<'image>,
    validator: &impl UserValidation,
    image_repo: &impl ImageRepository,
    password_hasher: &impl PasswordHasher,
) -> AppResult<UserUpdate> {
    let mut update_struct = UserUpdate::new();

    if let Some(g) = update.gender {
        let gender = UserGender::get_gender(g.as_str())
            .ok_or(AppUserError::InvalidGender)?;
        update_struct.gender = Some(gender);
    };

    if let Some(a) = update.avatar {
        let image = image_repo.upload_image(a).await?;
        update_struct.avatar = Some(image);
    }

    if let Some(u) = update.username {
        validator.valid_username(u.as_str()).await?;
        update_struct.username = Some(u);
    }

    if let Some(e) = update.email {
        //todo verify email and send event
        validator.valid_email(e.as_str()).await?;
        update_struct.email = Some(e);
    }

    if let Some(p) = update.password {
        let password = password_hasher.hash(p.as_str())?;
        update_struct.password = Some(password);
    }
    update_struct.introduce = update.introduce;

    update_struct
        .valid_update()
        .map_err(|e| AppError::UpdateEntityFailed {
            entity_type: "user".to_string(),
            message: e.to_string(),
            source: e,
        })?;
    Ok(update_struct)
}

pub async fn update_user<'image>(
    repo: &impl UserRepository,
    validator: &impl UserValidation,
    update: UpdateUserCommand<'image>,
    image_repo: &impl ImageRepository,
    password_hasher: &impl PasswordHasher,
    clock: &impl Clock,
) -> AppResult<Outcome<()>> {
    let id = update.id;
    let update_struct =
        get_update_struct(update, validator, image_repo, password_hasher)
            .await?;
    let user = repo.find_user_by_id(id).await?;
    let user = user.update_user(update_struct, clock).map_err(|e| {
        AppError::UpdateEntityFailed {
            entity_type: "user".to_string(),
            message: e.to_string(),
            source: e,
        }
    })?;
    repo.save_user(user).await?;
    Ok(Outcome::new((), AppUseCase::BasicUserProfile))
}
