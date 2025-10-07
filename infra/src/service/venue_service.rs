use std::{ops::Deref, sync::Arc};

use async_trait::async_trait;
use domain::{
    domain_error::{domain_venue_error::DomainVenueError, DomainError},
    venue_domain::{venue_dto::IndexVenue, VenueRepository},
    PageLimit,
};
use domain_core::{
    user::lessor::Lessor,
    venue::{venue_image::VenueImage, Venue, VenueBuilder},
};
use sea_orm::{
    ActiveModelTrait,
    ActiveValue::{NotSet, Set},
    ColumnTrait, DatabaseConnection, EntityTrait, FromQueryResult, JoinType,
    LoaderTrait, PaginatorTrait, QueryFilter, QueryOrder, QuerySelect,
};

use crate::{
    database::{
        entities::{
            lessor as LessorCrate,
            sea_orm_active_enums::{ActivityType, VenueState},
            user as UserCrate, venue as VenueCrate,
            venue_image_uri as VenueImageUri,
        },
        DatabaseError,
    },
    infra_error::InfraError,
    service::{
        user_service::{db_lessor_to_domain, db_user_to_domain_user},
        venue_service::enum_converstion::{
            venue_status_to_db, venue_status_to_domain,
        },
    },
};

pub mod enum_converstion;

#[derive(Debug, FromQueryResult)]
struct VenueWithLessor {
    pub venue_id: i64,
    pub venue_name: String,
    pub venue_address: String,
    pub lessor_id: i64,
    pub lessor_name: String,
    pub lessor_avatar: String,
}

pub struct VenueService {
    database: Arc<DatabaseConnection>,
}

impl VenueService {
    pub fn new(database: Arc<DatabaseConnection>) -> Self {
        Self { database }
    }
}

pub(crate) fn venue_image_to_db(
    image: &VenueImage,
) -> VenueImageUri::ActiveModel {
    let id = match image.id.clone() {
        Some(e) => Set(e),
        None => NotSet,
    };
    VenueImageUri::ActiveModel {
        id,
        venue_id: Set(image.venue_id.clone()),
        title: Set(image.title.clone()),
        uri: Set(image.uri.clone()),
        comment: Set(image.comment.clone()),
        create_time: Set(image.createtime.naive_utc()),
    }
}

pub(crate) fn venue_image_to_domain(
    image: &VenueImageUri::Model,
) -> VenueImage {
    VenueImage {
        id: Some(image.id.clone()),
        venue_id: image.venue_id.clone(),
        title: image.title.clone(),
        uri: image.uri.clone(),
        comment: image.comment.clone(),
        createtime: image.create_time.and_utc(),
    }
}

pub(crate) fn domain_venue_to_db(
    venue: Venue,
) -> (VenueCrate::ActiveModel, Vec<VenueImageUri::ActiveModel>) {
    let id = match venue.id().clone() {
        Some(e) => Set(e),
        None => NotSet,
    };

    let state = Set(venue_status_to_db(venue.status().clone()));

    let images = venue.images().iter().map(venue_image_to_db).collect();

    let venue = VenueCrate::ActiveModel {
        id,
        lessor_id: Set(venue.lessor_id().clone()),
        name: Set(venue.name().clone()),
        description: Set(venue.description().clone()),
        address: Set(venue.address().clone()),
        capacity: Set(venue.capacity().clone()),
        allow_activity: Set(ActivityType::All),
        state,
        createtime: Set(venue.createtime().naive_utc()),
        updatetime: Set(venue.updatetime().naive_utc()),
    };
    (venue, images)
}

pub(crate) fn db_venue_to_domain(
    venue: VenueCrate::Model,
    venue_images: Vec<VenueImageUri::Model>,
) -> Result<Venue, DomainError> {
    let venue_images = venue_images.iter().map(venue_image_to_domain).collect();
    let status = venue_status_to_domain(venue.state);
    let builder = VenueBuilder::default();
    let venue = builder
        .id(Some(venue.id))
        .lessor_id(venue.lessor_id)
        .name(venue.name)
        .images(venue_images)
        .description(venue.description)
        .address(venue.address)
        .capacity(venue.capacity)
        .status(status)
        .createtime(venue.updatetime.and_utc())
        .createtime(venue.updatetime.and_utc())
        .build()
        .map_err(|e| {
            log::error!("{}", e);
            DomainVenueError::InvalidVeuneContstruction
        })?;
    Ok(venue)
}

#[async_trait]
impl VenueRepository for VenueService {
    async fn find_venue_by_id(&self, id: i64) -> Result<Venue, DomainError> {
        let venue = VenueCrate::Entity::find_by_id(id)
            .one(self.database.deref())
            .await
            .map_err(|e| {
                log::error!("{}", e);
                InfraError::DatabaseError(DatabaseError::SelectFail)
            })?;
        let venue = venue
            .ok_or(InfraError::DatabaseError(DatabaseError::DataNotFound))?;
        let venue_images = VenueImageUri::Entity::find()
            .filter(VenueImageUri::Column::VenueId.eq(venue.id))
            .all(self.database.deref())
            .await
            .map_err(|e| {
                log::error!("{}", e);
                InfraError::DatabaseError(DatabaseError::SelectFail)
            })?;
        let venue = db_venue_to_domain(venue, venue_images)?;
        Ok(venue)
    }

