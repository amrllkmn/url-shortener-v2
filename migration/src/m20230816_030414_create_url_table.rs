use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        // Replace the sample below with your own migration scripts
        manager
            .create_table(
                Table::create()
                    .table(Url::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(Url::Id)
                            .integer()
                            .not_null()
                            .auto_increment()
                            .primary_key(),
                    )
                    .col(ColumnDef::new(Url::Url).string().not_null())
                    .col(ColumnDef::new(Url::Title).string().not_null())
                    .col(ColumnDef::new(Url::Slug).string().not_null())
                    .col(
                        ColumnDef::new(Url::CreatedAt)
                            .timestamp()
                            .default(Expr::current_timestamp())
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(Url::UpdatedAt)
                            .timestamp()
                            .default(Expr::current_timestamp())
                            .not_null(),
                    )
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        // Replace the sample below with your own migration scripts
        manager
            .drop_table(Table::drop().table(Url::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
enum Url {
    Table,
    Id,
    Url,
    Title,
    Slug,
    CreatedAt,
    UpdatedAt,
}
