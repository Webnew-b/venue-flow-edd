use std::io::{Error, ErrorKind};
use std::sync::Arc;
use std::time::Duration;

use super::database::config::*;
use log::{info, log, Level};
use sea_orm::{ConnectOptions, Database, DatabaseConnection};

pub(crate) mod config;
pub(crate) mod entities;

pub async fn start_db_connection() -> Result<Arc<DatabaseConnection>, Error> {
    let key_res = get_db_config();

    let key = key_res.map_err(|e| {
        log!(Level::Error, "{}", e.to_string());
        Error::new(
            ErrorKind::InvalidData,
            "The database configurtion seems to have an illegal format",
        )
    })?;

    let mut opt = ConnectOptions::new(key);

    opt.idle_timeout(Duration::from_secs(600))
        .test_before_acquire(true)
        .max_connections(10)
        .min_connections(5)
        .sqlx_logging(true)
        .connect_timeout(Duration::from_secs(30));

    let db_connection = Database::connect(opt)
        .await
        .map_err(|e| Error::new(ErrorKind::ConnectionRefused, e.to_string()))?;

    info!("The database connect successed");

    Ok(Arc::new(db_connection))
}
