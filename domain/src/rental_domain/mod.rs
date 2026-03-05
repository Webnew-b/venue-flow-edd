use async_trait::async_trait;
use domain_core::rental::Rental;

use crate::domain_error::DomainError;
use crate::rental_domain::rental_dto::RentalRes;
use crate::PageLimit;

pub mod rental_dto;

#[async_trait]
pub trait RentalRespository: Sync + Send {
    async fn find_rental_by_id(&self, id: i64) -> Result<Rental, DomainError>;

    async fn get_rental_lists(
        &self,
        lessor_id: i64,
        page: PageLimit,
    ) -> Result<Vec<RentalRes>, DomainError>;

    async fn create_rental_request(
        &self,
        rental: Rental,
    ) -> Result<Rental, DomainError>;

    async fn save_rental(&self, rental: Rental) -> Result<(), DomainError>;
}
