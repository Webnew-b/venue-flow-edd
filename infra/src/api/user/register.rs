use std::ops::Deref;

use actix_multipart::form::{tempfile::TempFile, text::Text, MultipartForm};
use actix_web::{post, web, HttpResponse};

use crate::{
    api::{
        user::{upload_image, UserClock},
        CustomResponse, CustomResponseError,
    },
    web::app_state::AppState,
};

#[derive(Debug, MultipartForm)]
struct Upload {
    pub username: Text<String>,
    pub email: Text<String>,
    pub password: Text<String>,
    pub gender: Text<String>,
    pub introduce: Text<Option<String>>,
    #[multipart(limit = "10MB")]
    pub file: TempFile,
}

#[post("/register")]
pub async fn register(
    state: web::Data<AppState>,
    MultipartForm(form): MultipartForm<Upload>,
) -> Result<HttpResponse, CustomResponseError> {
    let save_path = upload_image(form.file)?;

    let register_data = app::commands::user_commands::RegisterUserCommand {
        username: form.username.0,
        email: form.email.0,
        avatar: save_path.as_path(),
        gender: form.gender.0,
        password: form.password.0,
        introduce: form.introduce.0,
    };

    let clock = UserClock;

    let res = app::use_case::user::register_user::register_user(
        state.user_service.deref(),
        state.util_service.deref(),
        state.user_service.deref(),
        &clock,
        state.util_service.deref(),
        register_data,
    )
    .await
    .map_err(|e| {
        tracing::error!("{}", e);
        CustomResponseError::BadRequest(e.to_string())
    })?;

    let res = state.event_system.process_outcome(res).await.map_err(|e| {
        tracing::error!("{}", e);
        CustomResponseError::ServiceError
    })?;
    let res = CustomResponse::success_by_response(Some(res));
    Ok(res)
}
