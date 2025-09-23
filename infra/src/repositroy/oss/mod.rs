use aws_config::{BehaviorVersion, Region};
use aws_sdk_s3::config::Credentials;
use aws_sdk_s3::Client;
use std::io;
use std::sync::Arc;
use tracing::{error, info};

pub mod image_file;

#[derive(Debug, Clone)]
pub struct OssConfig {
    pub url: String,
    pub access_key: String,
    pub secret_key: String,
    pub bucket_name: String,
    pub temp_folder: String,
    pub region: Option<String>,
    pub custom_domain: Option<String>,
}

/// OSS 客户端配置
#[derive(Debug)]
pub struct OssClientConfig {
    pub client: Client,
    pub bucket_name: String,
    pub temp_folder: String,
    pub image_domain: String,
    pub region: Option<String>,
    pub custom_domain: Option<String>,
}

/// OSS 错误类型
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

    #[error("IO error: {0}")]
    Io(#[from] io::Error),
}

impl From<OssError> for io::Error {
    fn from(err: OssError) -> Self {
        match err {
            OssError::InvalidConfig(msg) => {
                io::Error::new(io::ErrorKind::InvalidInput, msg)
            },
            OssError::InvalidUrl(msg) => {
                io::Error::new(io::ErrorKind::InvalidInput, msg)
            },
            OssError::InvalidBucketName(msg) => {
                io::Error::new(io::ErrorKind::InvalidInput, msg)
            },
            OssError::ClientCreationFailed(msg) => {
                io::Error::new(io::ErrorKind::ConnectionAborted, msg)
            },
            OssError::ConnectionFailed(msg) => {
                io::Error::new(io::ErrorKind::ConnectionRefused, msg)
            },
            OssError::BucketNotExists { bucket_name } => io::Error::new(
                io::ErrorKind::NotFound,
                format!("Bucket '{}' not found", bucket_name),
            ),
            OssError::Io(io_err) => io_err,
        }
    }
}

fn gen_io_error<E: std::fmt::Display>(err: E, message: &str) -> io::Error {
    error!("{}: {}", message, err);
    io::Error::new(io::ErrorKind::InvalidInput, format!("{}: {}", message, err))
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
        // 使用自定义域名
        let domain = custom_domain.trim_end_matches('/');
        Ok(format!("https://{}/", domain))
    } else {
        // 使用标准 S3 域名格式
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

fn get_oss_fs() -> Result<OssConfig, Box<dyn std::error::Error>> {
    Ok(OssConfig {
        url: std::env::var("OSS_URL")
            .unwrap_or_else(|_| "https://s3.amazonaws.com".to_string()),
        access_key: std::env::var("OSS_ACCESS_KEY")
            .map_err(|_| "OSS_ACCESS_KEY environment variable not set")?,
        secret_key: std::env::var("OSS_SECRET_KEY")
            .map_err(|_| "OSS_SECRET_KEY environment variable not set")?,
        bucket_name: std::env::var("OSS_BUCKET_NAME")
            .map_err(|_| "OSS_BUCKET_NAME environment variable not set")?,
        temp_folder: std::env::var("OSS_TEMP_FOLDER")
            .unwrap_or_else(|_| "/tmp".to_string()),
        region: std::env::var("OSS_REGION").ok(),
        custom_domain: std::env::var("OSS_CUSTOM_DOMAIN").ok(),
    })
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_bucket_name_validation() {
        assert!(is_valid_bucket_name("my-bucket"));
        assert!(is_valid_bucket_name("my.bucket.name"));
        assert!(is_valid_bucket_name("bucket123"));

        assert!(!is_valid_bucket_name("MyBucket")); // 大写字母
        assert!(!is_valid_bucket_name("-bucket")); // 以连字符开头
        assert!(!is_valid_bucket_name("bucket-")); // 以连字符结尾
        assert!(!is_valid_bucket_name(".bucket")); // 以点开头
        assert!(!is_valid_bucket_name("bucket.")); // 以点结尾
        assert!(!is_valid_bucket_name("ab")); // 太短
    }

    #[test]
    fn test_image_domain_building() {
        let config = OssConfig {
            url: "https://s3.amazonaws.com".to_string(),
            access_key: "key".to_string(),
            secret_key: "secret".to_string(),
            bucket_name: "test-bucket".to_string(),
            temp_folder: "/tmp".to_string(),
            region: Some("us-west-2".to_string()),
            custom_domain: None,
        };

        let domain = build_image_domain(&config).unwrap();
        assert_eq!(domain, "https://test-bucket.s3.us-west-2.amazonaws.com/");

        let config_with_custom = OssConfig {
            custom_domain: Some("cdn.example.com".to_string()),
            ..config
        };

        let custom_domain = build_image_domain(&config_with_custom).unwrap();
        assert_eq!(custom_domain, "https://cdn.example.com/");
    }
}
