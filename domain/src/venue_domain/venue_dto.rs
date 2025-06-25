use serde::{Deserialize, Serialize};


#[derive(Deserialize,Serialize)]
pub struct IndexVenue {
    pub lessor_avatar:i64,
    pub lessor_id:i64,
    pub name:String,
    pub location:String,
}
