use crate::{config::get_env_value_from_key, infra_error::InfraError};

pub fn get_redis_url() -> Result<String, InfraError> {
    let addr_key = "REDIS_ADDR";
    let port_key = "REDIS_PORT";

    let addr_res = get_env_value_from_key(addr_key)?;

    let port_res = get_env_value_from_key(port_key)?;

    let url = format!("redis://{}:{}", addr_res, port_res);

    log::info!("{}", &url);

    Ok(url)
}
