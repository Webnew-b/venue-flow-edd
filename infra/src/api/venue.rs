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
pub mod upload_venue_image;

pub fn index() -> Scope {
    web::scope("/venue")
        .service(
            web::scope("")
                .wrap(from_fn(encrypt_middleware))
                .service(self::get_venue_by_user::get_venue_by_user)
                .service(self::create_venue::create_venue)
                .service(self::update_venue::update_venue)
                .service(self::upload_venue_image::upload_venue_image),
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
