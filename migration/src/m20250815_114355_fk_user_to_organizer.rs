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
        ALTER TABLE "organizer"
        ADD CONSTRAINT fk_user_to_organizer
        FOREIGN KEY (user_id) 
        REFERENCES "user" ("id")
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
            r#"ALTER TABLE "organizer" DROP CONSTRAINT fk_user_to_organizer;"#,
        )
        .await?;
        Ok(())
    }
}
