use std::time::Duration;

use actix_web::middleware::from_fn;
use actix_web::{get, web, HttpResponse, Responder};
use sea_orm::{DbBackend, FromQueryResult, Statement};

use crate::web::http::middleware::encrypt::encrypt_middleware;
use crate::web::http::CustomResponse;
use crate::web::AppState;


pub fn router(cfg: &mut web::ServiceConfig) {
    let m = from_fn(encrypt_middleware);
    cfg.service(
        web::scope("/example")
            .service(example_database)
            .service(example_database2)
            .service(
                web::scope("")
                .wrap(m)
                .service(index)
                )
    );
}

#[get("")]
pub async fn index() -> impl Responder {
    HttpResponse::Ok().body("Hello world")
}

#[derive(Debug, FromQueryResult)]
struct ExampleS {
    test: String,
}

#[get("/database")]
pub async fn example_database(state: web::Data<AppState>) -> impl Responder {
    let rows = ExampleS::find_by_statement(Statement::from_sql_and_values(
        DbBackend::Postgres,
        r#"Select 'aaa' as test"#,
        [],
    ))
    .all(&*state.db)
    .await;

    match rows {
        Ok(r) => {
            tracing::info!("{}", r[0].test);
        },
        Err(e) => {
            tracing::error!("{}", e.to_string());
            return HttpResponse::InternalServerError().body("Server error.");
        },
    }

    tokio::time::sleep(Duration::from_secs(3)).await;

    let res = CustomResponse::<()>::success(None);
    HttpResponse::Ok().json(res)
}

#[get("/database2")]
pub async fn example_database2(state: web::Data<AppState>) -> impl Responder {
    let rows = ExampleS::find_by_statement(Statement::from_sql_and_values(
        DbBackend::Postgres,
        r#"Select 'aaa' as test"#,
        [],
    ))
    .all(&*state.db)
    .await;
    match rows {
        Ok(r) => {
            tracing::info!("{}", r[0].test);
        },
        Err(e) => {
            tracing::error!("{}", e.to_string());
            return HttpResponse::InternalServerError().body("Server error.");
        },
    }

    let res = CustomResponse::<()>::success(None);
    HttpResponse::Ok().json(res)
}


