use crate::{
    config::{get_env_value_from_key, ConfigError},
    infra_error::InfraError,
};

pub fn get_db_config() -> Result<String, InfraError> {
    let mut db_url_array: Vec<String> = Vec::new();
    let keys: [&str; 5] = [
        "DATABASE_USER",
        "DATABASE_PASSWORD",
        "DATABASE_ADDR",
        "DATABASE_PORT",
        "DATABASE_NAME",
    ];

    for &item in &keys {
        let res = get_env_value_from_key(&item)?;
        if res.is_empty() {
            return Err(ConfigError::NotFound(item.to_owned()).into());
        }
        db_url_array.push(res);
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
