use async_trait::async_trait;
use domain::domain_error::DomainError;
use domain::rental_domain::rental_dto::RentalRes;
use domain::rental_domain::RentalRespository;
use domain_core::rental::Rental;
use mockall::mock;

mock! {
    pub RentalRepo {}
    #[async_trait]
    impl RentalRespository for RentalRepo{
        async fn find_rental_by_id(&self,id:i64)
            -> Result<Rental,DomainError>;

        async fn get_rental_lists(&self,lessor_id:i64)
             -> Result<Vec<RentalRes>,DomainError>;

        async fn create_rental_request(&self,rental:Rental)
             -> Result<Rental,DomainError>;

        async fn save_rental(&self,rental:Rental)
            -> Result<(),DomainError>;
    }
}

pub struct TestRentalMocks {
    pub repo: MockRentalRepo,
}

pub fn mock_rental_setup() -> TestRentalMocks {
    TestRentalMocks {
        repo: MockRentalRepo::new(),
    }
}
