use log::debug;
use sea_orm::DatabaseConnection;
use tokio::sync::mpsc;
use tonic::{Request, Response, Status};
use tonic::codegen::tokio_stream::wrappers::ReceiverStream;

use crate::agreements::{AcceptAgreementReply, AcceptAgreementRequest, Agreement, AgreementReply, AgreementVersion, AgreementVersionReply, CreateAgreementRequest, CreateVersionRequest, GetAgreementRequest, GetUnacceptedAgreementsReply, GetUnacceptedAgreementsRequest, GetVersionRequest, GetVersionsRequest, ListAgreementsRequest};
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
        let create_version = CreateVersionRequest {
            agreement_id: inserted_agreement.id.clone().unwrap(),
            content: create_agreement.content,
            author_id: inserted_agreement.author_id.clone().unwrap(),
        };
        let _ = AgreementsRepository::add_version(conn, create_version)
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
    async fn create_agreement_version(&self, request: Request<CreateVersionRequest>)
                                      -> Result<Response<AgreementVersionReply>, Status> {
        debug!("CALLED: create_agreement");
        let conn = &self.connection;
        let create_version = request.into_inner();

        let inserted = AgreementsRepository::add_version(conn, create_version)
            .await
            .ok()
            .unwrap();

        Ok(Response::new(AgreementVersionReply {
            agreement_version: Some(AgreementVersion {
                id: inserted.id.unwrap(),
                agreement_id: inserted.agreement_id.unwrap(),
                version: inserted.version.unwrap(),
                content: inserted.content.unwrap(),
                created_at: inserted.created_at.unwrap().timestamp(),
                updated_at: inserted.updated_at.unwrap().timestamp(),
                author_id: inserted.author_id.unwrap(),
                deleted: inserted.deleted.unwrap(),
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


    async fn get_agreement_version(&self, request: Request<GetVersionRequest>)
                                   -> Result<Response<AgreementVersionReply>, Status> {
        debug!("CALLED: get_agreement");

        let conn = &self.connection;
        let id = request.into_inner().id;

        let version = AgreementsRepository::find_version_by_agreement_id(conn, id)
            .await
            .ok().unwrap();

        let unwrapped = version.unwrap();

        Ok(Response::new(AgreementVersionReply {
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

    async fn get_agreement_versions(&self, request: Request<GetVersionsRequest>)
                                    -> Result<Response<Self::GetAgreementVersionsStream>, Status> {
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


    type ListAgreementsStream = ReceiverStream<Result<AgreementReply, Status>>;


    async fn list_agreements(&self, request: Request<ListAgreementsRequest>) -> Result<Response<Self::ListAgreementsStream>, Status> {
        debug!("CALLED: get_agreement");

        let conn = &self.connection;
        let request = request.into_inner();

        let versions = AgreementsRepository::find_versions_by_agreement_id(conn, id)
            .await
            .ok().unwrap();

        let (tx, rx) = mpsc::channel(4);

        tokio::spawn(async move {
            for unwrapped in &versions[..] {
                tx.send(Ok(AgreementReply {
                    agreement: Agreement {
                        id: 0,
                        inner_title: unwrapped,
                        public_title: "".to_string(),
                        created_at: 0,
                        updated_at: 0,
                        author_id: 0,
                        deleted: false,
                    }
                })).await.unwrap();
            }

            debug!(" /// done sending");
        });

        Ok(Response::new(ReceiverStream::new(rx)))
    }


    async fn accept_agreement(&self, request: Request<AcceptAgreementRequest>) -> Result<Response<AcceptAgreementReply>, Status> {
        let accept_request = request.into_inner();

        let conn = &self.connection;

        match AgreementsRepository::accept_agreement(conn, accept_request.clone())
            .await {
            Ok(_) => {
                Ok(Response::new(AcceptAgreementReply {
                    agreement_id: accept_request.agreement_id,
                    version: accept_request.version,
                    user_id: accept_request.user_id,
                    provider_id: accept_request.provider_id,
                }))
            }
            Err(e) => {
                debug!("Accept agreement failed: {:?}", e);
                Err(Status::new(
                    tonic::Code::Aborted,
                    "Could not accept Agreement with id ".to_owned() +
                        &accept_request.agreement_id.to_string(),
                ))
            }
        }
    }

    async fn get_unaccepted_agreements(&self, request: Request<GetUnacceptedAgreementsRequest>) -> Result<Response<GetUnacceptedAgreementsReply>, Status> {
        let unaccepted_request = request.into_inner();
        let conn = &self.connection;

        let agreements = AgreementsRepository::find_unaccepted(conn, unaccepted_request)
            .await
            .ok()
            .unwrap();

        Ok(Response::new(GetUnacceptedAgreementsReply {
            agreements
        }))
    }
}
