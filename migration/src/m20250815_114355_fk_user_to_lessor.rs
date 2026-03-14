use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        // Replace the sample below with your own migration scripts
        let db = manager.get_connection();
        db.execute_unprepared(
            r#"
        ALTER TABLE "lessor"
        ADD CONSTRAINT fk_user_to_lessor
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
            r#"ALTER TABLE "lessor" DROP CONSTRAINT fk_user_to_lessor;"#,
        )
        .await?;
        Ok(())
    }
}
