use std::ops::Deref;

use actix_web::{post, web, HttpResponse};
use serde::{Deserialize, Serialize};

use crate::{
    api::{CustomResponse, CustomResponseError},
    web::app_state::AppState,
};

#[derive(Deserialize, Serialize)]
pub(crate) struct UserProfileData {
    pub id: i64,
}

#[post("/get_user_profile")]
pub async fn get_user_profile(
    state: web::Data<AppState>,
    profile_data: web::Json<UserProfileData>,
) -> Result<HttpResponse, CustomResponseError> {
    let res = app::use_case::user::get_user_detail::get_user_detail(
        profile_data.id,
        state.user_service.deref(),
    )
    .await
    .map_err(|e| -> CustomResponseError {
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
