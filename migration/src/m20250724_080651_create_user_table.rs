use sea_orm_migration::{prelude::*, schema::*};

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        // Replace the sample below with your own migration scripts
        let db = manager.get_connection();
        db.execute_unprepared(r#"CREATE TABLE IF NOT EXISTS "user" (
id BIGSERIAL PRIMARY KEY,
username varchar(255) NOT NULL,
email varchar(255) NOT NULL,
avatar varchar(255),
gender user_gender DEFAULT 'prefer_not_to_say',
introduce varchar(300),
is_show bool DEFAULT false,
is_delete bool DEFAULT false,
status user_status DEFAULT 'active',
createTime timestamp DEFAULT now(),
updateTime timestamp
);"#).await?;
        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        let db = manager.get_connection();
        db.execute_unprepared(r#"DROP TABLE IF EXISTS "user""#).await?;
        Ok(())
    }
}
