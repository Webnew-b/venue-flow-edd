use std::ops::Deref;

use actix_multipart::form::{tempfile::TempFile, text::Text, MultipartForm};
use actix_web::{post, web, HttpRequest, HttpResponse};
use app::commands::venue_commands::{
    ImageDeleteCommand, ImageUploadCommand, UpdateVenueCommand,
};
use serde::{Deserialize, Serialize};

use crate::{
    api::{
        upload_image,
        venue::{get_lessor_and_user_id, VenueClock},
        CustomResponse, CustomResponseError,
    },
    web::app_state::AppState,
};

#[derive(Deserialize, Serialize)]
pub(crate) struct VenueData {
    pub id: i64,
    pub name: Option<String>,
    pub address: Option<String>,
    pub capacity: Option<i32>,
    pub description: Option<String>,
}

#[post("/update_venue")]
pub async fn update_venue(
    req: HttpRequest,
    state: web::Data<AppState>,
    data: web::Json<VenueData>,
) -> Result<HttpResponse, CustomResponseError> {
    let (_user_id, lessor_id) = get_lessor_and_user_id(req)?;
    let time = VenueClock;
    let command = UpdateVenueCommand {
        id: data.id,
        name: data.name.clone(),
        address: data.address.clone(),
        capacity: data.capacity,
        description: data.description.clone(),
        lessor_id,
    };
    let res = app::use_case::venue::update_venue::update_venue(
        state.venue_service.deref(),
        command,
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

#[derive(Debug, MultipartForm)]
struct Upload {
    pub venue_id: Text<i64>,
    pub title: Text<String>,
    pub comment: Text<Option<String>>,
    #[multipart(limit = "10MB")]
    pub file: TempFile,
}

#[post("/update_venue_image")]
pub async fn update_venue_image(
    req: HttpRequest,
    state: web::Data<AppState>,
    MultipartForm(form): MultipartForm<Upload>,
) -> Result<HttpResponse, CustomResponseError> {
    let (_user_id, lessor_id) = get_lessor_and_user_id(req)?;
    let time = VenueClock;

    let temp_path = state.util_service.deref().get_temp_folder();
    let image = upload_image(temp_path, form.file)?;
    let command = ImageUploadCommand {
        lessor_id,
        venue_id: form.venue_id.0,
        title: form.title.0,
        image,
        comment: form.comment.0,
    };
    let res = app::use_case::venue::update_venue::upload_image(
        state.venue_service.deref(),
        state.util_service.deref(),
        &time,
        command,
    )
    .await?;

    let res = state.event_system.process_outcome(res).await.map_err(|e| {
        tracing::error!("{}", e);
        CustomResponseError::ServiceError
    })?;

    let res = CustomResponse::success_by_response(Some(res));
    Ok(res)
}

#[derive(Deserialize, Serialize)]
struct VenueImageData {
    pub image_id: Vec<i64>,
    pub venue_id: i64,
}

#[post("/delete_venue_image")]
pub async fn delete_venue_image(
    req: HttpRequest,
    state: web::Data<AppState>,
    data: web::Json<VenueImageData>,
) -> Result<HttpResponse, CustomResponseError> {
    let (_user_id, lessor_id) = get_lessor_and_user_id(req)?;
    let command = ImageDeleteCommand {
        image_id: data.image_id.clone(),
        venue_id: data.venue_id,
        lessor_id,
    };
    let res = app::use_case::venue::update_venue::delete_image(
        state.venue_service.deref(),
        command,
    )
    .await?;

    let _ = state.event_system.process_outcome(res).await.map_err(|e| {
        tracing::error!("{}", e);
        CustomResponseError::ServiceError
    })?;

    let res = CustomResponse::<()>::success_by_response(None);
    Ok(res)
}
