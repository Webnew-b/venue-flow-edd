use actix_web::{web, Scope};
use chrono::Utc;
use domain_core::utils::Clock;

pub mod cannel_rental_req;
pub mod create_rental_req;
pub mod get_rental_reqs;
pub mod process_rental_req;
pub mod update_rental_time;

pub fn index() -> Scope {
    web::scope("/rental")
        .service(self::create_rental_req::create_rental_req)
        .service(self::process_rental_req::approve_rental_request)
        .service(self::process_rental_req::reject_rental_request)
        .service(self::cannel_rental_req::cancel_rental_request)
        .service(self::get_rental_reqs::get_rental_requests)
        .service(self::update_rental_time::update_rental_time)
}

pub(super) struct RentalClock;

impl Clock for RentalClock {
    fn now(&self) -> chrono::DateTime<chrono::Utc> {
        Utc::now()
    }
}
