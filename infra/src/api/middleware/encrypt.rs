use std::ops::Deref;

use actix_web::body::BoxBody;
use actix_web::dev::{ServiceRequest, ServiceResponse};
use actix_web::middleware::{ErrorHandlerResponse, Next};
use actix_web::{web, Error, HttpMessage, HttpRequest, HttpResponse};

use crate::api::response_code::CodeEnum;
use crate::api::CustomResponse;
use crate::web::app_state::AppState;

fn create_bad_request_res(req: HttpRequest) -> ServiceResponse {
    let c_res =
        CustomResponse::<()>::new("Bad request", CodeEnum::BadRequest, None);
    let http_res = HttpResponse::BadRequest().json(c_res);
    ServiceResponse::new(req, http_res.map_into_boxed_body())
}

#[derive(Debug)]
pub(crate) struct UserAuthRequest {
    pub user_id: i64,
    pub lessor_id: Option<i64>,
    pub organizer_id: Option<i64>,
}

pub async fn encrypt_middleware(
    req: ServiceRequest,
    next: Next<BoxBody>,
) -> Result<ServiceResponse<BoxBody>, Error> {
    let state = req.app_data::<web::Data<AppState>>();
    if state.is_none() {
        tracing::error!("App State is missing.");
        return Err(actix_web::error::ErrorInternalServerError(
            "500 Server Error",
        ));
    }
    let state = state.unwrap();

    let header = req.headers();

    let auth = header.get("Authorization");

    match auth {
        Some(e) => {
            let token = e.to_str().map_err(|e| {
                tracing::error!("{}", e);
                actix_web::error::ErrorUnauthorized("Token format is illegal.")
            })?;
            let claims = state
                .user_service
                .deref()
                .decode_token(token)
                .await
                .map_err(|e| {
                tracing::error!("{}", e);
                actix_web::error::ErrorForbidden("Access denied.")
            })?;
            tracing::info!("User id:{} request middleware.", &claims.sub);
            let auth_req = UserAuthRequest {
                user_id: claims
                    .sub
                    .parse::<i64>()
                    .expect("user id is not a number"),
                lessor_id: claims.lessor_id.map(|x| {
                    x.parse::<i64>().expect("lessor id is not a number")
                }),
                organizer_id: claims.organizer_id.map(|x| {
                    x.parse::<i64>().expect("organizer id is not a number")
                }),
            };
            req.extensions_mut().insert(auth_req);
        },
        None => {
            return Ok(create_bad_request_res(req.request().clone()));
            //return Err(actix_web::error::ErrorBadRequest("Access denied."));
        },
    };

    tracing::debug!("you request this middleware.");
    next.call(req).await
}

pub fn add_service_error_handle<B>(
    res: actix_web::dev::ServiceResponse<B>,
) -> actix_web::Result<ErrorHandlerResponse<B>> {
    let c_res =
        CustomResponse::<()>::new("Bad request", CodeEnum::BadRequest, None);
    let http_res = HttpResponse::BadRequest().json(c_res);
    Ok(ErrorHandlerResponse::Response(ServiceResponse::new(
        res.request().clone(),
        http_res.map_into_right_body(),
    )))
}
