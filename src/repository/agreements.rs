use log::debug;
use sea_orm::*;

use crate::agreements::{AcceptAgreementRequest, CreateAgreementRequest, CreateVersionRequest, GetUnacceptedAgreementsRequest};
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

        // Открываем транзакцию
        let tx = db.begin().await?;

        // Находим все Версии Соглашения, чтобы узнать новый номер Версии.
        let versions = agreement_versions::Entity::find()
            .filter(agreement_versions::Column::AgreementId.eq(create_version.agreement_id))
            .count(&tx)
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
        }
            .save(&tx)
            .await;

        // Удаляем все старые принятые версии
        agreement_acceptance_status::Entity::delete_many()
            .filter(agreement_acceptance_status::Column::AgreementId.eq(create_version.agreement_id))
            .filter(agreement_acceptance_status::Column::Version.lt(versions + 1))
            .exec(&tx)
            .await?;

        tx.commit().await?;

        version
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

    /// Устанавливает флаг о принятии пользователем определённой Версии Соглашения.
    ///
    /// # Arguments
    ///
    /// * `db`: ссылка на подключение к БД
    /// * `accept_request`: запрос на принятие Соглашения
    ///
    /// returns: Result<ActiveModel, DbErr>
    ///
    /// # Examples
    ///
    /// ```rust
    ///  let conn = &self.connection;
    ///
    ///  match AgreementsRepository::accept_agreement(conn, accept_request.clone())
    ///      .await {
    ///      Ok(_) => {
    ///          Ok(Response::new(AcceptAgreementReply {
    ///              agreement_id: accept_request.agreement_id,
    ///              version: accept_request.version,
    ///              user_id: accept_request.user_id,
    ///              provider_id: accept_request.provider_id,
    ///          }))
    ///      }
    ///      Err(e) => {
    ///          debug!("Accept agreement failed: {:?}", e);
    ///          Err(Status::new(
    ///              tonic::Code::Aborted,
    ///              "Could not accept Agreement with id ".to_owned() +
    ///                  &accept_request.agreement_id.to_string(),
    ///          ))
    ///      }
    ///  }
    /// ```
    pub async fn accept_agreement(db: &DbConn, accept_request: AcceptAgreementRequest)
                                  -> Result<agreement_acceptance_status::ActiveModel, DbErr> {
        debug!("AgreementsRepository::accept_agreement <- {:?}", accept_request);

        let tx = db.begin().await?;

        let version: Option<i32> = agreement_versions::Entity::find()
            .select_only()
            .column_as(sea_query::Expr::col(agreement_versions::Column::Version).max().to_owned(), "version")
            .filter(agreement_versions::Column::AgreementId.eq(accept_request.agreement_id))
            .into_tuple()
            .one(&tx)
            .await
            .unwrap();

        let inserted = agreement_acceptance_status::ActiveModel {
            user_id: Set(accept_request.user_id),
            provider_id: Set(accept_request.provider_id),
            agreement_id: Set(accept_request.agreement_id),
            version: Set(version.unwrap()),
            accepted: Set(true),
            accepted_at: Set(chrono::Utc::now().naive_utc()),
            ..Default::default()
        }
            .save(&tx)
            .await;

        tx.commit().await?;

        inserted
    }

    /// Возвращает список идентификаторов Соглашений непринятых пользователем.
    ///
    /// Метод выбирает все Соглашения, для которых отсутствуют записи о принятии в таблице
    /// `agreement_acceptance_status`.
    ///
    /// # Arguments
    ///
    /// * `db`: ссылка на подключение к БД
    /// * `request`: запрос на поиск с указанием идентификаторов пользователя и провайдера (опционально).
    ///
    /// returns: Result<Vec<i32>, DbErr>
    ///
    /// # Examples
    ///
    /// ```rust
    ///  let conn = &self.connection;
    ///
    ///  let agreements = AgreementsRepository::find_unaccepted(conn, unaccepted_request)
    ///      .await
    ///      .ok()
    ///      .unwrap();
    /// ```
    pub async fn find_unaccepted(db: &DbConn, request: GetUnacceptedAgreementsRequest) -> Result<Vec<i32>, DbErr> {
        debug!("AgreementsRepository::find_unaccepted <- {:?}", request);

        agreement::Entity::find()
            .select_only()
            .column(agreement::Column::Id)
            .filter(agreement::Column::Id.not_in_subquery(
                sea_query::Query::select()
                    .column(agreement_acceptance_status::Column::AgreementId)
                    .from(agreement_acceptance_status::Entity)
                    .and_where(agreement_acceptance_status::Column::UserId.eq(request.user_id))
                    .and_where(agreement_acceptance_status::Column::ProviderId.is_null().or(
                        agreement_acceptance_status::Column::ProviderId.eq(request.provider_id))
                    )
                    .to_owned()
            ))
            .into_tuple::<i32>()
            .all(db)
            .await
    }
}