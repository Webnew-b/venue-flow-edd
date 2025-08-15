use std::time::Duration;
use std::io;
use std::path::PathBuf;
use std::sync::Arc;

use lazy_static::lazy_static;
use log::error;
use minio::s3::args::BucketExistsArgs;
use minio::s3::client::{Client, ClientBuilder};
use minio::s3::creds::StaticProvider;
use minio::s3::http::BaseUrl;
use tokio::sync::mpsc::{Receiver, Sender};
use tokio::sync::{mpsc, Mutex};
use tokio::time::sleep;

use super::config::config::get_oss_fs;
use super::config::gen_io_error;

pub mod image_file;

lazy_static!{
    pub static ref DETELE_FILE_QUEUE : Arc<(Sender<PathBuf>,Mutex<Receiver<PathBuf>>)> = {
        let (sender,receiver) = mpsc::channel(200);
        Arc::new((sender,Mutex::new(receiver)))
    };
}


#[derive(Clone)]
pub struct OssClientConfig {
    client:Client,
    bucket_name:String,
    temp_folder:String,
    image_domain:String,
}

impl OssClientConfig {
    pub fn get_client(&self) -> &Client {
        &self.client
    }
    
    pub fn get_bucket_name(&self) -> &String {
        &self.bucket_name
    }

    pub fn get_temp_folder(&self) -> &String {
        &self.temp_folder
    }
    pub fn get_image_domain(&self) -> &String {
        &self.image_domain
    }
}

async fn delete_cache_file(path:PathBuf) {
    if let Err(e) = tokio::fs::remove_file(&path).await {
        tracing::error!("Delete temporary file fail,reason:{}",e.to_string())
    }
}

async fn init_delete_queue(){
    tokio::spawn(async{
        let a = DETELE_FILE_QUEUE.clone();
        let r = &a.1;
        loop {
            let mut lock  = r.lock().await;

            while let Some(path) = lock.recv().await {
                delete_cache_file(path).await;
            }

            drop(lock);
            sleep(Duration::from_secs(2)).await; 
        }
    });
}

pub async fn init_oss_client()->Result<Arc<OssClientConfig>,io::Error> {

    let config = get_oss_fs()
        .map_err(|e| gen_io_error(e, "Oss configuration seems invalid"))?;
    let url = config.url.parse::<BaseUrl>()
        .map_err(|e| gen_io_error(e, "Oss url is invalid format"))?;

    let provider = StaticProvider::new(
        &config.access_key, 
        &config.secret_key, 
        None
    );
    let bucket_name = BucketExistsArgs::new(&config.bucket_name)
        .map_err(|e| gen_io_error(e, "Oss bucket name is invalid format"))?;

    let client = ClientBuilder::new(url.clone())
        .provider(Some(Box::new(provider)))
        .build()
        .map_err(|e|{
            error!("{}",e.to_string());
            io::Error::new(io::ErrorKind::ConnectionAborted,"Oss client created fail.")
        })?;

    let exists:bool = client
        .bucket_exists(&bucket_name)
        .await
        .map_err(|e|{
            error!("{}",e.to_string()); 
            io::Error::new(io::ErrorKind::InvalidInput,"Oss client created fail.")
        })?;
    if !exists {
        let e = io::Error::new(io::ErrorKind::InvalidData,"Oss bucket is invalid");
        return Err(e);
    }

    let uri = format!("{}/{}/",config.url,config.bucket_name);

    let oss_config = OssClientConfig{
        client,
        bucket_name:config.bucket_name,
        temp_folder:config.temp_folder,
        image_domain:uri
    };

    init_delete_queue().await;

    Ok(Arc::new(oss_config))
}
