use std::io::{Error, ErrorKind};
use std::sync::Arc;

use actix_web::http::StatusCode;
use actix_web::middleware::ErrorHandlers;
use actix_web::{web, App, HttpServer, };
use log::{log, Level};
use r2d2::Pool;
use r2d2_redis::RedisConnectionManager;
use sea_orm::DatabaseConnection;
use tokio::sync::Mutex;
use tracing_actix_web::{
    DefaultRootSpanBuilder, RootSpanBuilder, TracingLogger,
};

use crate::web::http::default_service_handle_error;
use crate::web::http::middleware::encrypt::add_service_error_handle;

use super::config::config::get_web_server_config;
use super::database::start_db_connection;
//use super::oss::{self, OssClientConfig};
use super::redis::create_redis_connection;


pub mod http;

#[derive(Clone)]
pub struct AppState {
    pub db: Arc<DatabaseConnection>,
    pub redis: Arc<Mutex<Pool<RedisConnectionManager>>>,
    // pub oss:Arc<OssClientConfig>,
}

#[derive(Clone)]
struct CustomRootSpanBuilder;

impl RootSpanBuilder for CustomRootSpanBuilder {
    fn on_request_start(
        request: &actix_web::dev::ServiceRequest,
    ) -> tracing::Span {
        let n_headers = request.headers().len();
        let span = tracing_actix_web::root_span!(
            level = tracing::Level::INFO,
            request,
            method = %request.method().to_string(),
            path = %request.path().to_string(),
            n_headers
        );
        tracing::info!(parent:&span,"Http Request");
        span
    }
    fn on_request_end<B: actix_web::body::MessageBody>(
        span: tracing::Span,
        outcome: &Result<actix_web::dev::ServiceResponse<B>, actix_web::Error>,
    ) {
        DefaultRootSpanBuilder::on_request_end(span, outcome);
    }
}

async fn create_app_state() -> Result<AppState, Error> {
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

pub async fn start_web_server() -> std::io::Result<()> {
    let config_res = get_web_server_config();

    let config = match config_res {
        Ok(e) => e,
        Err(e) => {
            log!(Level::Error, "{}", e.to_string());
            return Err(std::io::Error::new(
                ErrorKind::InvalidData,
                "The configurtion seem has a illegally format.",
            ));
        },
    };

    let state = create_app_state().await?;

    let server = HttpServer::new(move || {
        App::new()
            .wrap(TracingLogger::<CustomRootSpanBuilder>::new())
            .wrap(
                ErrorHandlers::new().handler(
                    StatusCode::BAD_REQUEST, 
                    add_service_error_handle)
                )
            .default_service(web::route().to(default_service_handle_error))
            .app_data(web::Data::new(state.clone()))
            .configure(http::example::router)
    })
    .bind(config)
    .map_err(|e| {
        log!(Level::Error, "{}", e.to_string());
        Error::new(ErrorKind::AddrNotAvailable, "Binding the server addr fail.")
    })?;

    server.run().await
}
