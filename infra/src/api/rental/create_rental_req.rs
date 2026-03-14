use std::ops::Deref;

use actix_web::{post, web, HttpRequest, HttpResponse};
use app::commands::rental_commands::CreateRentalCommand;
use chrono::{DateTime, Utc};
use domain_core::rental::ActivityType;
use serde::{Deserialize, Serialize};

use crate::{
    api::{
        rental::{get_organizer_id, RentalClock},
        CustomResponse, CustomResponseError,
    },
    web::app_state::AppState,
};

#[derive(Deserialize, Serialize)]
struct CreateRentalData {
    pub venue_id:         i64,
    pub start_time:       DateTime<Utc>,
    pub end_time:         DateTime<Utc>,
    pub activity_type:    String,
    pub request_comments: Option<String>,
}

fn get_activity_type(i: &str) -> Result<ActivityType, CustomResponseError> {
    match i {
        "all" => Ok(ActivityType::All),
        "exhibition" => Ok(ActivityType::Exhibition),
        "seminar" => Ok(ActivityType::Seminar),
        _ => Err(CustomResponseError::BadRequest(
            "The activity type is wrong".to_string(),
        )),
    }
}
fn get_command(
    l: CreateRentalData,
    o_id: i64,
) -> Result<CreateRentalCommand, CustomResponseError> {
    let res = CreateRentalCommand {
        venue_id:         l.venue_id,
        organizer_id:     o_id,
        start_time:       l.start_time,
        end_time:         l.end_time,
        activity_type:    get_activity_type(l.activity_type.as_str())?,
        request_comments: l.request_comments,
    };
    Ok(res)
}

#[post("/create_rental_req")]
pub async fn create_rental_req(
    req: HttpRequest,
    state: web::Data<AppState>,
    create_rental_data: web::Json<CreateRentalData>,
) -> Result<HttpResponse, CustomResponseError> {
    let organizer_id = get_organizer_id(req)?;
    let time = RentalClock;
    let command = get_command(create_rental_data.into_inner(), organizer_id)?;
    let res =
        app::use_case::rental::create_rental_request::create_rental_request(
            state.rental_service.deref(),
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
