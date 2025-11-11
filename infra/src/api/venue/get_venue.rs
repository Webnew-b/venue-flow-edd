use std::ops::Deref;

use actix_web::{get, web, HttpResponse};

use crate::{
    api::{CustomResponse, CustomResponseError},
    web::app_state::AppState,
};

#[get("/get_venue/{id}")]
pub async fn get_venue(
    state: web::Data<AppState>,
    data: web::Path<(i64,)>,
) -> Result<HttpResponse, CustomResponseError> {
    let id = data.0;
    let res = app::use_case::venue::get_venue::get_venue(
        state.venue_service.deref(),
        state.user_service.deref(),
        id,
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
