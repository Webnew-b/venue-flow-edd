use chrono::Utc;
use domain::domain_error::DomainError;
use domain::util_trait::ImageRepository;
use domain::util_trait::PasswordHasher;
use domain_core::utils::Clock;

use async_trait::async_trait;
use mockall::mock;
use std::path::Path;
use std::path::PathBuf;

mock! {
    pub PwdHasher {}
    #[async_trait]
    impl PasswordHasher for PwdHasher {
        fn hash(&self, password: &str) -> Result<String, DomainError>;
        fn verify(&self, password: &str, hashed: &str) -> Result<(), DomainError>;
    }
}

mock! {
    pub ImageRepo {}
    #[async_trait]
    impl ImageRepository for ImageRepo {
        async fn upload_image(&self,image:&Path)
            ->Result<String,DomainError>;

        async fn upload_images(&self,image:Vec<PathBuf>)
            ->Result<Vec<String>,DomainError>;

    }
}

pub struct TestUtilMock {
    pub image_repo:    MockImageRepo,
    pub password_hash: MockPwdHasher,
}

pub fn mock_utils_setup() -> TestUtilMock {
    TestUtilMock {
        image_repo:    MockImageRepo::new(),
        password_hash: MockPwdHasher::new(),
    }
}

pub struct MockTime {}

impl Clock for MockTime {
    fn now(&self) -> chrono::DateTime<chrono::Utc> {
        Utc::now()
    }
}
