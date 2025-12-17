use std::ops::Deref;

use actix_web::{post, web, HttpRequest, HttpResponse};
use serde::{Deserialize, Serialize};

use crate::{
    api::{
        venue::{get_lessor_and_user_id, VenueClock},
        CustomResponse, CustomResponseError,
    },
    web::app_state::AppState,
};

#[derive(Deserialize, Serialize)]
struct Data {
    pub venue_id: i64,
}

#[post("/publish_venue")]
pub async fn publish_venue(
    req: HttpRequest,
    state: web::Data<AppState>,
    data: web::Json<Data>,
) -> Result<HttpResponse, CustomResponseError> {
    let (_user_id, lessor_id) = get_lessor_and_user_id(req)?;
    let time = VenueClock;
    let res = app::use_case::venue::manage_venue_status::publish_venue(
        state.venue_service.deref(),
        &time,
        data.venue_id,
        lessor_id,
    )
    .await?;

    let res = state.event_system.process_outcome(res).await.map_err(|e| {
        tracing::error!("{}", e);
        CustomResponseError::ServiceError
    })?;

    let res = CustomResponse::success_by_response(Some(res));
    Ok(res)
}

#[post("/unpublish_venue")]
pub async fn unpublish_venue(
    req: HttpRequest,
    state: web::Data<AppState>,
    data: web::Json<Data>,
) -> Result<HttpResponse, CustomResponseError> {
    let (_user_id, lessor_id) = get_lessor_and_user_id(req)?;
    let time = VenueClock;
    let res = app::use_case::venue::manage_venue_status::unpublish_venue(
        state.venue_service.deref(),
        &time,
        data.venue_id,
        lessor_id,
    )
    .await?;

    let res = state.event_system.process_outcome(res).await.map_err(|e| {
        tracing::error!("{}", e);
        CustomResponseError::ServiceError
    })?;

    let res = CustomResponse::success_by_response(Some(res));
    Ok(res)
}
