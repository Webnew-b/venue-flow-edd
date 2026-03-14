pub struct GetUserProfile {
    pub id:       Option<i64>,
    pub username: Option<String>,
    pub email:    Option<String>,
}

pub struct GetUsers {
    pub page:  u64,
    pub limit: u64,
}
