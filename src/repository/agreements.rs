use log::debug;
use sea_orm::*;

use crate::agreements::CreateAgreementRequest;
use crate::models::{agreement, agreement_versions};
use crate::models::agreement_versions::Model;

pub struct AgreementsRepository;


impl AgreementsRepository {
    pub async fn add(db: &DbConn, create_agreement: CreateAgreementRequest)
                     -> Result<agreement::ActiveModel, DbErr> {
        debug!("AgreementsRepository::add <- {:?}", create_agreement);
        let now = chrono::Utc::now().naive_utc();

        let mut agreement = agreement::ActiveModel {
            inner_title: Set(create_agreement.inner_title),
            created_at: Set(now),
            updated_at: Set(now),
            author_id: Set(create_agreement.author_id),
            deleted: Set(false),
            ..Default::default()
        };

        agreement.save(db).await
    }

    pub async fn add_version(db: &DbConn, agreement_id: i32, create_agreement: CreateAgreementRequest)
                             -> Result<agreement_versions::ActiveModel, DbErr>
    {
        debug!("AgreementsRepository::add_version <- {:?}", agreement_id);
        // Находим все Версии Соглашения, чтобы узнать новый номер Версии.
        let versions = agreement_versions::Entity::find()
            .filter(agreement_versions::Column::AgreementId.eq(agreement_id))
            .count(db)
            .await?;

        let now = chrono::Utc::now().naive_utc();

        let mut version = agreement_versions::ActiveModel {
            agreement_id: Set(agreement_id),
            version: Set((versions + 1) as i32),
            title: Set(create_agreement.inner_title),
            content: Set(create_agreement.content),
            created_at: Set(now),
            updated_at: Set(now),
            deleted: Set(false),
            ..Default::default()
        };

        version.save(db).await
    }

    /// Возвращает Соглашение по его идентификатору.
    pub async fn find_by_id(db: &DbConn, id: i32)
                            -> Result<Option<agreement::Model>, DbErr> {
        agreement::Entity::find_by_id(id).one(db).await
    }

    /// Возвращает Версию Соглашения с самой высокой версией.
    ///
    /// Фукнция использует поле `version` для выборки, т. о. обновление бодлее старых версий
    /// не изменит результат выполнения этой функции.
    pub async fn find_version_by_agreement_id(db: &DbConn, agreement_id: i32)
                                              -> Result<Option<agreement_versions::Model>, DbErr> {
        agreement_versions::Entity::find()
            .filter(agreement_versions::Column::AgreementId.eq(agreement_id))
            .order_by_desc(agreement_versions::Column::Version)
            .one(db)
            .await
    }

    pub async fn find_versions_by_agreement_id(db: &DbConn, agreement_id: i32) -> Result<Vec<Model>, DbErr> {
        agreement_versions::Entity::find()
            .filter(agreement_versions::Column::AgreementId.eq(agreement_id))
            .order_by_desc(agreement_versions::Column::Version)
            .all(db)
            .await
    }
}