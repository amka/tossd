use log::debug;
use tonic::{Request, Response, Status};
use tonic::codegen::tokio_stream::wrappers::ReceiverStream;

use crate::agreements::{Agreement, AgreementAcceptance, AgreementAcceptanceResponse};
use crate::agreements::agreements_service_server::AgreementsService;

#[derive(Default)]
pub struct Agreementer;

#[tonic::async_trait]
impl AgreementsService for Agreementer {
    async fn create_agreement(&self, request: Request<Agreement>) -> Result<Response<Agreement>, Status> {
        debug!("Called create_agreement");
        todo!()
    }

    async fn update_agreement(&self, request: Request<Agreement>) -> Result<Response<Agreement>, Status> {
        debug!("Called update_agreement");
        todo!()
    }

    async fn delete_agreement(&self, request: Request<Agreement>) -> Result<Response<Agreement>, Status> {
        debug!("Called delete_agreement");
        todo!()
    }

    async fn get_agreement(&self, request: Request<Agreement>) -> Result<Response<Agreement>, Status> {
        debug!("Called get_agreement");
        todo!()
    }

    type GetAgreementAcceptancesStream = ReceiverStream<Result<AgreementAcceptance, Status>>;

    async fn get_agreement_acceptances(&self, request: Request<Agreement>) -> Result<Response<Self::GetAgreementAcceptancesStream>, Status> {
        debug!("Called get_agreement_acceptances");
        todo!()
    }

    async fn set_agreement_acceptance(&self, request: Request<AgreementAcceptance>) -> Result<Response<AgreementAcceptanceResponse>, Status> {
        debug!("Called set_agreement_acceptance");
        todo!()
    }
}
