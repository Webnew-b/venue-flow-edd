use serde::{Deserialize, Serialize};



#[derive(Serialize,Deserialize)]
pub struct RentalRes{
    pub id:i64,
    pub venue_id:i64,
    pub venue_title:String,
    pub organizer_id:i64,
    pub start_time:String,
    pub end_time:String,
    pub activity_type:String,
    pub request_comments:Option<String>,
    pub status:String,
}
