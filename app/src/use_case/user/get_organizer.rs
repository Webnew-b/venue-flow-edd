use domain::user_domain::UserRepository;

use crate::app_error::{AppError, AppResult};
use crate::commands::user_commands::OrganizerDetail;
use crate::{AppUseCase, Outcome};


pub async fn get_user_detail(
    id:i64,
    repo:&impl UserRepository,
)->AppResult<Outcome<OrganizerDetail>> {
    let user = repo.find_organizer_by_user_id(id).await?;
    let id = user.user().id().ok_or(AppError::IdInexisted("user".to_string()))?;

    let res = OrganizerDetail{
        id,
        username:user.user().username().to_string(),
        email:user.user().email().to_string(),
        phone:user.phone().to_string(),
    };
    Ok(Outcome::new(res, AppUseCase::GetData("organizer profile".to_string())))
}
