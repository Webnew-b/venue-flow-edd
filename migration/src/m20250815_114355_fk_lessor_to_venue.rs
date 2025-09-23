use sea_orm_migration::{prelude::*, schema::*};

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        // Replace the sample below with your own migration scripts
        let db = manager.get_connection();
        db.execute_unprepared(
            r#"
        ALTER TABLE "venue"
        ADD CONSTRAINT fk_lessor_to_venue
        FOREIGN KEY (lessor_id) 
        REFERENCES "lessor" ("id")
        ON UPDATE CASCADE
ON DELETE CASCADE
;
        "#,
        )
        .await?;
        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        let db = manager.get_connection();
        db.execute_unprepared(
            r#"ALTER TABLE "venue" DROP CONSTRAINT fk_lessor_to_venue;"#,
        )
        .await?;
        Ok(())
    }
}
