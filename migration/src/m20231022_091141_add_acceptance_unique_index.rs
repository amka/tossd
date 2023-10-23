use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_index(
                Index::create()
                    .if_not_exists()
                    .name("idx-agreement-user-accepted")
                    .table(AgreementAcceptanceStatus::Table)
                    .col(AgreementAcceptanceStatus::AgreementId)
                    .col(AgreementAcceptanceStatus::UserId)
                    .col(AgreementAcceptanceStatus::Version)
                    .unique()
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager.drop_index(Index::drop().name("idx-agreement-user-accepted").to_owned())
            .await
    }
}

#[derive(DeriveIden)]
enum AgreementAcceptanceStatus {
    Table,
    UserId,
    AgreementId,
    Version,
}
