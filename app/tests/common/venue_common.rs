use async_trait::async_trait;
use domain::venue_domain::VenueRepository;
use domain_core::user::lessor::Lessor;
use domain_core::venue::venue_update::VenueUpdate;
use domain_core::venue::Venue;
use domain::domain_error::DomainError;
use domain::venue_domain::venue_dto::IndexVenue;
use domain::PageLimit;
use mockall::mock;


mock!{
    pub VenueRepo {}
    #[async_trait]
    impl VenueRepository for VenueRepo {
        async fn find_venue_by_id(&self,id:i64) 
            -> Result<Venue,DomainError>;
        
        async fn find_venue_by_lessor_id(&self,id:i64,page:PageLimit) 
            -> Result<Vec<Venue>,DomainError>;
        
        async fn find_venue_by_name(&self,name:String,page:PageLimit) 
            -> Result<Vec<Venue>,DomainError>;
        
        async fn modify_venue(&self,update:VenueUpdate)
            -> Result<(),DomainError>;
        
        async fn create_venue(&self,v:Venue)
            -> Result<Venue,DomainError>;
        
        async fn save_venue(&self,v:Venue)
            -> Result<(),DomainError>;
        
        async fn get_venues_for_index(&self,page:PageLimit)
            -> Result<Vec<IndexVenue>,DomainError>;
        
        async fn is_venue_owned_by_lessor(&self,lessor_id:i64,venue_id:i64)
            -> Result<bool,DomainError>;
        
        async fn find_lessor_by_venue_id(&self,venue_id:i64)
            -> Result<Lessor,DomainError>;
    } 
}

pub struct TestVenueMocks{
    pub repo:MockVenueRepo,
}

pub fn mock_venue_setup() -> TestVenueMocks {
    TestVenueMocks { repo: MockVenueRepo::new() }
}
