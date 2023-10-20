use log::debug;
use sea_orm::DatabaseConnection;
use tonic::{Request, Response, Status};
use tonic::codegen::tokio_stream::wrappers::ReceiverStream;

use crate::agreements::{Agreement, AgreementReply, AgreementVersionReply, CreateAgreementRequest, GetAgreementRequest, GetAgreementVersionsRequest};
use crate::agreements::agreement_service_server::AgreementService;
use crate::repository::agreements::AgreementsRepository;

pub struct Agreementer {
    pub connection: DatabaseConnection,
}

#[tonic::async_trait]
impl AgreementService for Agreementer {
    async fn create_agreement(&self, request: Request<CreateAgreementRequest>) -> Result<Response<AgreementReply>, Status> {
        todo!()
    }

    async fn get_agreement(&self, request: Request<GetAgreementRequest>) -> Result<Response<AgreementReply>, Status> {
        debug!("Called get_agreement");

        let conn = &self.connection;
        let id = request.into_inner().id;

        if let Some(model) = AgreementsRepository::find_by_id(conn, id)
            .await
            .ok().unwrap() {
            let agreement = AgreementReply {
                agreement: Some(Agreement {
                    id,
                    inner_title: model.inner_title,
                    created_at: model.created_at.timestamp(),
                    updated_at: model.updated_at.timestamp(),
                    provider_id: model.provider_id,
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
        todo!()
    }

    type GetAgreementVersionsStream = ReceiverStream<Result<AgreementVersionReply, Status>>;

    async fn get_agreement_versions(&self, request: Request<GetAgreementRequest>) -> Result<Response<Self::GetAgreementVersionsStream>, Status> {
        todo!()
    }
}
