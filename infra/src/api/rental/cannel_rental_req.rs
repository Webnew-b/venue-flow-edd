use std::ops::Deref;

use actix_web::{post, web, HttpResponse};
use serde::{Deserialize, Serialize};

use crate::{
    api::{rental::RentalClock, CustomResponse, CustomResponseError},
    web::app_state::AppState,
};

#[derive(Deserialize, Serialize)]
struct CannelRentalData {
    pub organizer_id: i64,
    pub rental_id: i64,
}

#[post("/cancel_rental_request")]
pub async fn cancel_rental_request(
    state: web::Data<AppState>,
    data: web::Json<CannelRentalData>,
) -> Result<HttpResponse, CustomResponseError> {
    let time = RentalClock;
    let res =
        app::use_case::rental::cancel_rental_request::cancel_rental_request(
            state.rental_service.deref(),
            state.user_service.deref(),
            state.venue_service.deref(),
            data.organizer_id,
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
