use async_trait::async_trait;
use domain_core::user::lessor::Lessor;
use domain_core::user::organizer::Organizer;
use domain_core::user::User;

use crate::domain_error::DomainError;
use crate::user_domain::user_dto::{UserLoginName, UserLoginToken};

pub mod user_dto;

#[async_trait]
pub trait UserValidation:Sync+Send {
    async fn valid_email(&self,email:&str) -> Result<(),DomainError>;
    async fn valid_username(&self,username:&str) -> Result<(),DomainError>;
    async fn exist_email(&self,email:&str) -> Result<(),DomainError>;
}

pub trait UserGenerator:Sync+Send {
    fn generate_token(&self,user:&User) -> Result<UserLoginToken,DomainError>;
}


#[async_trait]
pub trait UserRepository:Sync+Send {
    async fn find_user_by_id(&self,id:i64) ->
        Result<User,DomainError>;

    async fn find_user_by_name(&self,login:UserLoginName) ->
        Result<User,DomainError>;

    async fn save_user(self:&Self,user:User) ->
        Result<(),DomainError>;

    async fn create_user(self:&Self,user:User) ->
        Result<User,DomainError>;

    async fn delete_user(self:&Self,id:i64) ->
        Result<(),DomainError>;

    async fn logout(self:&Self,token:String) ->
        Result<(),DomainError>;

    async fn find_user_has_organizer_role(&self,user_id:i64) 
        -> Result<Option<Organizer>,DomainError>;

    async fn find_user_has_lessor_role(&self,user_id:i64) 
        -> Result<Option<Lessor>,DomainError>;

    async fn find_organizer_by_user_id(&self,user_id:i64) 
        -> Result<Organizer,DomainError>;

    async fn find_lessor_by_user_id(&self,user_id:i64) 
        -> Result<Lessor,DomainError>;

    async fn find_lessor_by_id(&self,id:i64)
        -> Result<Lessor,DomainError>;

    async fn find_organizer_by_id(&self,id:i64)
        -> Result<Organizer,DomainError>;
}
