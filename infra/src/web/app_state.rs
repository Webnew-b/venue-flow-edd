use std::sync::Arc;

use event::event_service::email_service::EmailService;
use event::EventSystem;
use sea_orm::DatabaseConnection;

use crate::config::get_jwt_secret_key;
use crate::database::start_db_connection;
use crate::infra_error::{InfraError, InfraResult};
use crate::queue::RedisAsyncQueue;
use crate::repositroy::oss::init_oss_client;
use crate::repositroy::redis::create_redis_connection;
use crate::service::rental_service::RentalService;
use crate::service::user_service::UserService;
use crate::service::util_service::UtilService;
use crate::service::venue_service::VenueService;

#[derive(Clone)]
pub struct AppState {
    pub db:             Arc<DatabaseConnection>,
    pub redis:          deadpool_redis::Pool,
    pub user_service:   Arc<UserService>,
    pub venue_service:  Arc<VenueService>,
    pub rental_service: Arc<RentalService>,
    pub event_system:   Arc<EventSystem>,
    pub util_service:   Arc<UtilService>,
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
    let event_system = init_event_handler(redis_connection.clone()).await?;

    let state = AppState {
        db: db_connection,
        redis: redis_connection,
        venue_service,
        user_service,
        rental_service,
        util_service,
        event_system,
    };

    Ok(state)
}

async fn init_event_handler(
    redis: deadpool_redis::Pool,
) -> InfraResult<Arc<EventSystem>> {
    let email_service = Arc::new(EmailService {
        sender: "aaaa".to_string(),
    });
    let queue = RedisAsyncQueue::new(redis)?;
    let event_system = EventSystem::new(Arc::new(queue), email_service)
        .await
        .map_err(|e| InfraError::FailToInitEventSystem {
        message: e.to_string(),
    })?;
    Ok(Arc::new(event_system))
}
