use std::ops::Deref;

use actix_web::{post, web, HttpRequest, HttpResponse};
use domain::PageLimit;
use serde::Deserialize;

use crate::{
    api::{rental::get_lessor_id, CustomResponse, CustomResponseError},
    web::app_state::AppState,
};

#[derive(Deserialize)]
struct GetRentalList {
    pub page: u64,
    pub limit: u64,
}

#[post("/get_rental_requests")]
pub async fn get_rental_requests(
    state: web::Data<AppState>,
    req: HttpRequest,
    data: web::Query<GetRentalList>,
) -> Result<HttpResponse, CustomResponseError> {
    let page = PageLimit {
        page: data.page,
        limit: data.limit,
    };
    let lessor_id = get_lessor_id(req)?;
    let res = app::use_case::rental::get_rental_requests::get_rental_requests(
        state.rental_service.deref(),
        lessor_id,
        page,
    )
    .await?;

    let res = state.event_system.process_outcome(res).await.map_err(|e| {
        tracing::error!("{}", e);
        CustomResponseError::ServiceError
    })?;

    let res = CustomResponse::success_by_response(Some(res));
    Ok(res)
}
