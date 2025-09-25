use std::ops::DerefMut;
use std::sync::Mutex;

use once_cell::sync::Lazy;
use tracing_appender::non_blocking::WorkerGuard;
use tracing_appender::rolling::{RollingFileAppender, Rotation};
use tracing_subscriber::filter::filter_fn;
use tracing_subscriber::layer::SubscriberExt;
pub use tracing_subscriber::EnvFilter;
use tracing_subscriber::Layer;

use crate::config::get_env_type;
use crate::infra_error::InfraError;

static LOG_GUARD: Lazy<Mutex<Option<[WorkerGuard; 2]>>> =
    Lazy::new(|| Mutex::new(None));

fn create_file(file_name: &str) -> RollingFileAppender {
    let file_appender =
        RollingFileAppender::new(Rotation::DAILY, "log", file_name);

    file_appender
}

pub fn init_logger() -> Result<(), InfraError> {
    let env_type = get_env_type().map_err(|e| {
        println!("{}", e.to_string());
        InfraError::FailToConstructLog
    })?;

    match env_type {
        super::config::AppEnv::DEVELOP => {
            tracing_subscriber::fmt()
                .with_env_filter(EnvFilter::from_default_env())
                .init();
            println!("Develop logger loaded successed!");
            Ok(())
        },
        super::config::AppEnv::PRODUCTION => init_production_log(),
    }
}

fn init_production_log() -> Result<(), InfraError> {
    let main_log = create_file("app.log"); //todo 配置问题
    let sql_log = create_file("sql.log");

    let (main_non_b, main_guard) = tracing_appender::non_blocking(main_log);
    let (sql_non_b, sql_guard) = tracing_appender::non_blocking(sql_log);

    let main_sub = tracing_subscriber::fmt::Layer::new()
        .json()
        .with_writer(main_non_b)
        .with_filter(EnvFilter::from_default_env());

    let sql_sub = tracing_subscriber::fmt::Layer::new()
        .json()
        .with_writer(sql_non_b)
        .with_filter(filter_fn(|metadata| metadata.target() == "sqlx::query"));

    let registry = tracing_subscriber::registry().with(main_sub).with(sql_sub);

    let _: () =
        tracing::subscriber::set_global_default(registry).map_err(|e| {
            eprintln!("Unable to set the default logger:{}", e.to_string());
            InfraError::FailToConstructLog
        })?;

    let guard = [main_guard, sql_guard];

    let _ = LOG_GUARD
        .lock()
        .map_err(|e| {
            eprintln!("Unable to save guard,cause:{}", e.to_string());
            InfraError::FailToConstructLog
        })?
        .deref_mut()
        .replace(guard);

    println!("Production logger loaded successed!");

    Ok(())
}
