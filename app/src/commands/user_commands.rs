use std::path::Path;

use domain::user_domain::user_dto::{UserLoginEnum, UserLoginName};
use garde::Validate;
use serde::{Deserialize, Serialize};


#[derive(Validate,Clone)]
pub struct Email{
    #[garde(email)]
    pub address:String
}

#[derive(Clone)]
pub enum UserLoginType {
    Email(Email),
    UserName(String)
}

pub struct LoginUserCommand {
    pub login_type:UserLoginType,
    pub password:String
}


impl From<LoginUserCommand> for UserLoginName {
    fn from(value: LoginUserCommand) -> Self {
        let login_type = match value.login_type {
            UserLoginType::Email(e) => UserLoginEnum::Email(e.address),
            UserLoginType::UserName(u) => UserLoginEnum::UserName(u),
        };
        Self {
            login_type,
            password:value.password,
        }
    }
}

#[derive(Serialize,Deserialize,Clone)]
pub struct LoginedRes {
    pub id:i64,
    pub username:String,
    pub token:String
}

pub struct RegisterUserCommand<'image> {
    pub username:String,
    pub email:String,
    pub avatar:&'image Path,
    pub gender:String,
    pub password:String,
    pub introduce:Option<String>
}

#[derive(Serialize,Deserialize,Clone)]
pub struct RegisteredUserDto {
    pub id:i64,
    pub username:String,
    pub email:String,
    pub avatar:String,
    pub gender:String,
    pub password:String,
}


pub struct UpdateUserCommand<'image_path> {
    pub id:i64,
    pub username:Option<String>,
    pub email:Option<String>,
    pub password:Option<String>,
    pub avatar:Option<&'image_path Path>,
    pub introduce:Option<String>,
    pub gender:Option<String>,
}

#[derive(Serialize,Deserialize,Clone)]
pub struct UserDetail{
    pub id:i64,
    pub username:String,
    pub email:String,
    pub avatar:String,
    pub gender:String,
    pub introduce:Option<String>,
    pub is_lessor:bool,
    pub is_organizer:bool,
}

#[derive(Serialize,Deserialize,Clone)]
pub struct LessorDetail{
    pub id:i64,
    pub username:String,
    pub email:String,
    pub phone:String,
    pub venues:Vec<String>,
}

#[derive(Serialize,Deserialize,Clone)]
pub struct OrganizerDetail{
    pub id:i64,
    pub username:String,
    pub email:String,
    pub phone:String,
}