    async fn find_venue_by_lessor_id(
        &self,
        id: i64,
        page: PageLimit,
    ) -> Result<Vec<Venue>, DomainError> {
        let venues = VenueCrate::Entity::find()
            .filter(VenueCrate::Column::LessorId.eq(id))
            .paginate(self.database.deref(), page.limit)
            .fetch_page(page.page)
            .await
            .map_err(|e| {
                log::error!("{}", e);
                InfraError::DatabaseError(DatabaseError::SelectFail)
            })?;

        let venue_with_images = venues
            .load_many(VenueImageUri::Entity, self.database.deref())
            .await
            .map_err(|e| {
                log::error!("{}", e);
                InfraError::DatabaseError(DatabaseError::SelectFail)
            })?;

        let venues =
            venues.into_iter().try_fold(Vec::new(), |mut acc, v| {
                let images = venue_with_images
                    .iter()
                    .flatten()
                    .filter(|t| t.venue_id == v.id)
                    .cloned()
                    .collect();
                let item = db_venue_to_domain(v, images)?;
                acc.push(item);
                Ok::<Vec<Venue>, DomainError>(acc)
            })?;

        Ok(venues)
    }

    async fn find_venue_by_name(
        &self,
        name: String,
        page: PageLimit,
    ) -> Result<Vec<Venue>, DomainError> {
        let venue = VenueCrate::Entity::find()
            .filter(VenueCrate::Column::Name.contains(name.as_str()))
            .filter(VenueCrate::Column::State.eq(VenueState::Published))
            .paginate(self.database.deref(), page.limit)
            .fetch_page(page.page)
            .await
            .map_err(|e| {
                log::error!("{}", e);
                InfraError::DatabaseError(DatabaseError::SelectFail)
            })?;

        let venue_with_images = venue
            .load_many(VenueImageUri::Entity, self.database.deref())
            .await
            .map_err(|e| {
                log::error!("{}", e);
                InfraError::DatabaseError(DatabaseError::SelectFail)
            })?;

        let venue = venue.into_iter().try_fold(Vec::new(), |mut acc, v| {
            let images = venue_with_images
                .iter()
                .flatten()
                .filter(|t| t.venue_id == v.id)
                .cloned()
                .collect();
            let item = db_venue_to_domain(v, images)?;
            acc.push(item);
            Ok::<Vec<Venue>, DomainError>(acc)
        })?;

        Ok(venue)
    }

    async fn create_venue(&self, v: Venue) -> Result<Venue, DomainError> {
        let (venue, venue_images) = domain_venue_to_db(v.clone());
        let venue = venue.insert(self.database.deref()).await.map_err(|e| {
            log::error!("{}", e);
            InfraError::DatabaseError(DatabaseError::SaveEntityFail)
        })?;
        let _ = VenueImageUri::Entity::insert_many(venue_images)
            .exec(self.database.deref())
            .await
            .map_err(|e| {
                log::error!("{}", e);
                InfraError::DatabaseError(DatabaseError::SaveEntityFail)
            })?;
        let v = v.update_id(venue.id);
        let venue_images = VenueImageUri::Entity::find()
            .filter(VenueImageUri::Column::VenueId.eq(venue.id))
            .all(self.database.deref())
            .await
            .map_err(|e| {
                log::error!("{}", e);
                InfraError::DatabaseError(DatabaseError::SelectFail)
            })?;
        let venue_images =
            venue_images.iter().map(venue_image_to_domain).collect();
        let v = v.update_images(venue_images);
        Ok(v)
    }

    async fn save_venue(&self, v: Venue) -> Result<(), DomainError> {
        let (venue, _venue_images) = domain_venue_to_db(v);
        venue.save(self.database.deref()).await.map_err(|e| {
            log::error!("{}", e);
            InfraError::DatabaseError(DatabaseError::SaveEntityFail)
        })?;
        Ok(())
    }

