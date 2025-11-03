use std::ops::Deref;

use actix_web::{post, web, HttpResponse};
use serde::{Deserialize, Serialize};

use crate::{
    api::{CustomResponse, CustomResponseError},
    web::app_state::AppState,
};

#[derive(Deserialize, Serialize)]
struct GetRentalData {
    pub lessor_id: i64,
}

#[post("/get_rental_requests")]
pub async fn get_rental_requests(
    state: web::Data<AppState>,
    data: web::Json<GetRentalData>,
) -> Result<HttpResponse, CustomResponseError> {
    let res = app::use_case::rental::get_rental_requests::get_rental_requests(
        state.rental_service.deref(),
        data.lessor_id,
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
