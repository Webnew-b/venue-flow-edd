use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .alter_table(
                Table::alter()
                    .table(VenueImageUri::Table)
                    .add_column(
                        ColumnDef::new(VenueImageUri::CreateTime)
                            .timestamp()
                            .not_null()
                            .default(Expr::current_timestamp()), // Sets default to now()
                    )
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .alter_table(
                Table::alter()
                    .table(VenueImageUri::Table)
                    .drop_column(VenueImageUri::CreateTime)
                    .to_owned(),
            )
            .await
    }
}

#[derive(Iden)]
enum VenueImageUri {
    Table,
    CreateTime,
}