    async fn get_venues_for_index(
        &self,
        page: PageLimit,
    ) -> Result<Vec<IndexVenue>, DomainError> {
        let venues = VenueCrate::Entity::find()
            .filter(VenueCrate::Column::State.eq(VenueState::Published))
            .join(
                JoinType::InnerJoin,
                VenueCrate::Entity::belongs_to(LessorCrate::Entity).into(),
            )
            .join(
                JoinType::InnerJoin,
                LessorCrate::Entity::belongs_to(UserCrate::Entity).into(),
            )
            .select_only()
            .column_as(VenueCrate::Column::Id, "venue_id")
            .column_as(VenueCrate::Column::Name, "venue_name")
            .column_as(VenueCrate::Column::Address, "venue_address")
            .column_as(LessorCrate::Column::Id, "lessor_id")
            .column_as(UserCrate::Column::Username, "lessor_name")
            .column_as(UserCrate::Column::Avatar, "lessor_avatar")
            .into_model::<VenueWithLessor>()
            .paginate(self.database.deref(), page.limit)
            .fetch_page(page.page)
            .await
            .map_err(|e| {
                log::error!("{}", e);
                InfraError::DatabaseError(DatabaseError::SelectFail)
            })?;
        let venue_ids: Vec<i64> =
            venues.iter().map(|i| i.venue_id.clone()).collect();
        let images = VenueImageUri::Entity::find()
            .filter(VenueImageUri::Column::VenueId.is_in(venue_ids))
            .order_by_asc(VenueImageUri::Column::CreateTime)
            .group_by(VenueImageUri::Column::VenueId)
            .all(self.database.deref())
            .await
            .map_err(|e| {
                log::error!("{}", e);
                InfraError::DatabaseError(DatabaseError::SelectFail)
            })?;
        let index_venues = venues
            .into_iter()
            .map(|m| {
                let mut venue_image = String::new();
                if let Some(image) =
                    images.iter().find(|i| i.venue_id == m.venue_id)
                {
                    venue_image = image.uri.to_string();
                }
                IndexVenue {
                    lessor_avatar: m.lessor_avatar,
                    lessor_id: m.lessor_id,
                    venue_name: m.venue_name,
                    address: m.venue_address,
                    venue_id: m.venue_id,
                    venue_image,
                }
            })
            .collect();
        Ok(index_venues)
    }

    async fn is_venue_owned_by_lessor(
        &self,
        lessor_id: i64,
        venue_id: i64,
    ) -> Result<bool, DomainError> {
        let venue = VenueCrate::Entity::find()
            .filter(VenueCrate::Column::Id.eq(venue_id))
            .filter(VenueCrate::Column::LessorId.eq(lessor_id))
            .one(self.database.deref())
            .await
            .map_err(|e| {
                log::error!("{}", e);
                InfraError::DatabaseError(DatabaseError::SelectFail)
            })?;
        Ok(venue.is_some())
    }

    async fn find_lessor_by_venue_id(
        &self,
        venue_id: i64,
    ) -> Result<Lessor, DomainError> {
        let venue = VenueCrate::Entity::find_by_id(venue_id)
            .one(self.database.deref())
            .await
            .map_err(|e| {
                log::error!("{}", e);
                InfraError::DatabaseError(DatabaseError::SelectFail)
            })?;

        let venue = venue
            .ok_or(InfraError::DatabaseError(DatabaseError::DataNotFound))?;
        let (lessor, user) = LessorCrate::Entity::find_by_id(venue.lessor_id)
            .find_also_related(UserCrate::Entity)
            .one(self.database.deref())
            .await
            .map_err(|e| {
                log::error!("{}", e);
                InfraError::DatabaseError(DatabaseError::SelectFail)
            })?
            .ok_or(InfraError::DatabaseError(DatabaseError::DataNotFound))?;

        let user =
            user.ok_or(InfraError::DatabaseError(DatabaseError::DataNotFound))?;
        let user = db_user_to_domain_user(user)?;
        let lessor = db_lessor_to_domain(user, lessor)?;
        Ok(lessor)
    }

    async fn save_image_data(
        &self,
        images: Vec<VenueImage>,
    ) -> Result<Vec<VenueImage>, DomainError> {
        let mut res: Vec<VenueImageUri::Model> = vec![];
        for item in images {
            let image = venue_image_to_db(&item);
            let image =
                image.insert(self.database.deref()).await.map_err(|e| {
                    log::error!("{}", e);
                    InfraError::DatabaseError(DatabaseError::SaveEntityFail)
                })?;
            res.push(image);
        }
        let images = res.iter().map(venue_image_to_domain).collect();
        Ok(images)
    }

    async fn delete_images(
        &self,
        images: Vec<i64>,
        venue_id: i64,
    ) -> Result<(), DomainError> {
        let res = VenueImageUri::Entity::find()
            .filter(VenueImageUri::Column::VenueId.eq(venue_id))
            .filter(VenueImageUri::Column::Id.is_in(images.clone()))
            .all(self.database.deref())
            .await
            .map_err(|e| {
                log::error!("{}", e);
                InfraError::DatabaseError(DatabaseError::SelectFail)
            })?;
        if res.len() < 1 {
            return Err(
                InfraError::DatabaseError(DatabaseError::DataNotFound).into()
            );
        }
        VenueImageUri::Entity::delete_many()
            .filter(VenueImageUri::Column::Id.is_in(images))
            .exec(self.database.deref())
            .await
            .map_err(|e| {
                log::error!("{}", e);
                InfraError::DatabaseError(DatabaseError::DeleteEntityFail)
            })?;
        Ok(())
    }
}
