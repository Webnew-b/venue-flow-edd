use core::fmt;
use std::path::{Path, PathBuf};

use actix_multipart::form::tempfile::TempFile;
use actix_web::{web, HttpResponse, Responder, ResponseError};
use image::{ImageFormat, ImageReader};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::{
    api::response_code::{get_code, CodeEnum},
    infra_error::{InfraError, InfraResult},
};

pub(crate) mod middleware;
pub mod rental;
pub(crate) mod response_code;
pub mod user;
pub mod venue;

pub fn api_route(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/api")
            .service(self::user::index())
            .service(self::rental::index())
            .service(self::venue::index()),
    );
}

#[derive(Serialize)]
pub struct CustomResponse<T: Serialize + for<'de> Deserialize<'de>> {
    pub message: String,
    pub code: u16,
    pub body: Option<T>,
}

impl<T: Serialize + for<'de> Deserialize<'de>> CustomResponse<T> {
    pub fn new(msg: &str, code: CodeEnum, res: Option<T>) -> Self {
        Self {
            code: get_code(code),
            message: msg.to_string(),
            body: res,
        }
    }
    pub fn success(resp: Option<T>) -> Self {
        Self {
            code: get_code(response_code::CodeEnum::Success),
            message: "Success".to_string(),
            body: resp,
        }
    }

    pub fn success_by_response(resp: Option<T>) -> HttpResponse {
        let res = Self {
            code: get_code(CodeEnum::Success),
            message: "Success".to_string(),
            body: resp,
        };
        HttpResponse::Ok().json(res)
    }
}

#[derive(Debug)]
#[allow(unused)]
pub(crate) enum CustomResponseError {
    NotFound(String),
    BadRequest(String),
    Unauthorized(String),
    Forbidden(String),
    MethodNotAllowed(String),
    Other(String, CodeEnum),
    ServiceError,
}

impl fmt::Display for CustomResponseError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            CustomResponseError::NotFound(s)
            | CustomResponseError::BadRequest(s)
            | CustomResponseError::Unauthorized(s)
            | CustomResponseError::Forbidden(s)
            | CustomResponseError::MethodNotAllowed(s)
            | CustomResponseError::Other(s, _) => {
                write!(f, "{}", s)
            },
            CustomResponseError::ServiceError => write!(f, "Server error"),
        }
    }
}

impl ResponseError for CustomResponseError {
    fn error_response(
        &self,
    ) -> actix_web::HttpResponse<actix_web::body::BoxBody> {
        match self {
            CustomResponseError::ServiceError => {
                let res = CustomResponse::<()>::new(
                    "Server Error",
                    CodeEnum::ServiceError,
                    None,
                );
                HttpResponse::InternalServerError().json(res)
            },
            CustomResponseError::Forbidden(e) => {
                let res =
                    CustomResponse::<()>::new(e, CodeEnum::Forbidden, None);
                HttpResponse::Forbidden().json(res)
            },
            CustomResponseError::Unauthorized(e) => {
                let res =
                    CustomResponse::<()>::new(e, CodeEnum::Unauthorized, None);
                HttpResponse::Unauthorized().json(res)
            },
            CustomResponseError::NotFound(e) => {
                let res =
                    CustomResponse::<()>::new(e, CodeEnum::NotFound, None);
                HttpResponse::NotFound().json(res)
            },
            CustomResponseError::BadRequest(e) => {
                let res =
                    CustomResponse::<()>::new(e, CodeEnum::BadRequest, None);
                HttpResponse::BadRequest().json(res)
            },
            CustomResponseError::MethodNotAllowed(e) => {
                let res = CustomResponse::<()>::new(
                    e,
                    CodeEnum::MethodNotAllowed,
                    None,
                );
                HttpResponse::MethodNotAllowed().json(res)
            },
            CustomResponseError::Other(s, c) => {
                let res = CustomResponse::<()>::new(s, c.clone(), None);
                HttpResponse::Ok().json(res)
            },
        }
    }
}

pub async fn default_service_handle_error() -> impl Responder {
    /*
    if let Some(_) = err.downcast_ref::<ContentTypeError>() {
        let response = CustomResponse::<()>::new(
            "Content-Type missing",
            CodeEnum::BadRequest,
            None
        );
        HttpResponse::BadRequest().json(response)
    } else {
        let response = CustomResponse::<()>::new(
            "Server error",
            CodeEnum::ServiceError,
            None
        );
        HttpResponse::InternalServerError().json(response)
    }
    */
    let response = CustomResponse::<()>::new(
        "Request path is not found",
        CodeEnum::NotFound,
        None,
    );
    HttpResponse::NotFound().json(response)
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

pub(crate) fn upload_image(
    file: TempFile,
) -> Result<PathBuf, CustomResponseError> {
    let original_name = &file.file_name.as_deref().unwrap_or("unknown");

    let ext = std::path::Path::new(original_name)
        .extension()
        .and_then(|s| s.to_str())
        .ok_or(CustomResponseError::BadRequest(
            "The file must be a image".to_string(),
        ))?;

    let unique_name = Uuid::new_v4().simple().to_string();

    std::fs::create_dir_all("./temp/").map_err(|e| {
        tracing::error!("{}", e);
        CustomResponseError::ServiceError
    })?;

    let save_path: PathBuf =
        ["./temp/", &unique_name, ".", ext].iter().collect();

    let _ = file.file.persist(save_path.as_path()).map_err(|e| {
        tracing::error!("{}", e);
        CustomResponseError::ServiceError
    })?;

    verify_image_type(save_path.as_path()).map_err(|e| {
        tracing::error!("{}", e);
        CustomResponseError::BadRequest(
            "The File format must be image".to_string(),
        )
    })?;

    Ok(save_path)
}
