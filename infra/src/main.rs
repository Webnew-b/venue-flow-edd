use infra_lib::local_log::init_logger;
use infra_lib::web::start_web_server;
use tracing::{event, Level};

#[actix_web::main]
async fn main() -> anyhow::Result<()> {
    init_logger()?;
    event!(Level::INFO, "Server is running in :8080");
    println!("Server is starting");
    start_web_server().await
}
