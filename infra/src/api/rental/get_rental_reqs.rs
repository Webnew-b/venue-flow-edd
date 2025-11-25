use std::ops::Deref;

use actix_web::{post, web, HttpRequest, HttpResponse};

use crate::{
    api::{rental::get_lessor_id, CustomResponse, CustomResponseError},
    web::app_state::AppState,
};

#[post("/get_rental_requests")]
pub async fn get_rental_requests(
    state: web::Data<AppState>,
    req: HttpRequest,
) -> Result<HttpResponse, CustomResponseError> {
    let lessor_id = get_lessor_id(req)?;
    let res = app::use_case::rental::get_rental_requests::get_rental_requests(
        state.rental_service.deref(),
        lessor_id,
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
