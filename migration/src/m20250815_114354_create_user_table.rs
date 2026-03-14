use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        // Replace the sample below with your own migration scripts
        let db = manager.get_connection();
        db.execute_unprepared(
            r#"CREATE TABLE IF NOT EXISTS "user" (
id BIGSERIAL PRIMARY KEY,
username varchar(255) NOT NULL,
email varchar(255) NOT NULL,
avatar varchar(255) NOT NULL,
gender user_gender NOT NULL DEFAULT 'prefer_not_to_say',
introduce varchar(300) NOT NULL,
is_show bool NOT NULL DEFAULT false,
is_delete bool NOT NULL DEFAULT false,
status user_status NOT NULL DEFAULT 'active',
createTime timestamp NOT NULL DEFAULT now(),
updateTime timestamp NOT NULL
);"#,
        )
        .await?;
        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        let db = manager.get_connection();
        db.execute_unprepared(r#"DROP TABLE IF EXISTS "user""#)
            .await?;
        Ok(())
    }
}
