use actix_web::{
    body::BoxBody,
    dev::{ServiceRequest, ServiceResponse},
    middleware::{from_fn, Next},
    web, HttpMessage, HttpRequest, Scope,
};
use chrono::Utc;
use domain_core::utils::Clock;

use crate::api::{middleware::encrypt::UserAuthRequest, CustomResponseError};

pub mod cannel_rental_req;
pub mod create_rental_req;
pub mod get_rental_reqs;
pub mod process_rental_req;
pub mod update_rental_time;

pub fn index() -> Scope {
    web::scope("/rental")
        .service(
            web::scope("/organizer")
                .wrap(from_fn(super::middleware::encrypt::encrypt_middleware))
                .wrap(from_fn(rental_organizer_auth))
                .service(self::create_rental_req::create_rental_req)
                .service(self::cannel_rental_req::cancel_rental_request)
                .service(self::update_rental_time::update_rental_time),
        )
        .service(
            web::scope("/lessor")
                .wrap(from_fn(super::middleware::encrypt::encrypt_middleware))
                .wrap(from_fn(rental_lessor_auth))
                .service(self::process_rental_req::approve_rental_request)
                .service(self::get_rental_reqs::get_rental_requests)
                .service(self::process_rental_req::reject_rental_request),
        )
}

async fn rental_organizer_auth(
    req: ServiceRequest,
    next: Next<BoxBody>,
) -> Result<ServiceResponse<BoxBody>, actix_web::Error> {
    let ext = req.extensions();
    let organizer = ext
        .get::<UserAuthRequest>()
        .ok_or(actix_web::error::ErrorBadRequest("Access denied."))?;
    if organizer.organizer_id.is_none() {
        let c_res = super::CustomResponse::<()>::new(
            "Bad request",
            super::response_code::CodeEnum::BadRequest,
            None,
        );
        let http_res = actix_web::HttpResponse::BadRequest().json(c_res);
        let rep = ServiceResponse::new(
            req.request().clone(),
            http_res.map_into_boxed_body(),
        );
        return Ok(rep);
    }
    drop(ext);
    next.call(req).await
}

async fn rental_lessor_auth(
    req: ServiceRequest,
    next: Next<BoxBody>,
) -> Result<ServiceResponse<BoxBody>, actix_web::Error> {
    let ext = req.extensions();
    let lessor_id = ext
        .get::<UserAuthRequest>()
        .ok_or(actix_web::error::ErrorUnauthorized("Access denied."))?;
    if lessor_id.lessor_id.is_none() {
        let c_res = super::CustomResponse::<()>::new(
            "Bad request",
            super::response_code::CodeEnum::Forbidden,
            None,
        );
        let http_res = actix_web::HttpResponse::Forbidden().json(c_res);
        let rep = ServiceResponse::new(
            req.request().clone(),
            http_res.map_into_boxed_body(),
        );
        return Ok(rep);
    }
    drop(ext);
    next.call(req).await
}

pub(super) fn get_organizer_id(
    req: HttpRequest,
) -> Result<i64, CustomResponseError> {
    let extensions = req.extensions();
    let identity = extensions.get::<UserAuthRequest>().ok_or(
        CustomResponseError::Unauthorized("Access denied".to_string()),
    )?;
    let id = identity
        .organizer_id
        .ok_or(CustomResponseError::Forbidden("Access denied".to_string()))?
        .clone();
    Ok(id)
}

pub(super) fn get_lessor_id(
    req: HttpRequest,
) -> Result<i64, CustomResponseError> {
    let extensions = req.extensions();
    let identity = extensions.get::<UserAuthRequest>().ok_or(
        CustomResponseError::Unauthorized("Access denied".to_string()),
    )?;
    let id = identity
        .lessor_id
        .ok_or(CustomResponseError::Forbidden("Access denied".to_string()))?
        .clone();
    Ok(id)
}

pub(super) struct RentalClock;

impl Clock for RentalClock {
    fn now(&self) -> chrono::DateTime<chrono::Utc> {
        Utc::now()
    }
}
