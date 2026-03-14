use actix_web::{middleware::from_fn, web, HttpMessage, HttpRequest, Scope};
use chrono::Utc;
use domain_core::utils::Clock;

use crate::api::{
    middleware::encrypt::{encrypt_middleware, UserAuthRequest},
    CustomResponseError,
};

pub mod get_user_profile;
pub mod login;
pub mod register;
pub mod update_profile;

pub fn index() -> Scope {
    web::scope("/user")
        .service(self::login::login)
        .service(self::register::register)
        .service(
            web::scope("")
                .wrap(from_fn(encrypt_middleware))
                .service(self::update_profile::update_user)
                .service(self::get_user_profile::get_user_profile),
        )
}

pub(super) struct UserClock;

impl Clock for UserClock {
    fn now(&self) -> chrono::DateTime<chrono::Utc> {
        Utc::now()
    }
}

pub(super) fn get_user_id(
    req: HttpRequest,
) -> Result<i64, CustomResponseError> {
    let extensions = req.extensions();
    let identity = extensions.get::<UserAuthRequest>().ok_or(
        CustomResponseError::Unauthorized("Access denied".to_string()),
    )?;
    let id = identity.user_id;
    Ok(id)
}
