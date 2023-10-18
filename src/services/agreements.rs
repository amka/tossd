use tonic::{Request, Response, Status};
use tonic::codegen::tokio_stream::wrappers::ReceiverStream;
use crate::agreements::{Agreement, AgreementAcceptance, AgreementAcceptanceResponse};
use crate::agreements::agreements_service_server::AgreementsService;

#[derive(Default)]
pub struct Agreementer;

#[tonic::async_trait]
impl AgreementsService for Agreementer {
    async fn create_agreement(&self, request: Request<Agreement>) -> Result<Response<Agreement>, Status> {
        todo!()
    }

    async fn update_agreement(&self, request: Request<Agreement>) -> Result<Response<Agreement>, Status> {
        todo!()
    }

    async fn delete_agreement(&self, request: Request<Agreement>) -> Result<Response<Agreement>, Status> {
        todo!()
    }

    async fn get_agreement(&self, request: Request<Agreement>) -> Result<Response<Agreement>, Status> {
        todo!()
    }

    type GetAgreementAcceptancesStream = ReceiverStream<Result<AgreementAcceptance, Status>>;

    async fn get_agreement_acceptances(&self, request: Request<Agreement>) -> Result<Response<Self::GetAgreementAcceptancesStream>, Status> {
        todo!()
    }

    async fn set_agreement_acceptance(&self, request: Request<AgreementAcceptance>) -> Result<Response<AgreementAcceptanceResponse>, Status> {
        todo!()
    }
}
