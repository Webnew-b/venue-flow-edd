use sea_orm_migration::{prelude::*, schema::*};

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        // Replace the sample below with your own migration scripts
        let db = manager.get_connection();
        db.execute_unprepared(
            r#"CREATE TABLE IF NOT EXISTS "rental_request" (
id BIGSERIAL PRIMARY KEY,
organizer_id BIGSERIAL NOT NULL,
venue_id BIGSERIAL NOT NULL,
start_time timestamp NOT NULL,
end_time timestamp NOT NULL,
activity_type activity_type NOT NULL DEFAULT 'all',
request_comments text NOT NULL,
status request_status NOT NULL DEFAULT 'pending',
createTime timestamp NOT NULL DEFAULT now(),
updateTime timestamp NOT NULL
);"#,
        )
        .await?;
        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        let db = manager.get_connection();
        db.execute_unprepared(r#"DROP TABLE IF EXISTS "rental_request""#)
            .await?;
        Ok(())
    }
}
