use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(Agreement::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(Agreement::Id)
                            .integer()
                            .not_null()
                            .auto_increment()
                            .primary_key(),
                    )
                    .col(ColumnDef::new(Agreement::InnerTitle).string().not_null())
                    .col(ColumnDef::new(Agreement::CreatedAt).timestamp().not_null())
                    .col(ColumnDef::new(Agreement::UpdatedAt).timestamp().not_null())
                    .col(ColumnDef::new(Agreement::ProviderId).big_integer().not_null())
                    .col(ColumnDef::new(Agreement::AuthorId).big_integer().not_null())
                    .col(ColumnDef::new(Agreement::Deleted).boolean().not_null())
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(Agreement::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
enum Agreement {
    Table,
    Id,
    InnerTitle,
    CreatedAt,
    UpdatedAt,
    ProviderId,
    AuthorId,
    Deleted,
}
