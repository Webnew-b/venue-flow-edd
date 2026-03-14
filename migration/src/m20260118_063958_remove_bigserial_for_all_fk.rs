use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        // Replace the sample below with your own migration scripts
        let db = manager.get_connection();
        db.execute_unprepared(r#"ALTER TABLE public.rental_request ALTER COLUMN organizer_id DROP DEFAULT;"#).await?;
        db.execute_unprepared(
            r#"ALTER TABLE public.lessor ALTER COLUMN user_id DROP DEFAULT;"#,
        )
        .await?;
        db.execute_unprepared(r#"ALTER TABLE public.rental_request ALTER COLUMN venue_id DROP DEFAULT;"#).await?;
        db.execute_unprepared(r#"ALTER TABLE public.organizer ALTER COLUMN user_id DROP DEFAULT;"#).await?;
        db.execute_unprepared(
            r#"ALTER TABLE public.venue ALTER COLUMN lessor_id DROP DEFAULT;"#,
        )
        .await?;
        db.execute_unprepared(r#"ALTER TABLE public.venue_image_uri ALTER COLUMN venue_id DROP DEFAULT;"#).await?;

        Ok(())
    }

    async fn down(&self, _manager: &SchemaManager) -> Result<(), DbErr> {
        Ok(())
    }
}
