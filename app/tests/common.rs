use mockall::mock;
use async_trait::async_trait;

mock!{
    struct UserRepo;
    #[async_trait]
    impl UserRepos
}
