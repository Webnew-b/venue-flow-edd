use aws_sdk_s3::Client;

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

#[derive(Debug)]
pub struct OssClientConfig {
    pub client: Client,
    pub bucket_name: String,
    pub temp_folder: String,
    pub image_domain: String,
    pub region: Option<String>,
    pub custom_domain: Option<String>,
}

pub fn get_oss_fs() -> Result<OssConfig, Box<dyn std::error::Error>> {
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
