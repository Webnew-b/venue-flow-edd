use domain::user_domain::{ UserRepository, UserValidation};
use domain_core::user::user_update::UserUpdate;
use domain_core::user::UserGender;

use crate::app_error::user_error::AppUserError;
use crate::app_error::AppResult;
use crate::commands::user_commands::UpdateUserCommand;
use crate::{AppUseCase, Outcome};

async fn get_update_struct(
    update:UpdateUserCommand,
    validator:&impl UserValidation,
    )
    -> AppResult<UserUpdate> {
    
    let mut update_struct = UserUpdate::new();

    if let Some(g) = update.gender {
        let gender = UserGender::get_gender(g.as_str())
            .ok_or(AppUserError::InvalidGender)?;
        update_struct.gender = Some(gender);
    };

    if let Some(a) = update.avatar {
        //todo update file to the oss
        update_struct.avatar = Some(a);
    }

    if let Some(u) = update.username{
        validator.valid_username(u.as_str()).await?;
        update_struct.username = Some(u);
    }

    if let Some(e) = update.email{
        //todo verify email and send event
        validator.valid_email(e.as_str()).await?;
        update_struct.email = Some(e);
    }
    update_struct.password = update.password;
    update_struct.introduce = update.introduce;

    update_struct.valid_update().map_err(|e|{
        AppUserError::UpdateUserEntityFailed { message: e.to_string(), source:e }
    })?;
    Ok(update_struct)
}

pub async fn update_user(
    repo:&impl UserRepository,
    validator:&impl UserValidation,
    update:UpdateUserCommand,
    )-> AppResult<Outcome<()>> {
    let id = update.id;
    let update_struct = get_update_struct(update,validator).await?;
    let user = repo.find_user_by_id(id).await?;
    let user = user.update_user(update_struct).map_err(|e|{
        AppUserError::UpdateUserEntityFailed { message: e.to_string(), source:e }
    })?;
    repo.save_user(user).await?;
    Ok(Outcome::new((),AppUseCase::BasicUserProfile))
}
