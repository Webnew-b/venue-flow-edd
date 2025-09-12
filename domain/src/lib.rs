use serde::{Deserialize, Serialize};

pub mod domain_error;
pub mod event_trait;
pub mod rental_domain;
pub mod user_domain;
pub mod util_trait;
pub mod venue_domain;

#[derive(Deserialize, Serialize)]
pub struct PageLimit {
    pub page: u64,
    pub limit: u64,
}
