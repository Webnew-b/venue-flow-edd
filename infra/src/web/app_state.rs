use std::sync::Arc;

use r2d2::Pool;
use r2d2_redis::RedisConnectionManager;
use sea_orm::DatabaseConnection;
use tokio::sync::Mutex;

use crate::database::start_db_connection;
use crate::repositroy::redis::create_redis_connection;

#[derive(Clone)]
pub struct AppState {
    pub db: Arc<DatabaseConnection>,
    pub redis: Arc<Mutex<Pool<RedisConnectionManager>>>,
    // pub oss:Arc<OssClientConfig>,
}

pub(super) async fn create_app_state() -> Result<AppState, std::io::Error> {
    let db_connection = start_db_connection().await?;
    let redis_connection = create_redis_connection()?;
    // let oss_client = oss::init_oss_client().await?;

    let state = AppState {
        db: db_connection,
        redis: redis_connection,
        //oss:oss_client
    };

    Ok(state)
}
