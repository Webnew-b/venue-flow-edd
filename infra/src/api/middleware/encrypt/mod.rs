
use actix_web::body::BoxBody;
use actix_web::dev::{ServiceRequest, ServiceResponse};
use actix_web::middleware::{ErrorHandlerResponse, Next};
use actix_web::{Error, HttpResponse};
use log::info;

use crate::api::response_code::CodeEnum;
use crate::api::CustomResponse;


pub async fn encrypt_middleware(
    req:ServiceRequest,
    next:Next<BoxBody>
) -> Result<ServiceResponse<BoxBody>,Error> {
    let header = req.headers();

    let auth = header.get("X-User-id");

    match auth {
        Some(e) => {
            info!("{:?}",e);
        },
        None => {
            let c_res = CustomResponse::<()>::new("Bad request", CodeEnum::BadRequest, None);
            let http_res = HttpResponse::BadRequest()
                .json(c_res);
            let service = ServiceResponse::new(
                req.request().clone(),
                http_res.map_into_boxed_body()
                );
            
            return Ok(service)
            //return Err(actix_web::error::ErrorBadRequest("Access denied."));
        }
    };

    info!("you request this middleware.");
    next.call(req).await
}


pub fn add_service_error_handle<B>(res:actix_web::dev::ServiceResponse<B>)
    -> actix_web::Result<ErrorHandlerResponse<B>>
{
    let c_res = CustomResponse::<()>::new("Bad request", CodeEnum::BadRequest, None);
    let http_res = HttpResponse::BadRequest()
        .json(c_res);
    Ok(ErrorHandlerResponse::Response(
            ServiceResponse::new(
                res.request().clone(), 
                http_res.map_into_right_body()
                )
            )
        )
}
