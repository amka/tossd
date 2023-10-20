use log::debug;
use sea_orm::DatabaseConnection;
use tokio::sync::mpsc;
use tonic::{Request, Response, Status};
use tonic::codegen::tokio_stream::wrappers::ReceiverStream;

use crate::agreements::{Agreement, AgreementReply, AgreementVersion, AgreementVersionReply, CreateAgreementRequest, GetAgreementRequest, GetAgreementVersionsRequest};
use crate::agreements::agreement_service_server::AgreementService;
use crate::repository::agreements::AgreementsRepository;

pub struct Agreementer {
    pub connection: DatabaseConnection,
}

#[tonic::async_trait]
impl AgreementService for Agreementer {
    /// Создаёт новое Соглашение и Версию соглашения.
    ///
    /// Если указан список `providers` - Соглашение привязывается к провайдерам
    /// (на случай “вирутальных” провайдеров, таких как МотивТВ и др.).
    ///
    /// Если флаг `overrider_version` выставлен в `true` - обновляется последняя Версия
    /// без создания новой.
    async fn create_agreement(&self, request: Request<CreateAgreementRequest>) -> Result<Response<AgreementReply>, Status> {
        debug!("CALLED: create_agreement");
        let conn = &self.connection;
        let create_agreement = request.into_inner();

        // Сначала создаём Соглашение
        let inserted_agreement = AgreementsRepository::add(
            conn,
            create_agreement.clone())
            .await
            .ok()
            .unwrap();

        // Затем создаём Версию.
        // Если при создании Версии получим ошибку - пользователь должен создать новую версию
        // минуя создание нового Соглашения.
        let _ = AgreementsRepository::add_version(
            conn,
            inserted_agreement.clone().id.unwrap(),
            create_agreement)
            .await
            .ok();

        Ok(Response::new(AgreementReply {
            agreement: Some(Agreement {
                id: inserted_agreement.id.unwrap(),
                inner_title: inserted_agreement.inner_title.unwrap(),
                public_title: inserted_agreement.public_title.unwrap(),
                created_at: inserted_agreement.created_at.unwrap().timestamp(),
                updated_at: inserted_agreement.updated_at.unwrap().timestamp(),
                author_id: inserted_agreement.author_id.unwrap(),
                deleted: inserted_agreement.deleted.unwrap(),
            })
        }))
    }

    async fn get_agreement(&self, request: Request<GetAgreementRequest>) -> Result<Response<AgreementReply>, Status> {
        debug!("CALLED: get_agreement");

        let conn = &self.connection;
        let id = request.into_inner().id;

        if let Some(model) = AgreementsRepository::find_by_id(conn, id)
            .await
            .ok().unwrap() {
            let agreement = AgreementReply {
                agreement: Some(Agreement {
                    id,
                    inner_title: model.inner_title,
                    public_title: model.public_title,
                    created_at: model.created_at.timestamp(),
                    updated_at: model.updated_at.timestamp(),
                    author_id: model.author_id,
                    deleted: model.deleted,
                }),
            };

            Ok(Response::new(agreement))
        } else {
            Err(Status::new(
                tonic::Code::Aborted,
                "Could not find Agreement with id ".to_owned() + &id.to_string(),
            ))
        }
    }

    async fn get_agreement_version(&self, request: Request<GetAgreementVersionsRequest>) -> Result<Response<AgreementVersionReply>, Status> {
        debug!("CALLED: get_agreement");

        let conn = &self.connection;
        let id = request.into_inner().id;

        let version = AgreementsRepository::find_version_by_agreement_id(conn, id)
            .await
            .ok().unwrap();

        let unwrapped = version.unwrap();

        Ok(Response::new(AgreementVersionReply {
            agreement: None,
            agreement_version: Some(AgreementVersion {
                id: unwrapped.id as i64,
                agreement_id: unwrapped.agreement_id,
                version: unwrapped.version,
                content: unwrapped.content,
                created_at: unwrapped.created_at.timestamp(),
                updated_at: unwrapped.updated_at.timestamp(),
                author_id: unwrapped.author_id,
                deleted: unwrapped.deleted,
            }),
        }))
    }

    type GetAgreementVersionsStream = ReceiverStream<Result<AgreementVersionReply, Status>>;

    async fn get_agreement_versions(&self, request: Request<GetAgreementRequest>) -> Result<Response<Self::GetAgreementVersionsStream>, Status> {
        debug!("CALLED: get_agreement");

        let conn = &self.connection;
        let id = request.into_inner().id;

        let versions = AgreementsRepository::find_versions_by_agreement_id(conn, id)
            .await
            .ok().unwrap();

        let (tx, rx) = mpsc::channel(4);

        tokio::spawn(async move {
            for unwrapped in &versions[..] {
                tx.send(Ok(AgreementVersionReply {
                    agreement: None,
                    agreement_version: Some(AgreementVersion {
                        id: unwrapped.id as i64,
                        agreement_id: unwrapped.agreement_id,
                        version: unwrapped.version,
                        content: unwrapped.content.clone(),
                        created_at: unwrapped.created_at.timestamp(),
                        updated_at: unwrapped.updated_at.timestamp(),
                        author_id: unwrapped.author_id,
                        deleted: unwrapped.deleted,
                    }),
                })).await.unwrap();
            }

            debug!(" /// done sending");
        });

        Ok(Response::new(ReceiverStream::new(rx)))
    }
}
