use serde::{Deserialize, Serialize};

use crate::commands::venue_commands::VenueImageRes;

#[derive(Serialize, Deserialize, Clone)]
pub struct GetVenueRes {
    pub id:            i64,
    pub lessor_id:     i64,
    pub lessor_phone:  String,
    pub lessor_name:   String,
    pub lessor_avatar: String,
    pub name:          String,
    pub address:       String,
    pub images:        Vec<VenueImageRes>,
    pub capacity:      i32,
    pub description:   String,
}
