pub enum UserLoginEnum {
    UserName(String),
    Email(String),
}

pub struct UserLoginName {
    pub login_type:UserLoginEnum,
    pub password:String,
}

pub struct UserLoginToken {
    pub token:String
}


