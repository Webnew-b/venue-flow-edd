use std::{ops::Deref, sync::Arc};

use async_trait::async_trait;
use domain::{
    domain_error::{domain_rental_error::DomainRentalError, DomainError},
    rental_domain::{rental_dto::RentalRes, RentalRespository},
    PageLimit,
};
use domain_core::rental::{Rental, RentalBuilder};
use sea_orm::{
    ActiveModelTrait,
    ActiveValue::{NotSet, Set},
    ColumnTrait, PaginatorTrait, QueryOrder, QuerySelect, RelationTrait,
};
use sea_orm::{DatabaseConnection, EntityTrait, QueryFilter};

use crate::{
    database::{
        entities::{rental_request as RentalCrate, venue as VenueCrate},
        DatabaseError,
    },
    infra_error::InfraError,
    service::rental_service::enum_converstion::{
        rental_activity_type_to_db, rental_activity_type_to_domain,
        rental_status_to_db, rental_status_to_domain,
    },
};

pub mod enum_converstion;

pub(crate) fn domain_rental_to_db(rental: Rental) -> RentalCrate::ActiveModel {
    let id = match rental.id().clone() {
        Some(e) => Set(e),
        None => NotSet,
    };

    let activity = rental.activity_type().clone();
    RentalCrate::ActiveModel {
        id,
        organizer_id: Set(rental.organizer_id().clone()),
        venue_id: Set(rental.venue_id().clone()),
        start_time: Set(rental.start_time().naive_utc()),
        end_time: Set(rental.end_time().naive_utc()),
        activity_type: Set(rental_activity_type_to_db(activity)),
        request_comments: Set(rental.request_comments().clone()),
        status: Set(rental_status_to_db(rental.status().clone())),
        createtime: Set(rental.createtime().naive_utc()),
        updatetime: Set(rental.updatetime().naive_utc()),
    }
}

pub(crate) fn db_rental_to_domain(
    rental: RentalCrate::Model,
) -> Result<Rental, DomainError> {
    let builder = RentalBuilder::default();
    let activity = rental_activity_type_to_domain(rental.activity_type);
    let rental = builder
        .id(Some(rental.id))
        .organizer_id(rental.organizer_id)
        .venue_id(rental.venue_id)
        .start_time(rental.start_time.and_utc())
        .end_time(rental.end_time.and_utc())
        .activity_type(activity)
        .status(rental_status_to_domain(rental.status))
        .createtime(rental.createtime.and_utc())
        .updatetime(rental.updatetime.and_utc())
        .build()
        .map_err(|e| {
            tracing::error!("{}", e);
            DomainRentalError::InvalidRentalContstruction
        })?;
    Ok(rental)
}

pub struct RentalService {
    database: Arc<DatabaseConnection>,
}

impl RentalService {
    pub fn new(database: Arc<DatabaseConnection>) -> Self {
        Self { database }
    }
}

#[async_trait]
impl RentalRespository for RentalService {
    async fn find_rental_by_id(&self, id: i64) -> Result<Rental, DomainError> {
        let rental = RentalCrate::Entity::find_by_id(id)
            .one(self.database.deref())
            .await
            .map_err(|e| {
                tracing::error!("{}", e);
                InfraError::DatabaseError(DatabaseError::SelectFail)
            })?;
        let res =
            rental.ok_or(DomainError::DataIsNotFound("rental".to_string()))?;
        db_rental_to_domain(res)
    }

    async fn get_rental_lists(
        &self,
        lessor_id: i64,
        page: PageLimit,
    ) -> Result<Vec<RentalRes>, DomainError> {
        let rentals = RentalCrate::Entity::find()
            .find_also_related(VenueCrate::Entity)
            .filter(VenueCrate::Column::LessorId.eq(lessor_id))
            .order_by_desc(RentalCrate::Column::Createtime)
            .paginate(self.database.deref(), page.limit)
            .fetch_page(page.page.saturating_sub(1))
            .await
            .map_err(|e| {
                tracing::error!("{}", e);
                InfraError::DatabaseError(DatabaseError::SelectFail)
            })?;
        let rentals = rentals
            .into_iter()
            .map(|(l, v)| {
                //todo check v is existed.
                let venue = v.expect("The venue is not found.");
                let status = rental_status_to_domain(l.status).to_string();
                let activity_type =
                    rental_activity_type_to_domain(l.activity_type).to_string();
                RentalRes {
                    id: l.id,
                    venue_id: venue.id,
                    venue_title: venue.name,
                    organizer_id: l.organizer_id,
                    start_time: l
                        .start_time
                        .format("%Y-%m-%d %H:%M:%S")
                        .to_string(),
                    end_time: l
                        .end_time
                        .format("%Y-%m-%d %H:%M:%S")
                        .to_string(),
                    activity_type,
                    request_comments: l.request_comments,
                    status,
                }
            })
            .collect();
        Ok(rentals)
    }

    async fn create_rental_request(
        &self,
        rental: Rental,
    ) -> Result<Rental, DomainError> {
        let res = domain_rental_to_db(rental.clone());
        let res = res.insert(self.database.deref()).await.map_err(|e| {
            tracing::error!("{}", e);
            InfraError::DatabaseError(DatabaseError::SaveEntityFail)
        })?;
        let rental = rental.update_id(res.id);
        Ok(rental)
    }

    async fn save_rental(&self, rental: Rental) -> Result<(), DomainError> {
        let rental = domain_rental_to_db(rental);
        rental.save(self.database.deref()).await.map_err(|e| {
            tracing::error!("{}", e);
            InfraError::DatabaseError(DatabaseError::SaveEntityFail)
        })?;
        Ok(())
    }
}
