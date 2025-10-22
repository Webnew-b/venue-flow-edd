use std::ops::Deref;

use actix_web::{post, web, HttpResponse};
use app::commands::user_commands::{Email, LoginUserCommand, UserLoginType};
use garde::Validate;
use serde::{Deserialize, Serialize};

use crate::{
    api::{CustomResponse, CustomResponseError},
    web::app_state::AppState,
};

#[derive(Deserialize, Serialize)]
pub(crate) enum LoginType {
    Email,
    Username,
}

#[derive(Deserialize, Serialize)]
pub(crate) struct LoginData {
    pub identifier: String,
    pub password: String,
    pub login_type: LoginType,
}

fn get_login_command(
    l: LoginData,
) -> Result<LoginUserCommand, CustomResponseError> {
    let res = match l.login_type {
        LoginType::Email => {
            let email = Email {
                address: l.identifier,
            };

            email
                .validate()
                .map_err(|e| CustomResponseError::BadRequest(e.to_string()))?;

            LoginUserCommand {
                login_type: UserLoginType::Email(email),
                password: l.password,
            }
        },
        LoginType::Username => LoginUserCommand {
            login_type: UserLoginType::UserName(l.identifier),
            password: l.password,
        },
    };
    Ok(res)
}

#[post("/login")]
pub async fn login(
    state: web::Data<AppState>,
    login_data: web::Json<LoginData>,
) -> Result<HttpResponse, CustomResponseError> {
    let command = get_login_command(login_data.into_inner())?;
    let res = app::use_case::user::login::login_user(
        state.user_service.deref(),
        state.user_service.deref(),
        state.user_service.deref(),
        state.util_service.deref(),
        command,
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
