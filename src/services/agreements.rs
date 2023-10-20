use sea_orm::DatabaseConnection;
use tonic::{Request, Response, Status};
use tonic::codegen::tokio_stream::wrappers::ReceiverStream;

use crate::agreements::{AgreementReply, AgreementVersionReply, CreateAgreementRequest, GetAgreementRequest, GetAgreementVersionsRequest};
use crate::agreements::agreement_service_server::AgreementService;

pub struct Agreementer {
    pub connection: DatabaseConnection,
}

#[tonic::async_trait]
impl AgreementService for Agreementer {
    async fn create_agreement(&self, request: Request<CreateAgreementRequest>) -> Result<Response<AgreementReply>, Status> {
        todo!()
    }

    async fn get_agreement(&self, request: Request<GetAgreementRequest>) -> Result<Response<AgreementReply>, Status> {
        todo!()
    }

    async fn get_agreement_version(&self, request: Request<GetAgreementVersionsRequest>) -> Result<Response<AgreementVersionReply>, Status> {
        todo!()
    }

    type GetAgreementVersionsStream = ReceiverStream<Result<AgreementVersionReply, Status>>;

    async fn get_agreement_versions(&self, request: Request<GetAgreementRequest>) -> Result<Response<Self::GetAgreementVersionsStream>, Status> {
        todo!()
    }
}
