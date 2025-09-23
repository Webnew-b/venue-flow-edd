use std::env;
use std::str::FromStr;

use dotenv::{dotenv, Error};
use log::{error, log, Level};

use super::ConfigError;

pub enum AppEnv {
    DEVELOP,
    PRODUCTION,
}

impl FromStr for AppEnv {
    type Err = ConfigError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "develop" => Ok(Self::DEVELOP),
            "production" => Ok(Self::PRODUCTION),
            _ => Err(ConfigError::Illegal(
                "The env mode configurtion is invaild".to_string(),
            )),
        }
    }
}

fn get_env_value_from_key(key: &str) -> Result<String, Error> {
    dotenv().ok();
    match env::var(key) {
        Ok(e) => Ok(e),
        Err(e) => Err(Error::EnvVar(e)),
    }
}

pub fn get_web_server_config() -> Result<(String, u16), ConfigError> {
    let key = "SERVER_ADDR";
    let addr_res = get_env_value_from_key(&key);
    let addr = match addr_res {
        Ok(a) => a,
        Err(e) => {
            log!(Level::Error, "{}", e.to_string());
            return Err(ConfigError::Illegal(key.to_owned()));
        },
    };

    let key = "PORT";
    let port_res = get_env_value_from_key(key);
    let port = match port_res {
        Ok(p) => p,
        Err(e) => {
            log!(Level::Error, "{}", e.to_string());
            return Err(ConfigError::Illegal(key.to_owned()));
        },
    };

    let port_check: Result<u16, _> = port.parse();
    let port = match port_check {
        Ok(p) => p,
        Err(_) => {
            log!(Level::Error, "The port is not a number");
            return Err(ConfigError::Illegal(key.to_owned()));
        },
    };

    Ok((addr, port))
}

pub fn get_db_config() -> Result<String, ConfigError> {
    let mut db_url_array: Vec<String> = Vec::new();
    let keys: [&str; 5] = [
        "DATABASE_USER",
        "DATABASE_PASSWORD",
        "DATABASE_ADDR",
        "DATABASE_PORT",
        "DATABASE_NAME",
    ];

    for &item in &keys {
        let res = get_env_value_from_key(&item);
        let value = match res {
            Ok(v) => v,
            Err(e) => {
                log!(Level::Error, "{}", e.to_string());
                return Err(ConfigError::Illegal(item.to_owned()));
            },
        };
        if value.is_empty() {
            return Err(ConfigError::NotFound(item.to_owned()));
        }
        db_url_array.push(value);
    }

    Ok(format!(
        "postgres://{}:{}@{}:{}/{}",
        db_url_array[0],
        db_url_array[1],
        db_url_array[2],
        db_url_array[3],
        db_url_array[4],
    ))
}

pub fn get_redis_url() -> Result<String, ConfigError> {
    let addr_key = "REDIS_ADDR";
    let port_key = "REDIS_PORT";

    let addr_res =
        get_env_value_from_key(addr_key).map_err(|e| gen_error(e, addr_key))?;

    let port_res =
        get_env_value_from_key(port_key).map_err(|e| gen_error(e, port_key))?;

    let url = format!("redis://{}:{}", addr_res, port_res);

    log!(Level::Info, "{}", &url);

    Ok(url)
}

pub fn get_env_type() -> Result<AppEnv, ConfigError> {
    let key = "APP_ENV";

    let env = get_env_value_from_key(key).map_err(|e| gen_error(e, key))?;

    let value = AppEnv::from_str(&env)?;

    Ok(value)
}

pub fn get_oss_fs() -> Result<super::OssConfig, ConfigError> {
    let key_list: [&str; 5] = [
        "OSS_URL",
        "OSS_ACCESS_KEY",
        "OSS_SECRET_KEY",
        "OSS_BUCKET_NAME",
        "OSS_TENP_FOLDER",
    ];
    let mut config = super::OssConfig::new();
    for item in key_list {
        let env =
            get_env_value_from_key(item).map_err(|e| gen_error(e, item))?;
        let _ = &config.add_by_env_key(item, env.to_string());
    }
    Ok(config)
}

fn gen_error<E>(error: E, key: &str) -> ConfigError
where
    E: std::error::Error + ToString,
{
    error!("{}", error.to_string());
    ConfigError::NotFound(key.to_owned())
}
