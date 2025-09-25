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
