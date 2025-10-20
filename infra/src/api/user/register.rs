use std::{
    ops::Deref,
    path::{Path, PathBuf},
};

use actix_multipart::form::{tempfile::TempFile, text::Text, MultipartForm};
use actix_web::{post, web, HttpResponse};
use chrono::Utc;
use domain_core::utils::Clock;
use image::{ImageFormat, ImageReader};
use uuid::Uuid;

use crate::{
    api::{CustomResponse, CustomResponseError},
    infra_error::{InfraError, InfraResult},
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

fn verify_image_type(path: &Path) -> InfraResult<()> {
    if !path.exists() {
        return Err(InfraError::FileNotFound);
    }

    let reader = ImageReader::open(path).map_err(|e| {
        tracing::error!("{}", e.to_string());
        InfraError::FileTypeIsInvalid
    })?;
    let reader = reader.with_guessed_format().map_err(|e| {
        tracing::error!("{}", e.to_string());
        InfraError::FileTypeIsInvalid
    })?;
    let fmt = reader.format();

    match fmt {
        Some(ImageFormat::Png)
        | Some(ImageFormat::Jpeg)
        | Some(ImageFormat::Gif) => Ok(()),
        _ => Err(InfraError::FileTypeIsInvalid),
    }
}

struct UserClock;

impl Clock for UserClock {
    fn now(&self) -> chrono::DateTime<chrono::Utc> {
        Utc::now()
    }
}

#[post("/register")]
pub async fn register(
    state: web::Data<AppState>,
    MultipartForm(form): MultipartForm<Upload>,
) -> Result<HttpResponse, CustomResponseError> {
    let original_name = form.file.file_name.as_deref().unwrap_or("unknown");

    let ext = std::path::Path::new(original_name)
        .extension()
        .and_then(|s| s.to_str())
        .ok_or(CustomResponseError::BadRequest(
            "The file must be a image".to_string(),
        ))?;

    let unique_name = Uuid::new_v4().simple().to_string();

    let save_path: PathBuf = ["./temp", &unique_name, ext].iter().collect();

    verify_image_type(save_path.as_path()).map_err(|e| {
        tracing::error!("{}", e);
        CustomResponseError::BadRequest(
            "The File format must be image".to_string(),
        )
    })?;

    let _ = form.file.file.persist(save_path.as_path()).map_err(|e| {
        tracing::error!("{}", e);
        CustomResponseError::ServiceError
    })?;

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
