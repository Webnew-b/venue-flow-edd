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
