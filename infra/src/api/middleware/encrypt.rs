use std::ops::Deref;

use actix_web::body::BoxBody;
use actix_web::dev::{ServiceRequest, ServiceResponse};
use actix_web::middleware::{ErrorHandlerResponse, Next};
use actix_web::{web, Error, HttpMessage};

use crate::api::response_code::CodeEnum;
use crate::api::CustomResponse;
use crate::web::app_state::AppState;

fn create_error_json(msg: &str, code: CodeEnum) -> String {
    let c_res = CustomResponse::<()>::new(msg, code, None);
    serde_json::to_string(&c_res).unwrap_or_else(|e| {
        tracing::error!("{}", e);
        r#"{"code":"500","message":"serialize failed","data":null}"#.to_string()
    })
}

#[derive(Debug)]
pub(crate) struct UserAuthRequest {
    pub user_id:      i64,
    pub lessor_id:    Option<i64>,
    pub organizer_id: Option<i64>,
}

fn option_string2i64(s: Option<String>) -> Result<Option<i64>, Error> {
    s.map(|s| {
        s.parse::<i64>().map_err(|e| {
            tracing::error!("{}", e);
            actix_web::error::ErrorUnauthorized(create_error_json(
                "Token format is illegal.",
                CodeEnum::Unauthorized,
            ))
        })
    })
    .transpose()
}

// TODO: Refactor this middleware into three separate concerns: Token, Auth,
// and Identify. Extract the authentication logic into a dedicated service
// to decouple it from the user service.
pub async fn encrypt_middleware(
    req: ServiceRequest,
    next: Next<BoxBody>,
) -> Result<ServiceResponse<BoxBody>, Error> {
    let state = req.app_data::<web::Data<AppState>>();
    if state.is_none() {
        tracing::error!("App State is missing.");
        return Err(actix_web::error::ErrorInternalServerError(
            create_error_json("500 Server Error", CodeEnum::ServiceError),
        ));
    }
    let state = state.unwrap();

    let header = req.headers();

    let auth = header.get("Authorization");

    match auth {
        Some(e) => {
            let token = e.to_str().map_err(|e| {
                tracing::error!("{}", e);
                actix_web::error::ErrorUnauthorized(create_error_json(
                    "Token format is illegal.",
                    CodeEnum::Unauthorized,
                ))
            })?;
            let claims = state
                .user_service
                .deref()
                .decode_token(token)
                .await
                .map_err(|e| {
                tracing::error!("{}", e);
                actix_web::error::ErrorForbidden(create_error_json(
                    "Access denied.",
                    CodeEnum::Forbidden,
                ))
            })?;
            tracing::info!("User id:{} request middleware.", &claims.sub);
            let auth_req = UserAuthRequest {
                user_id:      claims.sub.parse::<i64>().map_err(|e| {
                    tracing::error!("{}", e);
                    actix_web::error::ErrorUnauthorized(create_error_json(
                        "Token format is illegal.",
                        CodeEnum::Unauthorized,
                    ))
                })?,
                lessor_id:    option_string2i64(claims.lessor_id)?,
                organizer_id: option_string2i64(claims.organizer_id)?,
            };
            req.extensions_mut().insert(auth_req);
        },
        None => {
            return Err(actix_web::error::ErrorForbidden(create_error_json(
                "Access denied.",
                CodeEnum::Forbidden,
            )));
        },
    };

    tracing::debug!("request this middleware:encrypt_middleware");
    next.call(req).await
}

pub fn add_service_error_handle<B>(
    res: actix_web::dev::ServiceResponse<B>,
) -> actix_web::Result<ErrorHandlerResponse<B>> {
    Ok(ErrorHandlerResponse::Response(res.map_into_left_body()))
}
