use domain::user_domain::UserRepository;

use crate::app_error::{AppError, AppResult};
use crate::commands::user_commands::UserDetail;
use crate::{AppUseCase, Outcome};

pub async fn get_user_detail(
    id: i64,
    repo: &impl UserRepository,
) -> AppResult<Outcome<UserDetail>> {
    let user = repo.find_user_by_id(id).await?;
    let id = user.id().ok_or(AppError::IdInexisted("user".to_string()))?;
    let is_organizer = repo.find_user_has_organizer_role(id).await?.is_some();
    let is_lessor = repo.find_user_has_lessor_role(id).await?.is_some();

    let res = UserDetail {
        id,
        username: user.username().to_string(),
        email: user.email().to_string(),
        avatar: user.avatar().to_string(),
        introduce: user.introduce().clone(),
        gender: user.gender().to_string(),
        is_organizer,
        is_lessor,
    };
    Ok(Outcome::new(
        res,
        AppUseCase::GetData("user profile".to_string()),
    ))
}
