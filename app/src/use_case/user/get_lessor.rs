use domain::user_domain::UserRepository;

use crate::app_error::{AppError, AppResult};
use crate::commands::user_commands::LessorDetail;
use crate::{AppUseCase, Outcome};

pub async fn get_lessor_detail(
    id: i64,
    repo: &impl UserRepository,
) -> AppResult<Outcome<LessorDetail>> {
    let user = repo.find_lessor_by_user_id(id).await?;
    let id = user
        .user()
        .id()
        .ok_or(AppError::IdInexisted("user".to_string()))?;
    let venues = vec!["aaa".to_string()];

    let res = LessorDetail {
        id,
        username: user.user().username().to_string(),
        email: user.user().email().to_string(),
        phone: user.phone().to_string(),
        venues,
    };
    Ok(Outcome::new(
        res,
        AppUseCase::GetData("lessor profile".to_string()),
    ))
}
