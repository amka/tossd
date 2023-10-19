use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(AgreementAcceptanceStatus::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(AgreementAcceptanceStatus::Id)
                            .integer()
                            .not_null()
                            .auto_increment()
                            .primary_key(),
                    )
                    .col(ColumnDef::new(AgreementAcceptanceStatus::UserId).big_integer().not_null())
                    .col(ColumnDef::new(AgreementAcceptanceStatus::ProviderId).integer().not_null())
                    .col(ColumnDef::new(AgreementAcceptanceStatus::AgreementId).integer().not_null())
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk-agreements-agreement_id")
                            .from(AgreementAcceptanceStatus::Table, AgreementAcceptanceStatus::AgreementId)
                            .to(Agreement::Table, Agreement::Id),
                    )
                    .col(ColumnDef::new(AgreementAcceptanceStatus::Version).integer().not_null())
                    .col(ColumnDef::new(AgreementAcceptanceStatus::Accepted).boolean().not_null())
                    .col(ColumnDef::new(AgreementAcceptanceStatus::AcceptedAt).timestamp().not_null())
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(AgreementAcceptanceStatus::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
enum AgreementAcceptanceStatus {
    Table,
    Id,
    UserId,
    ProviderId,
    AgreementId,
    Version,
    Accepted,
    AcceptedAt,
}

#[derive(DeriveIden)]
enum Agreement {
    Table,
    Id,
}
