use actix_web::{middleware::from_fn, web, Scope};
use chrono::Utc;
use domain_core::utils::Clock;

use crate::api::middleware::encrypt::encrypt_middleware;

pub mod get_user_profile;
pub mod login;
pub mod logout;
pub mod register;
pub mod register_lessor;
pub mod register_organizer;
pub mod update_profile;

pub fn index() -> Scope {
    web::scope("/user")
        .service(self::login::login)
        .service(self::register::register)
        .service(
            web::scope("")
                .wrap(from_fn(encrypt_middleware))
                .service(self::update_profile::update_user),
        )
        .service(
            web::scope("")
                .wrap(from_fn(encrypt_middleware))
                .service(self::get_user_profile::get_user_profile),
        )
}

pub(super) struct UserClock;

impl Clock for UserClock {
    fn now(&self) -> chrono::DateTime<chrono::Utc> {
        Utc::now()
    }
}
