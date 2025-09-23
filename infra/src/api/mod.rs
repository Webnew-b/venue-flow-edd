use core::fmt;

use actix_web::{HttpResponse, Responder, ResponseError};
use serde::{Deserialize, Serialize};

use crate::api::response_code::{get_code, CodeEnum};

pub mod example;
pub(crate) mod middleware;
pub(crate) mod response_code;

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
