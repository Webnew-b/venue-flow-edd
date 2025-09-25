use aws_config::{BehaviorVersion, Region};
use aws_sdk_s3::config::Credentials;
use aws_sdk_s3::Client;
use std::sync::Arc;
use tracing::{error, info};

use super::oss::config::*;

pub mod config;
pub mod image_file;

#[derive(Debug, thiserror::Error)]
pub enum OssError {
    #[error("OSS configuration is invalid: {0}")]
    InvalidConfig(String),

    #[error("OSS URL format is invalid: {0}")]
    InvalidUrl(String),

    #[error("OSS bucket name format is invalid: {0}")]
    InvalidBucketName(String),

    #[error("Failed to create OSS client: {0}")]
    ClientCreationFailed(String),

    #[error("Failed to connect to OSS: {0}")]
    ConnectionFailed(String),

    #[error("OSS bucket does not exist: {bucket_name}")]
    BucketNotExists { bucket_name: String },

    #[error("Could not open the file.")]
    CoundNotOpenFile,

    #[error("The file is invalid format.")]
    InvalidFileFormat,
}

pub async fn init_oss_client() -> Result<Arc<OssClientConfig>, OssError> {
    info!("Initializing OSS client...");

    let config =
        get_oss_fs().map_err(|e| OssError::InvalidConfig(e.to_string()))?;

    validate_oss_config(&config)?;

    let client = create_s3_client(&config).await?;

    validate_bucket_exists(&client, &config.bucket_name).await?;

    let image_domain = build_image_domain(&config)?;

    let oss_client_config = OssClientConfig {
        client,
        bucket_name: config.bucket_name.clone(),
        temp_folder: config.temp_folder.clone(),
        image_domain,
        region: config.region.clone(),
        custom_domain: config.custom_domain.clone(),
    };

    info!("OSS client initialized successfully");
    info!("Bucket: {}", config.bucket_name);
    info!("Region: {:?}", config.region);
    info!("Image domain: {}", oss_client_config.image_domain);

    Ok(Arc::new(oss_client_config))
}

fn validate_oss_config(config: &OssConfig) -> Result<(), OssError> {
    if config.url.is_empty() {
        return Err(OssError::InvalidUrl(
            "OSS URL cannot be empty".to_string(),
        ));
    }

    if config.access_key.is_empty() {
        return Err(OssError::InvalidConfig(
            "OSS access key cannot be empty".to_string(),
        ));
    }

    if config.secret_key.is_empty() {
        return Err(OssError::InvalidConfig(
            "OSS secret key cannot be empty".to_string(),
        ));
    }

    if config.bucket_name.is_empty() {
        return Err(OssError::InvalidBucketName(
            "OSS bucket name cannot be empty".to_string(),
        ));
    }

    if !is_valid_bucket_name(&config.bucket_name) {
        return Err(OssError::InvalidBucketName(format!(
            "Invalid bucket name format: {}",
            config.bucket_name
        )));
    }

    info!("OSS configuration validation passed");
    Ok(())
}

fn is_valid_bucket_name(bucket_name: &str) -> bool {
    if bucket_name.len() < 3 || bucket_name.len() > 63 {
        return false;
    }

    bucket_name.chars().all(|c| {
        c.is_ascii_lowercase() || c.is_ascii_digit() || c == '.' || c == '-'
    }) && !bucket_name.starts_with('-')
        && !bucket_name.ends_with('-')
        && !bucket_name.starts_with('.')
        && !bucket_name.ends_with('.')
}

async fn create_s3_client(config: &OssConfig) -> Result<Client, OssError> {
    info!("Creating S3 client...");

    let region = config.region.as_deref().unwrap_or("us-east-1");

    let credentials = Credentials::new(
        &config.access_key,
        &config.secret_key,
        None, // session_token
        None, // expiry
        "static",
    );

    let mut aws_config = aws_config::defaults(BehaviorVersion::latest())
        .credentials_provider(credentials)
        .region(Region::new(region.to_string()));

    if !config.url.contains("amazonaws.com") {
        let endpoint_url = config.url.trim_end_matches('/');
        aws_config = aws_config.endpoint_url(endpoint_url);
        info!("Using custom S3 endpoint: {}", endpoint_url);
    }

    let aws_config = aws_config.load().await;
    let client = Client::new(&aws_config);

    info!("S3 client created successfully");
    Ok(client)
}

async fn validate_bucket_exists(
    client: &Client,
    bucket_name: &str,
) -> Result<(), OssError> {
    info!("Validating bucket existence: {}", bucket_name);

    match client.head_bucket().bucket(bucket_name).send().await {
        Ok(_) => {
            info!("Bucket '{}' exists and is accessible", bucket_name);
            Ok(())
        },
        Err(e) => {
            error!("Failed to access bucket '{}': {}", bucket_name, e);
            Err(OssError::BucketNotExists {
                bucket_name: bucket_name.to_string(),
            })
        },
    }
}

fn build_image_domain(config: &OssConfig) -> Result<String, OssError> {
    if let Some(custom_domain) = &config.custom_domain {
        let domain = custom_domain.trim_end_matches('/');
        Ok(format!("https://{}/", domain))
    } else {
        let region = config.region.as_deref().unwrap_or("us-east-1");
        let url = if region == "us-east-1" {
            format!("https://{}.s3.amazonaws.com/", config.bucket_name)
        } else {
            format!(
                "https://{}.s3.{}.amazonaws.com/",
                config.bucket_name, region
            )
        };
        Ok(url)
    }
}

pub async fn init_oss_client_with_config(
    config: OssConfig,
) -> Result<Arc<OssClientConfig>, OssError> {
    info!("Initializing OSS client with custom configuration...");

    validate_oss_config(&config)?;

    let client = create_s3_client(&config).await?;

    validate_bucket_exists(&client, &config.bucket_name).await?;

    let image_domain = build_image_domain(&config)?;

    let oss_client_config = OssClientConfig {
        client,
        bucket_name: config.bucket_name.clone(),
        temp_folder: config.temp_folder.clone(),
        image_domain,
        region: config.region.clone(),
        custom_domain: config.custom_domain.clone(),
    };

    info!("OSS client initialized successfully with custom config");
    Ok(Arc::new(oss_client_config))
}

pub async fn health_check_oss(
    client_config: &OssClientConfig,
) -> Result<(), OssError> {
    info!("Performing OSS health check...");

    match client_config
        .client
        .head_bucket()
        .bucket(&client_config.bucket_name)
        .send()
        .await
    {
        Ok(_) => {
            info!("OSS health check passed");
            Ok(())
        },
        Err(e) => {
            error!("OSS health check failed: {}", e);
            Err(OssError::ConnectionFailed(format!(
                "Health check failed: {}",
                e
            )))
        },
    }
}
