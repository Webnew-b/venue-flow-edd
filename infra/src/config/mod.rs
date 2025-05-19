use core::fmt;
use std::io;
use log::error;

pub mod config;

#[derive(Debug)]
pub enum ConfigError {
    NotFound(String),
    Illegal(String),
    Other(String),
}

impl fmt::Display for ConfigError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ConfigError::NotFound(v) => {
                write!(f, "The {} fleid is not found", v)
            },
            ConfigError::Illegal(v) => write!(f, "The {} fleid is illegal", v),
            ConfigError::Other(v) => write!(f, "{}", v),
        }
    }
}

impl std::error::Error for ConfigError {}

pub struct OssConfig {
    pub access_key:String,
    pub secret_key:String,
    pub bucket_name:String,
    pub url:String,
    pub temp_folder:String
}

impl OssConfig {
    pub fn new()->Self {
        Self{
            access_key:String::new(),
            secret_key:String::new(),
            bucket_name:String::new(),
            url:String::new(),
            temp_folder:String::new(),
        }
    }
    pub fn add_by_env_key(&mut self,key:&str,value:String){
        match key {
            "OSS_URL" => {
                self.url = value;
            },
            "OSS_ACCESS_KEY" => {
                self.access_key = value;
            },
            "OSS_SECRET_KEY" => {
                self.secret_key = value;
            }
            "OSS_BUCKET_NAME" => {
                self.bucket_name = value;
            },
            "OSS_TENP_FOLDER" => {
                self.temp_folder = value;
            }
            _ => ()
        };
    }
}
pub fn gen_io_error<E>(error: E, tips:&str) -> io::Error
where
    E: std::error::Error + ToString,
{
    error!( "{}", error.to_string());
    io::Error::new(io::ErrorKind::InvalidData,tips)
}

