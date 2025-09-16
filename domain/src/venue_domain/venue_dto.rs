use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize)]
pub struct IndexVenue {
    pub lessor_avatar: String,
    pub lessor_id: i64,
    pub venue_name: String,
    pub address: String,
    pub venue_id: i64,
    pub venue_image: String,
}
