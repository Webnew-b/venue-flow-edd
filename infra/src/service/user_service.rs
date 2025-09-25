use std::ops::Deref;
use std::sync::Arc;

use crate::database::entities::lessor as LessorCrate;
use crate::database::entities::organizer as OrganizerCrate;
use crate::database::entities::user::{
    self as UserCrate, ActiveModel as UserModel, Entity as UserEntity,
    Model as UserReadModel,
};
use crate::database::DatabaseError;
use crate::infra_error::InfraError;
use crate::service::user_service::enum_converstion::{
    user_gender_to_db, user_gender_to_domain, user_status_to_db,
    user_status_to_domain,
};

use async_trait::async_trait;
use chrono::Duration;
use chrono::Utc;
use domain::domain_error::domain_user_error::DomainUserError;
use domain::domain_error::DomainError;
use domain::user_domain::user_dto::UserLoginToken;
use domain::user_domain::user_dto::{UserLoginEnum, UserLoginName};
use domain::user_domain::UserGenerator;
use domain::user_domain::UserRepository;
use domain::user_domain::UserValidation;
use domain_core::user::lessor::Lessor;
use domain_core::user::lessor::LessorBuilder;
use domain_core::user::organizer::{Organizer, OrganizerBuilder};
use domain_core::user::{User, UserBuilder};
use jsonwebtoken::{encode, Algorithm, EncodingKey, Header};
use r2d2::Pool;
use r2d2_redis::RedisConnectionManager;
use sea_orm::ActiveValue::{NotSet, Set};
use sea_orm::{
    ActiveModelTrait, ColumnTrait, DatabaseConnection, EntityTrait, ModelTrait,
    QueryFilter,
};
use serde::{Deserialize, Serialize};
use tokio::sync::Mutex;

pub mod enum_converstion;

pub(crate) fn domain_user_to_db_user(user: User) -> UserModel {
    let id = match user.id().clone() {
        Some(i) => Set(i),
        None => NotSet,
    };
    let user_gender = user.gender().clone();
    let user_status = user.status().clone();

    UserModel {
        id: id,
        username: Set(user.username().clone()),
        password: Set(user.password().to_string()),
        email: Set(user.email().clone()),
        avatar: Set(user.avatar().clone()),
        gender: Set(user_gender_to_db(user_gender)),
        introduce: Set(user.introduce().clone()),
        is_show: Set(user.is_show().clone()),
        is_delete: Set(user.is_delete().clone()),
        status: Set(user_status_to_db(user_status)),
        createtime: Set(user.createtime().clone().naive_utc()),
        updatetime: Set(user.updatetime().clone().naive_utc()),
    }
}

pub(crate) fn db_user_to_domain_user(
    user: UserReadModel,
) -> Result<User, DomainError> {
    let builder = UserBuilder::default();
    let gender = user_gender_to_domain(user.gender);
    let status = user_status_to_domain(user.status);

    let domain_user = builder
        .id(Some(user.id))
        .username(user.username)
        .password(user.password)
        .avatar(user.avatar)
        .gender(gender)
        .email(user.email)
        .introduce(user.introduce)
        .is_show(user.is_show)
        .is_delete(user.is_delete)
        .status(status)
        .createtime(user.createtime.and_utc())
        .updatetime(user.updatetime.and_utc());
    let domain_user = domain_user.build().map_err(|e| {
        log::error!("{}", e);
        DomainUserError::InvalidUserContstruction
    })?;
    Ok(domain_user)
}

pub(crate) fn db_organizer_to_domain(
    user: User,
    organizer: OrganizerCrate::Model,
) -> Result<Organizer, DomainError> {
    let bulider = OrganizerBuilder::default();
    let organizer = bulider
        .id(Some(organizer.id))
        .user(user)
        .phone(organizer.phone)
        .is_delete(organizer.is_delete)
        .createtime(organizer.createtime.and_utc())
        .updatetime(organizer.updatetime.and_utc());
    let organizer = organizer.build().map_err(|e| {
        log::error!("{}", e);
        DomainUserError::InvalidOrganizerContstruction
    })?;
    Ok(organizer)
}

pub(crate) fn db_lessor_to_domain(
    user: User,
    lessor: LessorCrate::Model,
) -> Result<Lessor, DomainError> {
    let bulider = LessorBuilder::default();
    let lessor = bulider
        .id(Some(lessor.id))
        .user(user)
        .phone(lessor.phone)
        .is_delete(lessor.is_delete)
        .createtime(lessor.createtime.and_utc())
        .updatetime(lessor.updatetime.and_utc());
    let lessor = lessor.build().map_err(|e| {
        log::error!("{}", e);
        DomainUserError::InvalidLessorContstruction
    })?;
    Ok(lessor)
}

