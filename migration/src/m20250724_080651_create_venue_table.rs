use sea_orm_migration::{prelude::*, schema::*};

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        // Replace the sample below with your own migration scripts
        let db = manager.get_connection();
        db.execute_unprepared(r#"CREATE TABLE IF NOT EXISTS "venue" (
id BIGSERIAL PRIMARY KEY,
lessor_id BIGSERIAL NOT NULL,
name varchar(255) NOT NULL,
description varchar(500) NOT NULL,
address varchar(255) NOT NULL,
capacity int NOT NULL DEFAULT 0,
images JSONB NOT NULL,
allow_activity JSONB DEFAULT '["all"]'::jsonb,
state venue_state DEFAULT 'published',
createTime timestamp DEFAULT now(),
updateTime timestamp
);"#).await?;
        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        let db = manager.get_connection();
        db.execute_unprepared(r#"DROP TABLE IF EXISTS "venue""#).await?;
        Ok(())
    }
}
