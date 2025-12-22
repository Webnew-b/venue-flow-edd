use std::ops::Deref;

use actix_multipart::form::{tempfile::TempFile, text::Text, MultipartForm};
use actix_web::{post, web, HttpRequest, HttpResponse};
use app::commands::user_commands::UpdateUserCommand;

use crate::{
    api::{
        upload_image,
        user::{get_user_id, UserClock},
        CustomResponse, CustomResponseError,
    },
    web::app_state::AppState,
};

#[derive(Debug, MultipartForm)]
struct Updation {
    pub username: Text<Option<String>>,
    pub password: Text<Option<String>>,
    pub email: Text<Option<String>>,
    pub gender: Text<Option<String>>,
    pub introduce: Text<Option<String>>,
    #[multipart(limit = "10MB")]
    pub avatar: Option<TempFile>,
}

#[post("/update_user")]
pub async fn update_user(
    MultipartForm(form): MultipartForm<Updation>,
    state: web::Data<AppState>,
    request: HttpRequest,
) -> Result<HttpResponse, CustomResponseError> {
    let id = get_user_id(request)?;
    let temp_path = state.util_service.deref().get_temp_folder();
    tracing::debug!("{:?}", form);
    let save_path = match form.avatar {
        Some(e) => Some(upload_image(temp_path, e)?),
        None => None,
    };
    let update = UpdateUserCommand {
        id,
        username: form.username.0,
        email: form.email.0,
        password: form.password.0,
        avatar: save_path.as_deref(),
        introduce: form.introduce.0,
        gender: form.gender.0,
    };
    let clock = UserClock;
    let res = app::use_case::user::update_user::update_user(
        state.user_service.deref(),
        state.user_service.deref(),
        update,
        state.util_service.deref(),
        state.util_service.deref(),
        &clock,
    )
    .await?;

    let _ = state.event_system.process_outcome(res).await.map_err(|e| {
        tracing::error!("{}", e);
        CustomResponseError::ServiceError
    })?;
    let res = CustomResponse::<()>::success_by_response(None);
    Ok(res)
}
