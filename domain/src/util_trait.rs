use std::path::Path;

use async_trait::async_trait;

use crate::domain_error::DomainError;


#[async_trait]
pub trait ImageRepository:Send + Sync {
    async fn upload_image(&self,image:&Path)
        ->Result<String,DomainError>;

    async fn upload_images(&self,image:Vec<&Path>)
        ->Result<Vec<String>,DomainError>;
}