pub(crate) struct UserService {
    database: Arc<DatabaseConnection>,
    redis: Arc<Mutex<Pool<RedisConnectionManager>>>,
    jwt_secret: Arc<String>,
}

#[derive(Debug, Serialize, Deserialize)]
struct Claims {
    sub: String,
    company: String,
    exp: usize,
}

impl UserService {
    async fn get_user_by_lessor(
        &self,
        lessor: Option<LessorCrate::Model>,
    ) -> Result<Option<Lessor>, DomainError> {
        match lessor {
            Some(o) => {
                let user = o
                    .find_related(UserEntity)
                    .one(self.database.deref())
                    .await
                    .map_err(|e| {
                        log::error!("{}", e);
                        InfraError::DatabaseError(DatabaseError::SelectFail)
                    })?;
                let user = user.ok_or(InfraError::DatabaseError(
                    DatabaseError::SelectPreantEntityFail,
                ))?;
                let user = db_user_to_domain_user(user)?;
                let res = db_lessor_to_domain(user, o)?;
                Ok(Some(res))
            },
            None => Ok(None),
        }
    }

    async fn get_user_by_organizer(
        &self,
        organizer: Option<OrganizerCrate::Model>,
    ) -> Result<Option<Organizer>, DomainError> {
        match organizer {
            Some(o) => {
                let user = o
                    .find_related(UserEntity)
                    .one(self.database.deref())
                    .await
                    .map_err(|e| {
                        log::error!("{}", e);
                        InfraError::DatabaseError(DatabaseError::SelectFail)
                    })?;
                let user = user.ok_or(InfraError::DatabaseError(
                    DatabaseError::SelectPreantEntityFail,
                ))?;
                let user = db_user_to_domain_user(user)?;
                let res = db_organizer_to_domain(user, o)?;
                Ok(Some(res))
            },
            None => Ok(None),
        }
    }
}

impl UserGenerator for UserService {
    fn generate_token(
        &self,
        user: &User,
    ) -> Result<UserLoginToken, DomainError> {
        let id = user.id().expect("The user id must be exsited.");
        let claims = Claims {
            sub: id.to_string(),
            company: "lexon".to_owned(),
            exp: (Utc::now() + Duration::hours(2)).timestamp() as usize,
        };

        let header = Header::new(Algorithm::HS256);

        let token = encode(
            &header,
            &claims,
            &EncodingKey::from_secret(self.jwt_secret.as_str().as_bytes()),
        )
        .map_err(|e| {
            log::error!("{}", e);
            DomainUserError::InvalidTokenGeneration
        })?;
        Ok(UserLoginToken { token })
    }
}

#[async_trait]
impl UserValidation for UserService {
    async fn valid_email(&self, email: &str) -> Result<(), DomainError> {
        // todo validate the email format and something else.
        self.exist_email(email).await?;
        Ok(())
    }
    async fn valid_username(&self, username: &str) -> Result<(), DomainError> {
        let username = UserEntity::find()
            .filter(UserCrate::Column::Username.eq(username))
            .one(self.database.deref())
            .await
            .map_err(|e| {
                log::error!("{}", e);
                InfraError::DatabaseError(DatabaseError::SelectFail)
            })?;
        username.ok_or(DomainUserError::EmailNotFound)?;
        Ok(())
    }
    async fn exist_email(&self, email: &str) -> Result<(), DomainError> {
        let email = UserEntity::find()
            .filter(UserCrate::Column::Email.eq(email))
            .one(self.database.deref())
            .await
            .map_err(|e| {
                log::error!("{}", e);
                InfraError::DatabaseError(DatabaseError::SelectFail)
            })?;
        email.ok_or(DomainUserError::EmailNotFound)?;
        Ok(())
    }
}

#[async_trait]
impl UserRepository for UserService {
    async fn find_user_by_id(&self, id: i64) -> Result<User, DomainError> {
        let user = UserEntity::find_by_id(id)
            .one(&*self.database)
            .await
            .map_err(|e| {
                log::error!("find user by {} id is failure cause:{}", id, e);
                InfraError::DatabaseError(DatabaseError::SelectFail)
            })?;
        let user =
            user.ok_or(InfraError::DatabaseError(DatabaseError::DataNotFound))?;
        db_user_to_domain_user(user)
    }

