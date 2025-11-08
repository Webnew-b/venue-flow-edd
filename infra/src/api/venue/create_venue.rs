use std::ops::Deref;

use actix_web::{post, web, HttpResponse};
use app::commands::venue_commands::{CreateVenueCommand, VenueImageCommand};
use serde::{Deserialize, Serialize};

use crate::{
    api::{venue::VenueClock, CustomResponse, CustomResponseError},
    web::app_state::AppState,
};

#[derive(Deserialize, Serialize)]
pub(crate) struct ImageData {
    pub title: String,
    pub image: String,
    pub comment: Option<String>,
}

#[derive(Deserialize, Serialize)]
pub(crate) struct VenueData {
    pub user_id: i64,
    pub name: String,
    pub address: String,
    pub images: Vec<ImageData>,
    pub capacity: i32,
    pub description: String,
}

#[post("/create_venue")]
pub async fn create_venue(
    state: web::Data<AppState>,
    data: web::Json<VenueData>,
) -> Result<HttpResponse, CustomResponseError> {
    let time = VenueClock;
    let images = data
        .images
        .iter()
        .map(|e| VenueImageCommand {
            title: e.title.clone(),
            image: e.image.clone(),
            comment: e.comment.clone(),
        })
        .collect();
    let command = CreateVenueCommand {
        user_id: data.user_id,
        name: data.name.clone(),
        address: data.address.clone(),
        images: images,
        capacity: data.capacity,
        description: data.description.clone(),
    };
    let res = app::use_case::venue::create_venue::create_venue(
        state.user_service.deref(),
        state.venue_service.deref(),
        command,
        &time,
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
