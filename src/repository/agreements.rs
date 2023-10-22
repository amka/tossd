use log::debug;
use sea_orm::*;

use crate::agreements::{AcceptAgreementRequest, CreateAgreementRequest, CreateVersionRequest};
use crate::models::{agreement, agreement_acceptance_status, agreement_versions};
use crate::models::agreement_versions::Model;

pub struct AgreementsRepository;

impl AgreementsRepository {
    pub async fn add(db: &DbConn, create_agreement: CreateAgreementRequest)
                     -> Result<agreement::ActiveModel, DbErr> {
        debug!("AgreementsRepository::add <- {:?}", create_agreement);
        let now = chrono::Utc::now().naive_utc();

        let agreement = agreement::ActiveModel {
            inner_title: Set(create_agreement.inner_title),
            public_title: Set(create_agreement.public_title),
            created_at: Set(now),
            updated_at: Set(now),
            author_id: Set(create_agreement.author_id),
            deleted: Set(false),
            ..Default::default()
        };

        agreement.save(db).await
    }

    pub async fn add_version(db: &DbConn, create_version: CreateVersionRequest)
                             -> Result<agreement_versions::ActiveModel, DbErr>
    {
        debug!("AgreementsRepository::add_version <- {:?}", create_version.agreement_id);
        // Находим все Версии Соглашения, чтобы узнать новый номер Версии.
        let versions = agreement_versions::Entity::find()
            .filter(agreement_versions::Column::AgreementId.eq(create_version.agreement_id))
            .count(db)
            .await?;

        let now = chrono::Utc::now().naive_utc();

        let version = agreement_versions::ActiveModel {
            agreement_id: Set(create_version.agreement_id),
            version: Set((versions + 1) as i32),
            content: Set(create_version.content),
            created_at: Set(now),
            updated_at: Set(now),
            author_id: Set(create_version.author_id),
            deleted: Set(false),
            ..Default::default()
        };

        version.save(db).await
    }

    /// Возвращает Соглашение по его идентификатору.
    pub async fn find_by_id(db: &DbConn, id: i32)
                            -> Result<Option<agreement::Model>, DbErr> {
        debug!("AgreementsRepository::find_by_id <- {:?}", id);
        agreement::Entity::find_by_id(id).one(db).await
    }

    /// Возвращает Версию Соглашения с самой высокой версией.
    ///
    /// Фукнция использует поле `version` для выборки, т. о. обновление бодлее старых версий
    /// не изменит результат выполнения этой функции.
    pub async fn find_version_by_agreement_id(db: &DbConn, agreement_id: i32)
                                              -> Result<Option<agreement_versions::Model>, DbErr> {
        debug!("AgreementsRepository::find_version_by_agreement_id <- {:?}", agreement_id);
        agreement_versions::Entity::find()
            .filter(agreement_versions::Column::AgreementId.eq(agreement_id))
            .order_by_desc(agreement_versions::Column::Version)
            .one(db)
            .await
    }

    /// Возвращает все Версии `agreement_versions::Entity` Соглашения по идентификатору Соглашения.
    ///
    /// # Arguments
    ///
    /// * `db`: ссылка на подключение к БД
    /// * `agreement_id`: идентификатор Соглашения
    ///
    /// returns: Result<Vec<Model, Global>, DbErr>
    ///
    /// # Examples
    ///
    /// ```rust
    /// let conn = &self.connection;
    /// let id = 123;
    ///
    /// let versions = AgreementsRepository::find_versions_by_agreement_id(conn, id)
    ///     .await
    ///     .ok().unwrap();
    /// ```
    pub async fn find_versions_by_agreement_id(db: &DbConn, agreement_id: i32) -> Result<Vec<Model>, DbErr> {
        debug!("AgreementsRepository::find_versions_by_agreement_id <- {:?}", agreement_id);
        agreement_versions::Entity::find()
            .filter(agreement_versions::Column::AgreementId.eq(agreement_id))
            .order_by_desc(agreement_versions::Column::Version)
            .all(db)
            .await
    }

    pub async fn accept_agreement(db: &DbConn, accept_request: AcceptAgreementRequest)
                                  -> Result<agreement_acceptance_status::ActiveModel, DbErr> {
        debug!("AgreementsRepository::accept_agreement <- {:?}", accept_request);
        agreement_acceptance_status::ActiveModel {
            user_id: Set(accept_request.user_id),
            provider_id: Set(accept_request.provider_id),
            agreement_id: Set(accept_request.agreement_id),
            version: Set(accept_request.version),
            accepted: Set(true),
            accepted_at: Set(chrono::Utc::now().naive_utc()),
            ..Default::default()
        }
            .save(db)
            .await
    }
}