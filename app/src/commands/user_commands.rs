use serde::{Deserialize, Serialize};


pub struct RegisterUserCommand {
    pub username:String,
    pub email:String,
    pub avatar:String,
    pub gender:String,
    pub password:String,
    pub introduce:Option<String>
}

pub enum UserLoginType {
    Email(String),
    UserName(String)
}

pub struct LoginUserCommand {
    pub login_type:UserLoginType,
    pub password:String
}

#[derive(Serialize,Deserialize)]
pub struct RegisteredUserDto {
    pub id:i64,
    pub username:String,
    pub email:String,
    pub avatar:String,
    pub gender:String,
    pub password:String,
}

