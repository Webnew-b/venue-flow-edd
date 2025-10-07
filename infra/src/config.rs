use core::fmt;
use std::{env, str::FromStr};

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

pub fn get_env_value_from_key(key: &str) -> Result<String, ConfigError> {
    dotenv::dotenv().ok();
    std::env::var(key).map_err(|e| match e {
        env::VarError::NotPresent => ConfigError::NotFound(key.to_string()),
        env::VarError::NotUnicode(_) => ConfigError::Illegal(key.to_string()),
    })
}

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

pub fn get_env_type() -> Result<AppEnv, ConfigError> {
    let key = "APP_ENV";

    let env = get_env_value_from_key(key)?;

    let value = AppEnv::from_str(&env)?;

    Ok(value)
}

pub fn get_web_server_config() -> Result<(String, u16), ConfigError> {
    let key = "SERVER_ADDR";
    let addr = get_env_value_from_key(&key)?;

    let key = "PORT";
    let port = get_env_value_from_key(key)?;

    let port_check: Result<u16, _> = port.parse();
    let port = match port_check {
        Ok(p) => p,
        Err(_) => {
            log::error!("The port is not a number");
            return Err(ConfigError::Illegal(key.to_owned()));
        },
    };

    Ok((addr, port))
}

pub fn get_jwt_secret_key() -> Result<String, ConfigError> {
    let key = "JWT_SECRET";
    let content = get_env_value_from_key(key)?;
    Ok(content)
}
