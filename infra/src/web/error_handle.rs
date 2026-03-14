use actix_web::{error::JsonPayloadError, http::StatusCode, web, HttpResponse};

use crate::api::{
    response_code::{get_code, CodeEnum},
    CustomResponse,
};

fn create_bad_request_for_error(message: String) -> HttpResponse {
    let res: CustomResponse<()> = CustomResponse {
        message,
        code: get_code(CodeEnum::BadRequest),
        body: None,
    };
    HttpResponse::build(StatusCode::BAD_REQUEST).json(res)
}

#[allow(unused)]
fn create_forbidden_for_error(message: String) -> HttpResponse {
    let res: CustomResponse<()> = CustomResponse {
        message,
        code: get_code(CodeEnum::Forbidden),
        body: None,
    };
    HttpResponse::build(StatusCode::FORBIDDEN).json(res)
}

pub fn query_error_handle() -> web::QueryConfig {
    web::QueryConfig::default().error_handler(|e, _| {
        let msg = e.to_string();
        actix_web::error::InternalError::from_response(
            e,
            create_bad_request_for_error(msg),
        )
        .into()
    })
}

pub fn path_error_handle() -> web::PathConfig {
    web::PathConfig::default().error_handler(|e, _| {
        let msg = e.to_string();
        actix_web::error::InternalError::from_response(
            e,
            create_bad_request_for_error(msg),
        )
        .into()
    })
}

pub fn form_error_handle() -> web::FormConfig {
    web::FormConfig::default().error_handler(|e, _| {
        let msg = e.to_string();
        actix_web::error::InternalError::from_response(
            e,
            create_bad_request_for_error(msg),
        )
        .into()
    })
}

pub fn json_error_handle() -> web::JsonConfig {
    web::JsonConfig::default().error_handler(|e, _| {
        let msg = match &e {
            JsonPayloadError::Overflow { .. } => {
                "request body too large".to_string()
            },
            JsonPayloadError::Deserialize(err) => err.to_string(),
            JsonPayloadError::ContentType => "invalid content-type".to_string(),
            _ => e.to_string(),
        };

        actix_web::error::InternalError::from_response(
            e,
            create_bad_request_for_error(msg),
        )
        .into()
    })
}
