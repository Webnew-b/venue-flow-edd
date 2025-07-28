use mockall::mock;
use async_trait::async_trait;
use domain_core::user::lessor::Lessor;
use domain_core::user::organizer::Organizer;
use domain_core::user::User;
use domain::user_domain::{UserGenerator, UserRepository, UserValidation};
use domain::domain_error::DomainError;
use domain::user_domain::user_dto::{UserLoginName, UserLoginToken};


mock!{
    pub UserRepo {}
    #[async_trait]
    impl UserRepository for UserRepo {
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
}

mock!{
    pub UserValidator {}
    #[async_trait]
    impl UserValidation for UserValidator {
        async fn valid_email(&self,email:&str) -> Result<(),DomainError>;
        async fn valid_username(&self,username:&str) -> Result<(),DomainError>;
        async fn exist_email(&self,email:&str) -> Result<(),DomainError>;
    } 
}

mock!{
    pub UserGeneration {}
    impl UserGenerator for UserGeneration {
        fn generate_token(&self,user:&User) -> Result<UserLoginToken,DomainError>;
    }
}



pub struct TestUserMocks {
    pub repo:MockUserRepo,
    pub validator:MockUserValidator,
    pub generator:MockUserGeneration
}

pub fn mock_user_setup() -> TestUserMocks {
    TestUserMocks { 
        repo: MockUserRepo::new(), 
        validator: MockUserValidator::new(), 
        generator: MockUserGeneration::new() 
    }
}
