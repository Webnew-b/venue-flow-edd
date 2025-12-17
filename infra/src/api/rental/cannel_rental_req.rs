use std::ops::Deref;

use actix_web::{post, web, HttpRequest, HttpResponse};
use serde::{Deserialize, Serialize};

use crate::{
    api::{
        rental::{get_organizer_id, RentalClock},
        CustomResponse, CustomResponseError,
    },
    web::app_state::AppState,
};

#[derive(Deserialize, Serialize)]
struct CannelRentalData {
    pub rental_id: i64,
}

#[post("/cancel_rental_request")]
pub async fn cancel_rental_request(
    req: HttpRequest,
    state: web::Data<AppState>,
    data: web::Json<CannelRentalData>,
) -> Result<HttpResponse, CustomResponseError> {
    let organizer_id = get_organizer_id(req)?;
    let time = RentalClock;
    let res =
        app::use_case::rental::cancel_rental_request::cancel_rental_request(
            state.rental_service.deref(),
            state.user_service.deref(),
            state.venue_service.deref(),
            organizer_id,
            data.rental_id,
            &time,
        )
        .await?;

    let _ = state.event_system.process_outcome(res).await.map_err(|e| {
        tracing::error!("{}", e);
        CustomResponseError::ServiceError
    })?;

    let res = CustomResponse::<()>::success_by_response(None);
    Ok(res)
}
