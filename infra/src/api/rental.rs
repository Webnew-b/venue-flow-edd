use actix_web::{middleware::from_fn, web, Scope};
use chrono::Utc;
use domain_core::utils::Clock;

use crate::api::middleware::encrypt::encrypt_middleware;

pub mod cannel_rental_req;
pub mod create_rental_req;
pub mod get_rental_reqs;
pub mod process_rental_req;
pub mod view_rental_req;

pub fn index() -> Scope {
    web::scope("/rental").service(self::create_rental_req::create_rental_req)
}

pub(super) struct RentalClock;

impl Clock for RentalClock {
    fn now(&self) -> chrono::DateTime<chrono::Utc> {
        Utc::now()
    }
}
