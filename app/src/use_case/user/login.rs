use domain::domain_error::domain_user_error::DomainUserError;
use domain::domain_error::DomainError;
use domain::user_domain::{UserGenerator, UserRepository, UserValidation};
use domain::util_trait::PasswordHasher;
use garde::Validate;

use crate::app_error::user_error::AppUserError;
use crate::app_error::{AppError, AppResult};
use crate::commands::user_commands::{
    LoginUserCommand, LoginedRes, UserLoginType,
};
use crate::{AppUseCase, Outcome};

async fn valid_login_type(
    validator: &impl UserValidation,
    login_type: &UserLoginType,
) -> AppResult<()> {
    match &login_type {
        UserLoginType::Email(e) => {
            e.validate().map_err(|_| AppUserError::EmailIllegal)?;
            validator.exist_email(&e.address).await?;
        },
        UserLoginType::UserName(u) => {
            validator.valid_username(u).await?;
        },
    }
    Ok(())
}

pub async fn login_user(
    repo: &impl UserRepository,
    validator: &impl UserValidation,
    generator: &impl UserGenerator,
    password_hash: &impl PasswordHasher,
    info: LoginUserCommand,
) -> AppResult<Outcome<LoginedRes>> {
    let login_type = info.login_type.clone();
    let password = info.password.clone();

    valid_login_type(validator, &login_type).await?;

    let user = repo.find_user_by_name(info.into()).await.map_err(|e| {
        let error: AppError = match e {
            DomainError::DomainUserError(DomainUserError::UserNotFound) => {
                AppUserError::LoginIncrrect("email".to_string()).into()
            },
            other => other.into(),
        };
        error
    })?;

    password_hash.verify(password.as_str(), user.password())?;

    let token = generator.generate_token(&user).await?.token;

    let id = user.id().ok_or(AppError::IdInexisted("user".to_string()))?;
    let res = LoginedRes {
        id,
        username: user.username().to_string(),
        token,
    };
    Ok(Outcome::new(res, AppUseCase::UserLogin))
}
