pub use sea_orm_migration::prelude::*;

pub struct Migrator;

mod m20250724_080650_userStatus;
mod m20250724_080650_user_gender;
mod m20250724_080650_venue_state;
mod m20250724_080650_activity_type;
mod m20250724_080650_request_status;

mod m20250724_080652_fk_user_to_lessor;
mod m20250724_080651_create_user_table;

mod m20250724_080652_fk_lessor_to_venue;
mod m20250724_080651_create_venue_table;

mod m20250724_080651_create_lessor_table;
mod m20250724_080652_fk_user_to_organizer;

mod m20250724_080651_create_organizer_table;
mod m20250724_080652_fk_venue_to_rental_request;

mod m20250724_080651_create_rental_request_table;
mod m20250724_080652_fk_organizer_to_rental_request;

#[async_trait::async_trait]
impl MigratorTrait for Migrator {
    fn migrations() -> Vec<Box<dyn MigrationTrait>> {
        vec![
            Box::new(m20250724_080650_userStatus::Migration),
            Box::new(m20250724_080650_user_gender::Migration),
            Box::new(m20250724_080650_venue_state::Migration),
            Box::new(m20250724_080650_activity_type::Migration),
            Box::new(m20250724_080650_request_status::Migration),
            Box::new(m20250724_080651_create_user_table::Migration),
            Box::new(m20250724_080651_create_venue_table::Migration),
            Box::new(m20250724_080651_create_lessor_table::Migration),
            Box::new(m20250724_080651_create_organizer_table::Migration),
            Box::new(m20250724_080651_create_rental_request_table::Migration),
            Box::new(m20250724_080652_fk_organizer_to_rental_request::Migration),
            Box::new(m20250724_080652_fk_user_to_lessor::Migration),
            Box::new(m20250724_080652_fk_venue_to_rental_request::Migration),
            Box::new(m20250724_080652_fk_user_to_organizer::Migration),
            Box::new(m20250724_080652_fk_lessor_to_venue::Migration),
        ]
    }
}