    async fn find_user_by_name(
        &self,
        login: UserLoginName,
    ) -> Result<User, DomainError> {
        let expr = match login.login_type {
            UserLoginEnum::UserName(u) => UserCrate::Column::Username.eq(u),
            UserLoginEnum::Email(e) => UserCrate::Column::Email.eq(e),
        };
        let user = UserEntity::find()
            .filter(expr)
            .one(&*self.database)
            .await
            .map_err(|e| {
            log::error!("find user by login info is failure,casuse:{}", e);
            InfraError::DatabaseError(DatabaseError::SelectFail)
        })?;
        let user =
            user.ok_or(InfraError::DatabaseError(DatabaseError::DataNotFound))?;
        db_user_to_domain_user(user)
    }

    async fn save_user(self: &Self, user: User) -> Result<(), DomainError> {
        let user = domain_user_to_db_user(user);
        user.save(self.database.deref()).await.map_err(|e| {
            log::error!("{}", e);
            InfraError::DatabaseError(DatabaseError::SaveEntityFail)
        })?;
        Ok(())
    }

    async fn create_user(self: &Self, user: User) -> Result<User, DomainError> {
        let user_model = domain_user_to_db_user(user.clone());
        let user_model: UserReadModel = user_model
            .insert(self.database.deref())
            .await
            .map_err(|e| {
                log::error!("{}", e);
                InfraError::DatabaseError(DatabaseError::SaveEntityFail)
            })?;

        let user = user.update_id(user_model.id);
        Ok(user)
    }

    async fn delete_user(self: &Self, id: i64) -> Result<(), DomainError> {
        let _ = UserEntity::delete_by_id(id)
            .exec(self.database.deref())
            .await
            .map_err(|e| {
                log::error!("{}", e);
                InfraError::DatabaseError(DatabaseError::DeleteEntityFail)
            })?;
        Ok(())
    }

    async fn logout(self: &Self, _token: String) -> Result<(), DomainError> {
        Ok(())
    }

    async fn find_user_has_organizer_role(
        &self,
        user_id: i64,
    ) -> Result<Option<Organizer>, DomainError> {
        let organizer = OrganizerCrate::Entity::find()
            .filter(OrganizerCrate::Column::UserId.eq(user_id))
            .one(self.database.deref())
            .await
            .map_err(|e| {
                log::error!("{}", e);
                InfraError::DatabaseError(DatabaseError::SelectFail)
            })?;
        self.get_user_by_organizer(organizer).await
    }

    async fn find_user_has_lessor_role(
        &self,
        user_id: i64,
    ) -> Result<Option<Lessor>, DomainError> {
        let lessor = LessorCrate::Entity::find()
            .filter(LessorCrate::Column::UserId.eq(user_id))
            .one(self.database.deref())
            .await
            .map_err(|e| {
                log::error!("{}", e);
                InfraError::DatabaseError(DatabaseError::SelectFail)
            })?;
        self.get_user_by_lessor(lessor).await
    }

    async fn find_organizer_by_user_id(
        &self,
        user_id: i64,
    ) -> Result<Organizer, DomainError> {
        let res = self.find_user_has_organizer_role(user_id).await?;
        let res =
            res.ok_or(InfraError::DatabaseError(DatabaseError::DataNotFound))?;
        Ok(res)
    }

    async fn find_lessor_by_user_id(
        &self,
        user_id: i64,
    ) -> Result<Lessor, DomainError> {
        let res = self.find_user_has_lessor_role(user_id).await?;
        res.ok_or(InfraError::DatabaseError(DatabaseError::DataNotFound).into())
    }

    async fn find_lessor_by_id(&self, id: i64) -> Result<Lessor, DomainError> {
        let lessor = LessorCrate::Entity::find_by_id(id)
            .one(self.database.deref())
            .await
            .map_err(|e| {
                log::error!("{}", e);
                InfraError::DatabaseError(DatabaseError::SelectFail)
            })?;

        self.get_user_by_lessor(lessor).await?.ok_or(
            InfraError::DatabaseError(DatabaseError::DataNotFound.into())
                .into(),
        )
    }

    async fn find_organizer_by_id(
        &self,
        id: i64,
    ) -> Result<Organizer, DomainError> {
        let organizer = OrganizerCrate::Entity::find_by_id(id)
            .one(self.database.deref())
            .await
            .map_err(|e| {
                log::error!("{}", e);
                InfraError::DatabaseError(DatabaseError::SelectFail)
            })?;

        self.get_user_by_organizer(organizer).await?.ok_or(
            InfraError::DatabaseError(DatabaseError::DataNotFound).into(),
        )
    }
}
