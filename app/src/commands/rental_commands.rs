use chrono::{DateTime, Utc};
use domain_core::rental::ActivityType;
use serde::{Deserialize, Serialize};

pub struct CreateRentalCommand {
    pub venue_id: i64,
    pub organizer_id: i64,
    pub start_time: DateTime<Utc>,
    pub end_time: DateTime<Utc>,
    pub activity_type: ActivityType,
    pub request_comments: Option<String>,
}

#[derive(Serialize, Deserialize)]
pub struct CreateRentalRes {
    pub id: i64,
    pub venue_id: i64,
    pub organizer_id: i64,
    pub start_time: String,
    pub end_time: String,
}
