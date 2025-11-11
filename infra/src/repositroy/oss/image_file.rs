use aws_sdk_s3::primitives::ByteStream;
use aws_sdk_s3::Client;
use std::path::{Path, PathBuf};

use crate::infra_error::InfraError;
use crate::repositroy::oss::OssError;

pub struct ImagePath<'a> {
    pub bucket_name: String,
    pub image_path: &'a Path,
}

pub fn gen_uuid_image_name(ext: &str) -> String {
    let id = uuid::Uuid::new_v4();
    format!("image_{}.{}", id, ext)
}

pub fn change_image_type(
    image_path: &Path,
    output: &Path,
) -> Result<(), InfraError> {
    let img = image::open(image_path).map_err(|e| {
        tracing::error!("Failed to open image: {}", e);
        OssError::CoundNotOpenFile
    })?;

    img.save_with_format(output, image::ImageFormat::WebP)
        .map_err(|e| {
            tracing::error!("Failed to save image as WebP: {}", e);
            OssError::InvalidFileFormat.into()
        })
}

#[allow(unused)]
fn change_path_extension(source_path: &Path) -> Result<PathBuf, InfraError> {
    let parent_path = match source_path.parent() {
        Some(p) => p,
        None => {
            return Err(InfraError::FileNotFound);
        },
    };

    let mut file_name = match source_path.file_stem() {
        Some(p) => p.to_string_lossy().to_string(),
        None => {
            return Err(InfraError::FileNotRead);
        },
    };

    file_name.push_str(".webp");
    Ok(parent_path.join(file_name))
}

async fn upload_object(
    client: &Client,
    bucket: &str,
    key: &str,
    data: Vec<u8>,
    content_type: Option<&str>,
) -> Result<String, InfraError> {
    let byte_stream = ByteStream::from(data);
    let mut put_object = client
        .put_object()
        .bucket(bucket)
        .key(key)
        .body(byte_stream);

    if let Some(ct) = content_type {
        put_object = put_object.content_type(ct);
    }

    let response = put_object.send().await.map_err(|e| {
        tracing::error!("Failed to upload object to S3: {}", e);
        InfraError::SaveImageFail
    })?;

    let etag = response.e_tag().unwrap_or("unknown").to_string();
    tracing::info!("Object uploaded successfully: {} (ETag: {})", key, etag);
    Ok(etag)
}

#[allow(unused)]
fn guess_content_type(path: &Path) -> &'static str {
    match path.extension().and_then(|ext| ext.to_str()) {
        Some("html") | Some("htm") => "text/html",
        Some("css") => "text/css",
        Some("js") => "application/javascript",
        Some("json") => "application/json",
        Some("png") => "image/png",
        Some("jpg") | Some("jpeg") => "image/jpeg",
        Some("gif") => "image/gif",
        Some("svg") => "image/svg+xml",
        Some("webp") => "image/webp",
        Some("pdf") => "application/pdf",
        Some("txt") => "text/plain",
        Some("zip") => "application/zip",
        _ => "application/octet-stream",
    }
}

fn create_temp_webp_path() -> PathBuf {
    let temp_dir = std::env::temp_dir();
    let uuid = uuid::Uuid::new_v4();
    let temp_filename = format!("temp_image_{}.webp", uuid);
    temp_dir.join(temp_filename)
}

pub async fn save_file_to_oss<'a>(
    client: &Client,
    path: ImagePath<'a>,
) -> Result<String, InfraError> {
    let temp_webp_path = create_temp_webp_path();

    change_image_type(path.image_path, &temp_webp_path).map_err(|e| {
        tracing::error!("Failed to convert image to WebP: {}", e);
        InfraError::FileNotRead
    })?;

    let webp_data = tokio::fs::read(&temp_webp_path).await.map_err(|e| {
        tracing::error!("Failed to read converted WebP file: {}", e);
        InfraError::FileNotRead
    })?;

    let s3_key = gen_uuid_image_name("webp");

    let result = upload_object(
        client,
        &path.bucket_name,
        &s3_key,
        webp_data,
        Some("image/webp"),
    )
    .await;

    send2queue(temp_webp_path).await;

    result
}

async fn send2queue(p: PathBuf) {
    if let Err(e) = tokio::fs::remove_file(&p).await {
        tracing::warn!("Failed to remove temporary file {:?}: {}", p, e);
    } else {
        tracing::debug!("Successfully removed temporary file: {:?}", p);
    }
}
