use std::path::{Path, PathBuf};

use actix_multipart::form::tempfile::TempFile;
use actix_web::{web, Scope};
use chrono::Utc;
use domain_core::utils::Clock;
use image::{ImageFormat, ImageReader};
use uuid::Uuid;

use crate::{
    api::CustomResponseError,
    infra_error::{InfraError, InfraResult},
};

pub mod login;
pub mod logout;
pub mod register;
pub mod register_lessor;
pub mod register_organizer;
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

pub fn index() -> Scope {
    web::scope("/user")
        .service(self::login::login)
        .service(self::register::register)
}

pub(super) struct UserClock;

impl Clock for UserClock {
    fn now(&self) -> chrono::DateTime<chrono::Utc> {
        Utc::now()
    }
}

pub(super) fn upload_image(
    file: &TempFile,
) -> Result<PathBuf, CustomResponseError> {
    let original_name = file.file_name.as_deref().unwrap_or("unknown");

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

    let _ = file.file.persist(save_path.as_path()).map_err(|e| {
        tracing::error!("{}", e);
        CustomResponseError::ServiceError
    })?;
    Ok(save_path)
}
