use std::ops::Deref;

use actix_multipart::form::{tempfile::TempFile, MultipartForm};
use actix_web::{post, web, HttpResponse};
use domain::util_trait::ImageRepository;

use crate::{
    api::{upload_image, CustomResponse, CustomResponseError},
    web::app_state::AppState,
};

#[derive(Debug, MultipartForm)]
struct Upload {
    #[multipart(limit = "10MB")]
    pub file: Vec<TempFile>,
}

#[post("/upload_venue_image")]
pub async fn upload_venue_image(
    state: web::Data<AppState>,
    MultipartForm(form): MultipartForm<Upload>,
) -> Result<HttpResponse, CustomResponseError> {
    let mut save_path = vec![];
    let temp_path = state.util_service.deref().get_temp_folder();
    for i in form.file {
        let path = upload_image(temp_path, i)?;
        save_path.push(path);
    }
    let res = state
        .util_service
        .deref()
        .upload_images(save_path)
        .await
        .map_err(|e| {
            tracing::error!("{}", e);
            CustomResponseError::BadRequest(e.to_string())
        })?;

    let res = CustomResponse::success_by_response(Some(res));
    Ok(res)
}
