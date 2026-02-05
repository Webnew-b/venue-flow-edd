use actix_web::http::StatusCode;
use actix_web::middleware::ErrorHandlers;
use actix_web::{web, App, HttpServer};
use tracing_actix_web::{
    DefaultRootSpanBuilder, RootSpanBuilder, TracingLogger,
};

use crate::api::middleware::encrypt::add_service_error_handle;
use crate::api::{self, default_service_handle_error};
use crate::config::get_web_server_config;
use crate::infra_error::InfraError;
use crate::web::app_state::create_app_state;
use crate::web::error_handle::{
    form_error_handle, json_error_handle, path_error_handle, query_error_handle,
};

pub mod app_state;
pub(super) mod error_handle;

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

pub async fn start_web_server() -> anyhow::Result<()> {
    let config = get_web_server_config()?;

    let state = create_app_state().await?;

    let server = HttpServer::new(move || {
        App::new()
            .app_data(json_error_handle())
            .app_data(form_error_handle())
            .app_data(query_error_handle())
            .app_data(path_error_handle())
            .wrap(TracingLogger::<CustomRootSpanBuilder>::new())
            .wrap(
                ErrorHandlers::new()
                    .handler(StatusCode::BAD_REQUEST, add_service_error_handle)
                    .handler(
                        StatusCode::METHOD_NOT_ALLOWED,
                        add_service_error_handle,
                    ),
            )
            .default_service(web::route().to(default_service_handle_error))
            .app_data(web::Data::new(state.clone()))
            .configure(api::api_route)
    })
    .bind(config)
    .map_err(|e| {
        tracing::error!("{}", e);
        InfraError::FailToInitHttpServer {
            message: e.to_string(),
        }
    })?;

    server.run().await?;
    Ok(())
}
