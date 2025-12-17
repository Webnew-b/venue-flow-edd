use std::ops::Deref;

use actix_web::{post, web, HttpRequest, HttpResponse};
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

use crate::{
    api::{
        rental::{get_organizer_id, RentalClock},
        CustomResponse, CustomResponseError,
    },
    web::app_state::AppState,
};

#[derive(Deserialize, Serialize)]
struct CreateRentalData {
    pub rental_id: i64,
    pub start_time: DateTime<Utc>,
    pub end_time: DateTime<Utc>,
}

#[post("/update_rental_time")]
pub async fn update_rental_time(
    req: HttpRequest,
    state: web::Data<AppState>,
    data: web::Json<CreateRentalData>,
) -> Result<HttpResponse, CustomResponseError> {
    let organizer_id = get_organizer_id(req)?;
    let time = RentalClock;
    let res = app::use_case::rental::update_rental_time::update_rental_time(
        state.rental_service.deref(),
        organizer_id,
        (data.start_time, data.end_time),
        data.rental_id,
        &time,
    )
    .await?;

    let res = state.event_system.process_outcome(res).await.map_err(|e| {
        tracing::error!("{}", e);
        CustomResponseError::ServiceError
    })?;

    let res = CustomResponse::success_by_response(Some(res));
    Ok(res)
}
