use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(AgreementVersions::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(AgreementVersions::Id)
                            .integer()
                            .not_null()
                            .auto_increment()
                            .primary_key(),
                    )
                    .col(ColumnDef::new(AgreementVersions::AgreementId).integer().not_null())
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk-agreements-agreement_id")
                            .from(AgreementVersions::Table, AgreementVersions::AgreementId)
                            .to(Agreement::Table, Agreement::Id),
                    )
                    .col(ColumnDef::new(AgreementVersions::Version).integer().not_null())
                    .col(ColumnDef::new(AgreementVersions::Content).text().not_null())
                    .col(ColumnDef::new(AgreementVersions::CreatedAt).timestamp().not_null())
                    .col(ColumnDef::new(AgreementVersions::UpdatedAt).timestamp().not_null())
                    .col(ColumnDef::new(AgreementVersions::AuthorId).big_integer().not_null())
                    .col(ColumnDef::new(AgreementVersions::Deleted).boolean().not_null())
                    .to_owned(),
            )
            .await?;

        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(AgreementVersions::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
enum AgreementVersions {
    Table,
    Id,
    AgreementId,
    Version,
    Content,
    CreatedAt,
    UpdatedAt,
    AuthorId,
    Deleted,
}

#[derive(DeriveIden)]
enum Agreement {
    Table,
    Id,
}

