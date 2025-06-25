use serde::{Deserialize, Serialize};

pub mod user_domain;
pub mod domain_error;
pub mod venue_domain;

#[derive(Deserialize,Serialize)]
pub struct PageLimit {
    pub page:i64,
    pub limit:i64
}
