use std::ops::Deref;

use actix_web::{get, web, HttpResponse};
use domain::PageLimit;
use serde::Deserialize;

use crate::{
    api::{CustomResponse, CustomResponseError},
    web::app_state::AppState,
};

#[derive(Deserialize)]
struct GetVenueList {
    pub page: u64,
    pub limit: u64,
}

#[get("/get_venue_list")]
pub async fn get_venue_list(
    state: web::Data<AppState>,
    data: web::Query<GetVenueList>,
) -> Result<HttpResponse, CustomResponseError> {
    let page = PageLimit {
        page: data.page,
        limit: data.limit,
    };
    let res = app::use_case::venue::get_venue_list::get_venue_list(
        state.venue_service.deref(),
        page,
    )
    .await
    .map_err(|e| {
        tracing::error!("{}", e);
        CustomResponseError::BadRequest(e.to_string())
    })?;

    let res = state.event_system.process_outcome(res).await.map_err(|e| {
        tracing::error!("{}", e);
        CustomResponseError::ServiceError
    })?;

    let res = CustomResponse::success_by_response(Some(res));
    Ok(res)
}
