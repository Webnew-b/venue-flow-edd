use sea_orm_migration::{prelude::*, schema::*};

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        // Replace the sample below with your own migration scripts
        let db = manager.get_connection();
        db.execute_unprepared(r#"CREATE TABLE IF NOT EXISTS "organizer" (
id BIGSERIAL PRIMARY KEY,
user_id BIGSERIAL NOT NULL UNIQUE,
phone varchar(255) NOT NULL,
is_delete bool DEFAULT false,
createTime timestamp DEFAULT now(),
updateTime timestamp
);"#).await?;
        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        let db = manager.get_connection();
        db.execute_unprepared(r#"DROP TABLE IF EXISTS "organizer""#).await?;
        Ok(())
    }
}
