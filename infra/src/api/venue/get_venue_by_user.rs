use std::ops::Deref;

use actix_web::{post, web, HttpRequest, HttpResponse};
use domain::PageLimit;
use serde::{Deserialize, Serialize};

use crate::{
    api::{user::get_user_id, CustomResponse, CustomResponseError},
    web::app_state::AppState,
};

#[derive(Deserialize, Serialize)]
struct GetVenueData {
    pub page:  u64,
    pub limit: u64,
}

#[post("/get_venue_by_user")]
pub async fn get_venue_by_user(
    state: web::Data<AppState>,
    data: web::Json<GetVenueData>,
    req: HttpRequest,
) -> Result<HttpResponse, CustomResponseError> {
    let user_id = get_user_id(req)?;
    let page = PageLimit {
        page:  data.page,
        limit: data.limit,
    };
    let res = app::use_case::venue::get_venue_by_user::get_venue_by_user(
        state.venue_service.deref(),
        state.user_service.deref(),
        page,
        user_id,
    )
    .await?;

    let res = state.event_system.process_outcome(res).await.map_err(|e| {
        tracing::error!("{}", e);
        CustomResponseError::ServiceError
    })?;

    let res = CustomResponse::success_by_response(Some(res));
    Ok(res)
}
