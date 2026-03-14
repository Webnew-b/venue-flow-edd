use actix_web::{
    body::BoxBody,
    dev::{ServiceRequest, ServiceResponse},
    middleware::{from_fn, Next},
    web, HttpMessage, HttpRequest, HttpResponse, Scope,
};
use chrono::Utc;
use domain_core::utils::Clock;

use crate::api::{
    middleware::encrypt::{encrypt_middleware, UserAuthRequest},
    CustomResponse, CustomResponseError,
};

pub mod create_venue;
pub mod get_venue;
pub mod get_venue_by_user;
pub mod get_venue_list;
pub mod manage_venue_status;
pub mod update_venue;
pub mod upload_venue_image;

pub fn index() -> Scope {
    web::scope("/venue")
        .service(self::get_venue::get_venue)
        .service(self::get_venue_list::get_venue_list)
        .service(
            web::scope("")
                .wrap(from_fn(venue_auth))
                .wrap(from_fn(encrypt_middleware))
                .service(self::get_venue_by_user::get_venue_by_user)
                .service(self::create_venue::create_venue)
                .service(self::update_venue::update_venue)
                .service(self::update_venue::update_venue_image)
                .service(self::update_venue::delete_venue_image)
                .service(self::upload_venue_image::upload_venue_image)
                .service(self::manage_venue_status::publish_venue)
                .service(self::manage_venue_status::unpublish_venue),
        )
}

fn create_error_json(
    msg: &str,
    code: super::response_code::CodeEnum,
) -> String {
    let c_res = CustomResponse::<()>::new(msg, code, None);
    serde_json::to_string(&c_res).unwrap_or_else(|e| {
        tracing::error!("{}", e);
        r#"{"code":"500","message":"serialize failed","data":null}"#.to_string()
    })
}

async fn venue_auth(
    req: ServiceRequest,
    next: Next<BoxBody>,
) -> Result<ServiceResponse<BoxBody>, actix_web::Error> {
    let ext = req.extensions();
    let lessor_id = ext.get::<UserAuthRequest>().ok_or(
        actix_web::error::ErrorForbidden(create_error_json(
            "Access denied.",
            super::response_code::CodeEnum::Forbidden,
        )),
    )?;
    if lessor_id.lessor_id.is_none() {
        let c_res = super::CustomResponse::<()>::new(
            "Forbidden",
            super::response_code::CodeEnum::Forbidden,
            None,
        );
        let http_res = HttpResponse::Forbidden().json(c_res);
        let rep = ServiceResponse::new(
            req.request().clone(),
            http_res.map_into_boxed_body(),
        );
        return Ok(rep);
    }
    drop(ext);
    next.call(req).await
}

pub(super) fn get_lessor_and_user_id(
    req: HttpRequest,
) -> Result<(i64, i64), CustomResponseError> {
    let extensions = req.extensions();
    let identity = extensions.get::<UserAuthRequest>().ok_or(
        CustomResponseError::Unauthorized("Access denied".to_string()),
    )?;
    let id = identity
        .lessor_id
        .ok_or(CustomResponseError::Forbidden("Access denied".to_string()))?
        .clone();
    Ok((identity.user_id.clone(), id))
}

pub(super) struct VenueClock;

impl Clock for VenueClock {
    fn now(&self) -> chrono::DateTime<chrono::Utc> {
        Utc::now()
    }
}
