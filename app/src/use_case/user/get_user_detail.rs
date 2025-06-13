use domain::user_domain::UserRepository;

use crate::app_error::user_error::AppUserError;
use crate::app_error::AppResult;
use crate::commands::user_commands::UserDetail;
use crate::{AppUseCase, Outcome};

pub async fn get_user_detail(
    id:i64,
    repo:&impl UserRepository,
)->AppResult<Outcome<UserDetail>> {
    let user = repo.find_user_by_id(id).await?;
    let id = user.id().ok_or(AppUserError::UserIdInexisted)?;
    //todo check the character(lessor ,ozgnaizer or both).
    let res = UserDetail{
        id,
        username:user.username().to_string(),
        email:user.email().to_string(),
        avatar:user.avatar().to_string(),
        introduce:user.introduce().cloned(),
        gender:user.gender().to_string(),
    };
    Ok(Outcome::new(res, AppUseCase::GetData("user profile".to_string())))
}
