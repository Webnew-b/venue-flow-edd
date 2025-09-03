
use sea_orm_migration::{prelude::*, schema::*};

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        // Replace the sample below with your own migration scripts
        let db = manager.get_connection();
        db.execute_unprepared(r#"
        ALTER TABLE "venue_image_uri"
        ADD CONSTRAINT fk_venue_to_venue_image_uri
        FOREIGN KEY (venue_id) 
        REFERENCES "venue" ("id")
        ;
        "#).await?;
        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        let db = manager.get_connection();
        db.execute_unprepared(r#"ALTER TABLE "venue_image_uri" DROP CONSTRAINT fk_venue_to_venue_image_uri;"#).await?;
        Ok(())
    }
}
