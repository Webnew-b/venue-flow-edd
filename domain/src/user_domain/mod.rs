use domain_core::user::User;

use crate::domain_error::DomainError;
use crate::user_domain::user_dto::{UserLoginName, UserLoginToken};

pub mod user_dto;

#[allow(async_fn_in_trait)]
pub trait UserValidation {
    async fn valid_email(&self,email:&str) -> Result<(),DomainError>;
    async fn valid_username(&self,username:&str) -> Result<(),DomainError>;
    async fn exist_email(&self,email:&str) -> Result<(),DomainError>;
}

pub trait UserGenerator {
    fn generate_token(&self,user:&User) -> Result<UserLoginToken,DomainError>;
}


#[allow(async_fn_in_trait)]
pub trait UserRepository {
    async fn find_user_by_id(&self,id:i64) ->
        Result<User,DomainError>;

    async fn find_user_by_name_and_pwd(&self,login:UserLoginName) ->
        Result<User,DomainError>;

    async fn save_user(self:&Self,user:User) ->
        Result<(),DomainError>;

    async fn create_user(self:&Self,user:User) ->
        Result<User,DomainError>;

    async fn delete_user(self:&Self,id:i64) ->
        Result<(),DomainError>;

    //async fn cache_user(self:&Self,user:UserLoginName) ->
    //    Result<String,DomainError>;

    async fn logout(self:&Self,token:String) ->
        Result<(),DomainError>;

}
