use std::path::{Path, PathBuf};

use async_trait::async_trait;

use crate::domain_error::DomainError;

#[async_trait]
pub trait ImageRepository: Send + Sync {
    async fn upload_image(&self, image: &Path) -> Result<String, DomainError>;

    async fn upload_images(
        &self,
        image: Vec<PathBuf>,
    ) -> Result<Vec<String>, DomainError>;
}

pub trait PasswordHasher: Send + Sync {
    fn hash(&self, password: &str) -> Result<String, DomainError>;
    fn verify(&self, password: &str, hashed: &str) -> Result<(), DomainError>;
}
