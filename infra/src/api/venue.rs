use actix_web::{middleware::from_fn, web, Scope};
use chrono::Utc;
use domain_core::utils::Clock;

use crate::api::middleware::encrypt::encrypt_middleware;

pub mod create_venue;
pub mod get_venue;
pub mod get_venue_by_user;
pub mod get_venue_list;
pub mod manage_venue_status;
pub mod update_venue;

pub fn index() -> Scope {
    web::scope("/venue")
        .service(
            web::scope("")
                .wrap(from_fn(encrypt_middleware))
                .service(self::get_venue_by_user::get_venue_by_user),
        )
        .service(self::get_venue::get_venue)
        .service(self::get_venue_list::get_venue_list)
}

pub(super) struct VenueClock;

impl Clock for VenueClock {
    fn now(&self) -> chrono::DateTime<chrono::Utc> {
        Utc::now()
    }
}
