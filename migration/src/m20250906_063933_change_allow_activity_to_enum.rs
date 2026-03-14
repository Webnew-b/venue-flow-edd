use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        // Drop the old JSONB column
        manager
            .alter_table(
                Table::alter()
                    .table(Venue::Table)
                    .drop_column(Venue::AllowActivity)
                    .to_owned(),
            )
            .await?;

        // Add the new column with enum type
        manager
            .alter_table(
                Table::alter()
                    .table(Venue::Table)
                    .add_column(
                        ColumnDef::new(Venue::AllowActivity)
                            .custom(Alias::new("activity_type"))
                            .not_null()
                            .default(Expr::cust("'all'::activity_type")), // Set your default enum value
                    )
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        // Drop the enum column
        manager
            .alter_table(
                Table::alter()
                    .table(Venue::Table)
                    .drop_column(Venue::AllowActivity)
                    .to_owned(),
            )
            .await?;

        // Recreate the original JSONB column
        manager
            .alter_table(
                Table::alter()
                    .table(Venue::Table)
                    .add_column(
                        ColumnDef::new(Venue::AllowActivity)
                            .json_binary()
                            .not_null()
                            .default(Expr::cust("'[\"all\"]'::jsonb")),
                    )
                    .to_owned(),
            )
            .await
    }
}

#[derive(Iden)]
enum Venue {
    Table,
    AllowActivity,
}
