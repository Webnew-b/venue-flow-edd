use std::io::{Error, ErrorKind};

use actix_web::http::StatusCode;
use actix_web::middleware::ErrorHandlers;
use actix_web::{web, App, HttpServer, };
use log::{log, Level};
use tracing_actix_web::{
    DefaultRootSpanBuilder, RootSpanBuilder, TracingLogger,
};

use crate::api::{self, default_service_handle_error};
use crate::api::middleware::encrypt::add_service_error_handle;
use crate::web::app_state::create_app_state;

use super::config::config::get_web_server_config;
//use super::oss::{self, OssClientConfig};

pub mod app_state;



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
            .configure(api::example::router)
    })
    .bind(config)
    .map_err(|e| {
        log!(Level::Error, "{}", e.to_string());
        Error::new(ErrorKind::AddrNotAvailable, "Binding the server addr fail.")
    })?;

    server.run().await
}
