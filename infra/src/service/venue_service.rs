use std::{ops::Deref, sync::Arc};

use async_trait::async_trait;
use domain::{
    domain_error::{
        database_error::DatabaseError, domain_venue_error::DomainVenueError,
        DomainError,
    },
    venue_domain::{venue_dto::IndexVenue, VenueRepository},
    PageLimit,
};
use domain_core::{
    user::lessor::Lessor,
    venue::{
        venue_image::VenueImage, venue_update::VenueUpdate, Venue, VenueBuilder,
    },
};
use sea_orm::{
    ActiveModelTrait,
    ActiveValue::{NotSet, Set},
    ColumnTrait, DatabaseBackend, DatabaseConnection, EntityTrait, LoaderTrait,
    PaginatorTrait, QueryFilter,
};

use crate::{
    database::entities::{
        lessor as LessorCrate,
        sea_orm_active_enums::{ActivityType, VenueState},
        venue as VenueCrate, venue_image_uri as VenueImageUri,
    },
    service::venue_service::enum_converstion::{
        venue_status_to_db, venue_status_to_domain,
    },
};

pub mod enum_converstion;

pub(crate) struct VenueService {
    database: Arc<DatabaseConnection>,
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
                DatabaseError::SelectFail
            })?;
        let venue = venue.ok_or(DatabaseError::DataNotFound)?;
        let venue_images = VenueImageUri::Entity::find()
            .filter(VenueImageUri::Column::VenueId.eq(venue.id))
            .all(self.database.deref())
            .await
            .map_err(|e| {
                log::error!("{}", e);
                DatabaseError::SelectFail
            })?;
        let venue = db_venue_to_domain(venue, venue_images)?;
        Ok(venue)
    }

    async fn find_venue_by_lessor_id(
        &self,
        id: i64,
        page: PageLimit,
    ) -> Result<Vec<Venue>, DomainError> {
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
                DatabaseError::SelectFail
            })?;

        let venue_with_images = venue
            .load_many(VenueImageUri::Entity, self.database.deref())
            .await
            .map_err(|e| {
                log::error!("{}", e);
                DatabaseError::SelectFail
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

    async fn modify_venue(
        &self,
        update: VenueUpdate,
    ) -> Result<(), DomainError> {
    }

    async fn create_venue(&self, v: Venue) -> Result<Venue, DomainError> {
        let venue = domain_venue_to_db(v.clone());
        let venue = venue.insert(self.database.deref()).await.map_err(|e| {
            log::error!("{}", e);
            DatabaseError::SaveEntityFail
        })?;
        let v = v.update_id(venue.id);
        Ok(v)
    }

    async fn save_venue(&self, v: Venue) -> Result<(), DomainError> {
        let venue = domain_venue_to_db(v);
        venue.save(self.database.deref()).await.map_err(|e| {
            log::error!("{}", e);
            DatabaseError::SaveEntityFail
        })?;
        Ok(())
    }

    async fn get_venues_for_index(
        &self,
        page: PageLimit,
    ) -> Result<Vec<IndexVenue>, DomainError> {
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
                DatabaseError::SelectFail
            })?;
        Ok(venue.is_some())
    }

    async fn find_lessor_by_venue_id(
        &self,
        venue_id: i64,
    ) -> Result<Lessor, DomainError> {
    }
}
