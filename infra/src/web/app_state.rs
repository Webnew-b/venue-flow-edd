use std::sync::Arc;

use r2d2::Pool;
use r2d2_redis::RedisConnectionManager;
use sea_orm::DatabaseConnection;
use tokio::sync::Mutex;

use crate::config::get_jwt_secret_key;
use crate::database::start_db_connection;
use crate::infra_error::InfraError;
use crate::repositroy::oss::init_oss_client;
use crate::repositroy::redis::create_redis_connection;
use crate::service::rental_service::RentalService;
use crate::service::user_service::UserService;
use crate::service::util_service::UtilService;
use crate::service::venue_service::VenueService;

#[derive(Clone)]
pub struct AppState {
    pub db: Arc<DatabaseConnection>,
    pub redis: Arc<Mutex<Pool<RedisConnectionManager>>>,
    pub user_service: Arc<UserService>,
    pub venue_service: Arc<VenueService>,
    pub rental_service: Arc<RentalService>,
    pub util_service: Arc<UtilService>,
}

pub(super) async fn create_app_state() -> Result<AppState, InfraError> {
    let db_connection = start_db_connection().await?;
    let redis_connection = create_redis_connection()?;
    let jwt_secert = get_jwt_secret_key()?;

    let venue_service = Arc::new(VenueService::new(db_connection.clone()));
    let rental_service = Arc::new(RentalService::new(db_connection.clone()));
    let user_service = Arc::new(UserService::new(
        db_connection.clone(),
        redis_connection.clone(),
        Arc::new(jwt_secert),
    ));
    let oss_config = init_oss_client().await?;
    let util_service = Arc::new(UtilService::new(oss_config));

    let state = AppState {
        db: db_connection,
        redis: redis_connection,
        venue_service,
        user_service,
        rental_service,
        util_service,
    };

    Ok(state)
}
