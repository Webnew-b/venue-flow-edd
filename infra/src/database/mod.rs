use std::sync::Arc;
use std::time::Duration;

use crate::infra_error::InfraError;

use super::database::config::*;
use log::info;
use sea_orm::{ConnectOptions, Database, DatabaseConnection};
use thiserror::Error;

pub(crate) mod config;
pub(crate) mod entities;

#[derive(Debug, Error)]
pub enum DatabaseError {
    #[error("The Database connection be refused.")]
    ConnectionRefused {
        source: Box<dyn std::error::Error + Send + Sync>,
    },

    #[error("Other message:{0}")]
    Other(String),

    #[error("Conld not select the element in database.")]
    SelectFail,

    #[error("Data is not found.")]
    DataNotFound,

    #[error("Failed to save entity.")]
    SaveEntityFail,

    #[error("Failed to delete the entity.")]
    DeleteEntityFail,

    #[error("Fail to select the preant entity.")]
    SelectPreantEntityFail,
}

pub async fn start_db_connection() -> Result<Arc<DatabaseConnection>, InfraError>
{
    let key = get_db_config()?;

    let mut opt = ConnectOptions::new(key);

    opt.idle_timeout(Duration::from_secs(600))
        .test_before_acquire(true)
        .max_connections(10)
        .min_connections(5)
        .sqlx_logging(true)
        .connect_timeout(Duration::from_secs(30));

    let db_connection = Database::connect(opt).await.map_err(|e| {
        DatabaseError::ConnectionRefused {
            source: Box::new(e),
        }
    })?;

    info!("The database connect successed");

    Ok(Arc::new(db_connection))
}
