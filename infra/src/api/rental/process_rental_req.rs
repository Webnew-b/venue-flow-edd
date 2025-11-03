use std::ops::Deref;

use actix_web::{post, web, HttpResponse};
use serde::{Deserialize, Serialize};

use crate::{
    api::{rental::RentalClock, CustomResponse, CustomResponseError},
    web::app_state::AppState,
};

#[derive(Deserialize, Serialize)]
struct ProcessRentalData {
    pub lessor_id: i64,
    pub rental_id: i64,
}

#[post("/approve_rental_request")]
pub async fn approve_rental_request(
    state: web::Data<AppState>,
    data: web::Json<ProcessRentalData>,
) -> Result<HttpResponse, CustomResponseError> {
    let time = RentalClock;
    let res =
        app::use_case::rental::process_rental_request::approve_rental_request(
            state.rental_service.deref(),
            state.venue_service.deref(),
            state.user_service.deref(),
            data.lessor_id,
            data.rental_id,
            &time,
        )
        .await
        .map_err(|e| {
            tracing::error!("{}", e);
            CustomResponseError::BadRequest(e.to_string())
        })?;

    let _ = state.event_system.process_outcome(res).await.map_err(|e| {
        tracing::error!("{}", e);
        CustomResponseError::ServiceError
    })?;

    let res = CustomResponse::<()>::success_by_response(None);
    Ok(res)
}

#[post("/reject_rental_request")]
pub async fn reject_rental_request(
    state: web::Data<AppState>,
    data: web::Json<ProcessRentalData>,
) -> Result<HttpResponse, CustomResponseError> {
    let time = RentalClock;
    let res =
        app::use_case::rental::process_rental_request::reject_rental_request(
            state.rental_service.deref(),
            state.venue_service.deref(),
            state.user_service.deref(),
            data.lessor_id,
            data.rental_id,
            &time,
        )
        .await
        .map_err(|e| {
            tracing::error!("{}", e);
            CustomResponseError::BadRequest(e.to_string())
        })?;

    let _ = state.event_system.process_outcome(res).await.map_err(|e| {
        tracing::error!("{}", e);
        CustomResponseError::ServiceError
    })?;

    let res = CustomResponse::<()>::success_by_response(None);
    Ok(res)
}
