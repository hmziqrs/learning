use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(Todo::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(Todo::Id)
                            .uuid()
                            .not_null()
                            .primary_key(),
                    )
                    .col(ColumnDef::new(Todo::Title).text().not_null())
                    .col(ColumnDef::new(Todo::Description).text())
                    .col(ColumnDef::new(Todo::Completed).boolean().not_null().default(false))
                    .col(ColumnDef::new(Todo::CreatedAt).timestamp_with_time_zone().not_null())
                    .col(ColumnDef::new(Todo::UpdatedAt).timestamp_with_time_zone().not_null())
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(Todo::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
enum Todo {
    Table,
    Id,
    Title,
    Description,
    Completed,
    CreatedAt,
    UpdatedAt,
}