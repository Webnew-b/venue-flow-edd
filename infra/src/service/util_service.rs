use std::ops::Deref;
use std::path::{Path, PathBuf};
use std::str::FromStr;
use std::sync::Arc;

use async_trait::async_trait;
use aws_sdk_s3::Client;
use bcrypt::{hash, verify, DEFAULT_COST};
use domain::domain_error::DomainError;

use domain::domain_error::domain_user_error::DomainUserError;
use domain::util_trait::{ImageRepository, PasswordHasher};
use tokio::fs;

use crate::infra_error::InfraError;
use crate::repositroy::oss::config::OssClientConfig;
use crate::repositroy::oss::image_file::{
    gen_uuid_image_name, save_file_to_oss, ImagePath,
};

pub struct UtilService {
    oss_domain: String,
    oss_client: Arc<Client>,
    oss_bucket_name: String,
    oss_temp_file_path: String,
}

impl UtilService {
    pub fn new(config: OssClientConfig) -> Self {
        Self {
            oss_domain: config.image_domain,
            oss_client: Arc::new(config.client),
            oss_bucket_name: config.bucket_name,
            oss_temp_file_path: config.temp_folder,
        }
    }
}

fn get_file_name(path: &Path) -> Result<String, DomainError> {
    if let Some(file_name_os_str) = path.file_name() {
        if let Some(file_name_str) = file_name_os_str.to_str() {
            return Ok(file_name_str.to_string());
        } else {
            return Err(InfraError::FileTypeIsInvalid.into());
        }
    } else {
        return Err(InfraError::FileNotFound.into());
    }
}

async fn upload_image(
    image: &Path,
    domain: String,
    temp_folder: &str,
    client: &Client,
    bucket_name: String,
) -> Result<String, DomainError> {
    let temp_file_name = get_file_name(image)?;

    let ext = Path::new(&temp_file_name)
        .extension()
        .and_then(|ext| ext.to_str())
        .ok_or(InfraError::ObtainExtensionFail)?;

    let file_name: &str = &gen_uuid_image_name(ext);

    let mut file_path = PathBuf::from_str(temp_folder).map_err(|e| {
        log::error!("{}", e);
        InfraError::SaveImageFail
    })?;

    file_path.push(file_name);

    fs::copy(image, file_path.clone()).await.map_err(|e| {
        log::error!("Could not move the file,cause:{}", e.to_string());
        InfraError::SaveImageFail
    })?;

    let image_path = ImagePath {
        image_path: file_path.as_path(),
        bucket_name,
    };

    let uri = save_file_to_oss(&client, image_path).await.map_err(|e| {
        log::error!("{}", e);
        InfraError::SaveImageFail
    })?;

    let uri = format!("{}{}", domain, uri);
    Ok(uri)
}

#[async_trait]
impl ImageRepository for UtilService {
    async fn upload_image(&self, image: &Path) -> Result<String, DomainError> {
        let uri = upload_image(
            image,
            self.oss_domain.clone(),
            self.oss_temp_file_path.as_str(),
            self.oss_client.deref(),
            self.oss_bucket_name.clone(),
        )
        .await?;
        Ok(uri)
    }

    async fn upload_images(
        &self,
        image: Vec<PathBuf>,
    ) -> Result<Vec<String>, DomainError> {
        // todo support rollback
        let mut uris = vec![];
        for item in image {
            let uri = upload_image(
                item.as_path(),
                self.oss_domain.clone(),
                self.oss_temp_file_path.as_str(),
                self.oss_client.deref(),
                self.oss_bucket_name.clone(),
            )
            .await?;
            uris.push(uri);
        }
        Ok(uris)
    }
}

impl PasswordHasher for UtilService {
    fn hash(
        &self,
        password: &str,
    ) -> Result<String, domain::domain_error::DomainError> {
        let password = hash(password, DEFAULT_COST).map_err(|e| {
            log::error!("{}", e);
            InfraError::HashPasswordFail {
                password: password.to_string(),
            }
        })?;
        Ok(password)
    }

    fn verify(
        &self,
        password: &str,
        hashed: &str,
    ) -> Result<(), domain::domain_error::DomainError> {
        let res = verify(password, hashed).map_err(|e| {
            log::error!("{}", e);
            InfraError::VerifyPasswordFail
        })?;
        if res {
            return Ok(());
        } else {
            return Err(DomainUserError::PasswordIncorrect.into());
        }
    }
}
